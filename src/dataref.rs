//!
//! Provides access to X-Plane datarefs
//!

use std::marker::PhantomData;
use std::ffi::NulError;
use std::ffi::CString;
use std::ptr;

use xplm_sys::data_access::*;

use libc;

/// Trait for types that have associated type IDs in X-Plane
trait DataType {
    /// Returns the XPLMDataTypeID for this type
    fn data_type() -> XPLMDataTypeID;
}

impl DataType for i32 {
    fn data_type() -> XPLMDataTypeID {
        xplmType_Int as XPLMDataTypeID
    }
}

impl DataType for f32 {
    fn data_type() -> XPLMDataTypeID {
        xplmType_Float as XPLMDataTypeID
    }
}

impl DataType for f64 {
    fn data_type() -> XPLMDataTypeID {
        xplmType_Double as XPLMDataTypeID
    }
}

impl DataType for Vec<f32> {
    fn data_type() -> XPLMDataTypeID {
        xplmType_FloatArray as XPLMDataTypeID
    }
}

impl DataType for Vec<i32> {
    fn data_type() -> XPLMDataTypeID {
        xplmType_IntArray as XPLMDataTypeID
    }
}

impl DataType for Vec<u8> {
    fn data_type() -> XPLMDataTypeID {
        xplmType_Data as XPLMDataTypeID
    }
}
impl DataType for String {
    fn data_type() -> XPLMDataTypeID {
        xplmType_Data as XPLMDataTypeID
    }
}
/// Trait for a read/write or read-only marker
pub trait DataAccess {
    /// Returns true if the dataref should be writeable
    fn writeable() -> bool;
}
/// Marks a dataref that can be read and written
#[derive(Debug,Clone)]
pub struct ReadWrite;
impl DataAccess for ReadWrite {
    fn writeable() -> bool { true }
}

/// Marks a dataref that can only be read
#[derive(Debug,Clone)]
pub struct ReadOnly;
impl DataAccess for ReadOnly {
    fn writeable() -> bool { false }
}

/// Possible errors encountered when finding a dataref
#[derive(Debug,Clone)]
pub enum SearchError {
    /// Indicates that the provided name contains one or more null bytes
    /// Includes the NulError to provide more details
    InvalidName(NulError),
    /// Indicates that no dataref with the specified name was found
    NotFound,
    /// Indicates that the requested data type and the dataref's type
    /// do not match
    WrongDataType,
    /// Indicates that the wrong DataAccess was requested, which usually
    /// means that a ReadWrite DataRef object was used with a read-only dataref
    WrongDataAccess,
}

/// Provides access to a dataref
#[derive(Debug,Clone)]
pub struct DataRef<D, A> {
    /// Dataref handle
    dataref: XPLMDataRef,
    /// Phantom data for data type
    type_phantom: PhantomData<D>,
    /// Phantom data for access
    access_phantom: PhantomData<A>,
}

impl<D, A> DataRef<D, A> where D: DataType, A: DataAccess {
    /// Finds a dataref with the provided name.
    /// Returns a DataRef object or an error
    pub fn find(name: &str) -> Result<DataRef<D, A>, SearchError> {
        match CString::new(name) {
            Ok(name_c) => unsafe {
                let dataref = XPLMFindDataRef(name_c.as_ptr());
                // Check found
                if dataref.is_null() {
                    return Err(SearchError::NotFound);
                }
                // Check writeability
                let actually_writeable = XPLMCanWriteDataRef(dataref) == 1;
                if A::writeable() && !actually_writeable {
                    return Err(SearchError::WrongDataAccess);
                }
                // Check type
                let actual_types = XPLMGetDataRefTypes(dataref);
                if (D::data_type() & actual_types) == 0 {
                    return Err(SearchError::WrongDataType);
                }
                // OK
                Ok(DataRef {
                    dataref: dataref,
                    type_phantom: PhantomData,
                    access_phantom: PhantomData,
                })
            },
            Err(e) => Err(SearchError::InvalidName(e)),
        }
    }
}

