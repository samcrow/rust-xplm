
use xplm_sys::*;
use std::string::FromUtf8Error;
use std::ffi::{CString, NulError};
use ffi::StringBuffer;

/// Datarefs created by X-Plane or other plugins
pub mod borrowed;
/// Datarefs created by this plugin
pub mod owned;

/// Marks a dataref as readable
pub enum ReadOnly {

}

/// Marks a dataref as writeable
pub enum ReadWrite {

}

/// Marker for data access types
pub trait Access {
    /// Returns true if this access allows the dataref to be written
    fn writeable() -> bool;
}

impl Access for ReadOnly {
    fn writeable() -> bool {
        false
    }
}

impl Access for ReadWrite {
    fn writeable() -> bool {
        true
    }
}

/// Trait for data accessors that can be read
pub trait DataRead<T> {
    /// Reads a value
    fn get(&self) -> T;
}

/// Trait for writable data accessors
pub trait DataReadWrite<T>: DataRead<T> {
    /// Writes a value
    fn set(&mut self, value: T);
}

/// Trait for readable array data accessors
pub trait ArrayRead<T: ArrayType + ?Sized> {
    /// Reads values
    ///
    /// Values are stored in the provided slice. If the dataref is larger than the provided slice,
    /// values beyond the bounds of the slice are ignored.
    ///
    /// If the dataref is smaller than the provided slice, the extra values in the slice will not
    /// be modified.
    ///
    /// The maximum number of values in an array dataref is i32::MAX.
    ///
    /// This function returns the number of values that were read.
    fn get(&self, dest: &mut [T::Element]) -> usize;

    /// Returns the length of the data array
    fn len(&self) -> usize;

    /// Returns all values in this accessor as a Vec
    fn as_vec(&self) -> Vec<T::Element>
    where
        T::Element: Default + Clone,
    {
        let mut values = vec![T::Element::default(); self.len()];
        self.get(&mut values);
        values
    }
}

/// Trait for array accessors that can be read and written
pub trait ArrayReadWrite<T: ArrayType + ?Sized>: ArrayRead<T> {
    /// Writes values
    ///
    /// Values are taken from the provided slice. If the dataref is larger than the provided slice,
    /// values beyond the bounds of the slice are not changed.
    ///
    /// If the dataref is smaller than the provided slice, the values beyond the dataref bounds
    /// will be ignored.
    fn set(&mut self, values: &[T::Element]);
}

/// Trait for data accessors that can be read as strings
pub trait StringRead {
    /// Reads the value of this dataref and appends it to the provided string
    ///
    /// Returns an error if the dataref is not valid UTF-8.
    ///
    /// If the provided string is not empty, the value of the dataref will be appended to it.
    fn get_to_string(&self, out: &mut String) -> Result<(), FromUtf8Error>;

    /// Reads the value of this dataref as a string and returns it
    fn get_as_string(&self) -> Result<String, FromUtf8Error>;
}

/// Trait for data accessors that can be written as strings
pub trait StringReadWrite: StringRead {
    /// Sets the value of this dataref from a string
    ///
    /// Returns an error if the string contains a null byte
    fn set_as_string(&mut self, value: &str) -> Result<(), NulError>;
}

impl<T> StringRead for T
where
    T: ArrayRead<[u8]>,
{
    fn get_to_string(&self, out: &mut String) -> Result<(), FromUtf8Error> {
        let mut buffer = StringBuffer::new(self.len());
        self.get(buffer.as_bytes_mut());
        let value_string = buffer.into_string()?;
        out.push_str(&value_string);
        Ok(())
    }
    fn get_as_string(&self) -> Result<String, FromUtf8Error> {
        let mut buffer = StringBuffer::new(self.len());
        self.get(buffer.as_bytes_mut());
        buffer.into_string()
    }
}

impl<T> StringReadWrite for T
where
    T: ArrayReadWrite<[u8]>,
{
    fn set_as_string(&mut self, value: &str) -> Result<(), NulError> {
        let name_c = CString::new(value)?;
        self.set(name_c.as_bytes_with_nul());
        Ok(())
    }
}

/// Marker for types that can be used with datarefs
pub trait DataType {
    /// The type that should be used to store data of this type
    /// For basic types, this is usually Self. For [T] types, this is Vec<T>.
    #[doc(hidden)]
    type Storage: Sized;
    /// Returns the X-Plane data type corresponding with this type
    #[doc(hidden)]
    fn sim_type() -> XPLMDataTypeID;
    /// Creates an instance of a storage type from an instance of self
    #[doc(hidden)]
    fn to_storage(&self) -> Self::Storage;
}

/// Marker for types that are arrays
pub trait ArrayType: DataType {
    /// The type of the array element
    type Element;
}

macro_rules! impl_type {
    ($native_type:ty as $sim_type:ident) => {
        impl DataType for $native_type {
            type Storage = Self;
            fn sim_type() -> XPLMDataTypeID {
                $sim_type as XPLMDataTypeID
            }
            fn to_storage(&self) -> Self::Storage {
                self.clone()
            }
        }
    };
    ([$native_type:ty]: array as $sim_type:ident) => {
        impl DataType for [$native_type] {
            type Storage = Vec<$native_type>;
            fn sim_type() -> XPLMDataTypeID {
                $sim_type as XPLMDataTypeID
            }
            fn to_storage(&self) -> Self::Storage {
                self.to_vec()
            }
        }
        impl ArrayType for [$native_type] {
            type Element = $native_type;
        }
    }
}

impl_type!(bool as xplmType_Int);
impl_type!(u8 as xplmType_Int);
impl_type!(i8 as xplmType_Int);
impl_type!(u16 as xplmType_Int);
impl_type!(i16 as xplmType_Int);
impl_type!(u32 as xplmType_Int);
impl_type!(i32 as xplmType_Int);
impl_type!(f32 as xplmType_Float);
impl_type!(f64 as xplmType_Double);
impl_type!([i32]: array as xplmType_IntArray);
impl_type!([u32]: array as xplmType_IntArray);
impl_type!([f32]: array as xplmType_FloatArray);
impl_type!([u8]: array as xplmType_Data);
impl_type!([i8]: array as xplmType_Data);