// Integer read
impl<A> DataRef<i32, A> where A: DataAccess {
    pub fn get(&self) -> i32 {
        unsafe { XPLMGetDatai(self.dataref) }
    }
}
// Integer write
impl DataRef<i32, ReadWrite> {
    pub fn set(&mut self, value: i32) {
        unsafe { XPLMSetDatai(self.dataref, value) }
    }
}
// Float read
impl<A> DataRef<f32, A> where A: DataAccess {
    pub fn get(&self) -> f32 {
        unsafe { XPLMGetDataf(self.dataref) }
    }
}
// Float write
impl DataRef<f32, ReadWrite> {
    pub fn set(&mut self, value: f32) {
        unsafe { XPLMSetDataf(self.dataref, value) }
    }
}
// Double read
impl<A> DataRef<f64, A> where A: DataAccess {
    pub fn get(&self) -> f64 {
        unsafe { XPLMGetDatad(self.dataref) }
    }
}
// Double write
impl DataRef<f64, ReadWrite> {
    pub fn set(&mut self, value: f64) {
        unsafe { XPLMSetDatad(self.dataref, value) }
    }
}
// Integer array read
impl<A> DataRef<Vec<i32>, A> where A: DataAccess {
    ///
    /// Replaces values in the provided slice with the values stored in this dataref
    ///
    /// If the dataref has more values than the length of the slice, values beyond the last element
    /// of the slice will not be copied.
    ///
    /// If the dataref has fewer values than the length of the slice, the content of entries in
    /// the slice after the last entry in the dataref is unspecified.
    ///
    /// If the slice has more than i32::max_value() elements, no more than i32::max_value()
    /// elements will be read.
    ///
    pub fn get(&self, data: &mut [i32]) {
        unsafe { XPLMGetDatavi(self.dataref, data.as_mut_ptr(), 0, array_length(data.len())) };
    }
    /// Returns the number of values in this dataref
    pub fn len(&self) -> usize {
        let size_int = unsafe { XPLMGetDatavi(self.dataref, ptr::null_mut(), 0, 0) };
        size_int as usize
    }
}
// Integer array write
impl DataRef<Vec<i32>, ReadWrite> {
    ///
    /// Writes values from the provided slice into this dataref
    ///
    /// If the dataref has more values than the length of the slice, values beyond the last element
    /// of the slice will not be modified.
    ///
    /// If the slice has more than i32::max_value() elements, no more than i32::max_value()
    /// elements will be written.
    ///
    pub fn set(&mut self, data: &[i32]) {
        unsafe { XPLMSetDatavi(self.dataref, data.as_ptr() as *mut i32, 0, array_length(data.len()) ) };
    }
}
// Float array read
impl<A> DataRef<Vec<f32>, A> where A: DataAccess {
    ///
    /// Replaces values in the provided slice with the values stored in this dataref
    ///
    /// If the dataref has more values than the length of the slice, values beyond the last element
    /// of the slice will not be copied.
    ///
    /// If the dataref has fewer values than the length of the slice, the content of entries in
    /// the slice after the last entry in the dataref is unspecified.
    ///
    /// If the slice has more than i32::max_value() elements, no more than i32::max_value()
    /// elements will be read.
    ///
    pub fn get(&self, data: &mut [f32]) {
        unsafe { XPLMGetDatavf(self.dataref, data.as_mut_ptr(), 0, array_length(data.len())) };
    }
    /// Returns the number of values in this dataref
    pub fn len(&self) -> usize {
        let size_int = unsafe { XPLMGetDatavf(self.dataref, ptr::null_mut(), 0, 0) };
        size_int as usize
    }
}
// Float array write
impl DataRef<Vec<f32>, ReadWrite> {
    ///
    /// Writes values from the provided slice into this dataref
    ///
    /// If the dataref has more values than the length of the slice, values beyond the last element
    /// of the slice will not be modified.
    ///
    /// If the slice has more than i32::max_value() elements, no more than i32::max_value()
    /// elements will be written.
    ///
    pub fn set(&mut self, data: &[f32]) {
        unsafe { XPLMSetDatavi(self.dataref, data.as_ptr() as *mut i32, 0, array_length(data.len()) ) };
    }
}

// Byte array read
impl<A> DataRef<Vec<u8>, A> where A: DataAccess {
    ///
    /// Replaces values in the provided slice with the values stored in this dataref
    ///
    /// If the dataref has more values than the length of the slice, values beyond the last element
    /// of the slice will not be copied.
    ///
    /// If the dataref has fewer values than the length of the slice, the content of entries in
    /// the slice after the last entry in the dataref is unspecified.
    ///
    /// If the slice has more than i32::max_value() elements, no more than i32::max_value()
    /// elements will be read.
    ///
    pub fn get(&self, data: &mut [u8]) {
        unsafe { XPLMGetDatab(self.dataref, data.as_mut_ptr() as *mut libc::c_void, 0, array_length(data.len())) };
    }
    /// Returns the number of values in this dataref
    pub fn len(&self) -> usize {
        let size_int = unsafe { XPLMGetDatab(self.dataref, ptr::null_mut(), 0, 0) };
        size_int as usize
    }
}
// Byte array write
impl DataRef<Vec<u8>, ReadWrite> {
    ///
    /// Writes values from the provided slice into this dataref
    ///
    /// If the dataref has more values than the length of the slice, values beyond the last element
    /// of the slice will not be modified.
    ///
    /// If the slice has more than i32::max_value() elements, no more than i32::max_value()
    /// elements will be written.
    ///
    pub fn set(&mut self, data: &[u8]) {
        unsafe { XPLMSetDatab(self.dataref, data.as_ptr() as *mut libc::c_void, 0, array_length(data.len()) ) };
    }
}

// String read
impl<A> DataRef<String, A> where A: DataAccess {
    ///
    /// Reads and returns the value of this dataref
    ///
    pub fn get(&self) -> String {
        // Create a byte array of the right length
        let length = self.len();
        let mut bytes: Vec<u8> = Vec::with_capacity(length);
        for _ in 0..length {
            bytes.push('\0' as u8);
        }
        // Copy data in
        unsafe { XPLMGetDatab(self.dataref, bytes.as_mut_ptr() as *mut libc::c_void, 0, array_length(length)) };
        // Convert into a string
        bytes_to_string(&bytes)
    }
    /// Returns the maximum length of this string dataref
    fn len(&self) -> usize {
        let size_int = unsafe { XPLMGetDatab(self.dataref, ptr::null_mut(), 0, 0) };
        size_int as usize
    }
}
// String write
impl DataRef<String, ReadWrite> {
    ///
    /// Sets the value of this dataref
    ///
    /// If the provided value contains more than i32::max_value() bytes, up to i32::max_value()
    /// bytes will be written.
    ///
    pub fn set(&mut self, value: &str) -> Result<(), NulError> {
        let value_c = try!(CString::new(value));
        unsafe { XPLMSetDatab(self.dataref, value_c.as_ptr() as *mut libc::c_void, 0,
            array_length(value_c.as_bytes_with_nul().len())) };
        Ok(())
    }
}

/// Fits a length into an i32.
/// If the provided value is greater than i32::max_value, returns i32::max_value().
/// Otherwise, returns the value as an i32.
fn array_length(length: usize) -> i32 {
    if length > (i32::max_value() as usize) {
        i32::max_value()
    }
    else {
        length as i32
    }
}


/// Converts a byte array into a String.
///
/// If the provided byte array contains any null bytes, the returned String excludes the first
/// null byte and any bytes that follow it.
fn bytes_to_string(bytes: &[u8]) -> String {
    let mut end_index = bytes.len();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == '\0' as u8 {
            end_index = i;
            break;
        }
    }
    String::from_utf8_lossy(&bytes[0..end_index]).into_owned()
}
