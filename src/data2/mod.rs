// Copyright (c) 2015 rust-xplm developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.


/// Provides access to existing datarefs
pub mod borrowed;
/// Allows creation of datarefs
pub mod owned;
/// Allows creation of and access to shared data
pub mod shared;

extern crate libc;

use xplm_sys::data_access::*;

use ffi::StringBuffer;

use std::ffi::{CString, NulError};
use std::ptr;
///
/// Trait for objects that can be accessed through an XPLMDataRef handle
///
/// D is the data type. A is the data access level
///
pub trait DataRef<D, A> {
    ///
    /// Returns the dataref that this object contains
    ///
    fn dataref(&self) -> XPLMDataRef;
}

///
/// A trait for objects that can be 'dereferenced' to get a value
///
pub trait Readable<T> {
    /// Returns the value stored in this object
    fn get(&self) -> T;
}

///
/// A trait for objects in which a value can be stored
///
pub trait Writeable<T> {
    /// Sets the value in this object
    fn set(&mut self, value: T);
}
///
/// A trait for objects in which an array of values can be stored
///
pub trait ArrayReadable<T> : Readable<Vec<T>> {
    /// Returns the number of values in this array
    fn len(&self) -> usize;
}

pub trait ArrayWriteable<T> : ArrayReadable<T> + Writeable<Vec<T>> {
    ///
    /// Sets the values in this array with the values from a slice.
    ///
    /// If the slice has more than `i32::max_value()`` elements, only
    /// `i32::max_value()` elements will be set.
    ///
    /// If the slice has more elements than this array, the extra values will
    /// be ignored.
    ///
    /// If the slice has fewer elements than this array, the extra elements
    /// in this array will not be changed.
    ///
    fn set_from_slice(&mut self, value: &[T]);
}

///
/// A trait for objects that can be read as Strings
///
pub trait StringReadable {
    /// Reads this value as a string
    fn get_string(&self) -> String;
    /// Returns the length of this string value in bytes
    fn len(&self) -> usize;
}
///
/// A trait for objects that can be written as Strings
///
pub trait StringWriteable : StringReadable {
    /// Sets the values in this array from a string
    /// If the string contains one or more null bytes, an error
    /// is returned.
    fn set_string(&mut self, value: &str) -> Result<(), NulError>;
}

// Integer read
impl<A> Readable<i32> for DataRef<i32, A> {
    /// Returns the value of this dataref
    fn get(&self) -> i32 {
        unsafe { XPLMGetDatai(self.dataref()) }
    }
}
// Integer write
impl Writeable<i32> for DataRef<i32, ReadWrite> {
    /// Sets the value of this dataref
    fn set(&mut self, value: i32) {
        unsafe { XPLMSetDatai(self.dataref(), value) }
    }
}
// Float read
impl<A> Readable<f32> for DataRef<f32, A> {
    /// Returns the value of this dataref
    fn get(&self) -> f32 {
        unsafe { XPLMGetDataf(self.dataref()) }
    }
}
// Float write
impl Writeable<f32> for DataRef<f32, ReadWrite> {
    /// Sets the value of this dataref
    fn set(&mut self, value: f32) {
        unsafe { XPLMSetDataf(self.dataref(), value) }
    }
}
// Double read
impl<A> Readable<f64> for DataRef<f64, A> {
    /// Returns the value of this dataref
    fn get(&self) -> f64 {
        unsafe { XPLMGetDatad(self.dataref()) }
    }
}
// Double write
impl DataRef<f64, ReadWrite> {
    /// Sets the value of this dataref
    pub fn set(&mut self, value: f64) {
        unsafe { XPLMSetDatad(self.dataref(), value) }
    }
}

// Integer array read
impl<A> Readable<Vec<i32>> for DataRef<Vec<i32>, A> {
    fn get(&self) -> Vec<i32> {
        let length = self.len();
        let mut values = Vec::with_capacity(length);
        unsafe {
            values.set_len(length);
            XPLMGetDatavi(self.dataref(), values.as_mut_ptr(), 0, array_length(length));
        }
        values
    }
}
impl<A> ArrayReadable<i32> for DataRef<Vec<i32>, A> {
    fn len(&self) -> usize {
        let size_int = unsafe { XPLMGetDatavi(self.dataref(), ptr::null_mut(), 0, 0) };
        size_int as usize
    }
}
// Integer array write
impl Writeable<Vec<i32>> for DataRef<Vec<i32>, ReadWrite> {
    fn set(&mut self, value: Vec<i32>) {
        self.set_from_slice(&value)
    }
}
impl ArrayWriteable<i32> for DataRef<Vec<i32>, ReadWrite> {
    fn set_from_slice(&mut self, value: &[i32]) {
        unsafe {
            XPLMSetDatavi(self.dataref(), value.as_ptr() as *mut i32,
                0, array_length(value.len()));
        }
    }
}

// Float array read
impl<A> Readable<Vec<f32>> for DataRef<Vec<f32>, A> {
    fn get(&self) -> Vec<f32> {
        let length = self.len();
        let mut values = Vec::with_capacity(length);
        unsafe {
            values.set_len(length);
            XPLMGetDatavf(self.dataref(), values.as_mut_ptr(), 0, array_length(length));
        }
        values
    }
}
impl<A> ArrayReadable<f32> for DataRef<Vec<f32>, A> {
    fn len(&self) -> usize {
        let size_int = unsafe { XPLMGetDatavf(self.dataref(), ptr::null_mut(), 0, 0) };
        size_int as usize
    }
}
// Float array write
impl Writeable<Vec<f32>> for DataRef<Vec<f32>, ReadWrite> {
    fn set(&mut self, value: Vec<f32>) {
        self.set_from_slice(&value)
    }
}
impl ArrayWriteable<f32> for DataRef<Vec<f32>, ReadWrite> {
    fn set_from_slice(&mut self, value: &[f32]) {
        unsafe {
            XPLMSetDatavf(self.dataref(), value.as_ptr() as *mut f32,
                0, array_length(value.len()));
        }
    }
}

// Byte array read
impl<A> Readable<Vec<u8>> for DataRef<Vec<u8>, A> {
    fn get(&self) -> Vec<u8> {
        let length = self.len();
        let mut values = Vec::with_capacity(length);
        unsafe {
            values.set_len(length);
            XPLMGetDatab(self.dataref(), values.as_mut_ptr() as *mut libc::c_void,
                0, array_length(length));
        }
        values
    }
}
impl<A> ArrayReadable<u8> for DataRef<Vec<u8>, A> {
    fn len(&self) -> usize {
        let size_int = unsafe { XPLMGetDatab(self.dataref(), ptr::null_mut(), 0, 0) };
        size_int as usize
    }
}
// Byte array write
impl Writeable<Vec<u8>> for DataRef<Vec<u8>, ReadWrite> {
    fn set(&mut self, value: Vec<u8>) {
        self.set_from_slice(&value)
    }
}
impl ArrayWriteable<u8> for DataRef<Vec<u8>, ReadWrite> {
    fn set_from_slice(&mut self, value: &[u8]) {
        unsafe {
            XPLMSetDatab(self.dataref(), value.as_ptr() as *mut libc::c_void,
                0, array_length(value.len()));
        }
    }
}
// String read
impl<A> StringReadable for DataRef<String, A> {
    fn get_string(&self) -> String {
        // Create a byte array of the right length
        let length = self.len();
        let mut buffer = StringBuffer::new(length);
        // Copy data in
        unsafe { XPLMGetDatab(self.dataref(), buffer.as_mut_ptr() as *mut libc::c_void,
            0, array_length(length)) };
        // Convert into a string
        buffer.as_string()
    }
    fn len(&self) -> usize {
        let size_int = unsafe { XPLMGetDatab(self.dataref(), ptr::null_mut(), 0, 0) };
        size_int as usize
    }
}
// String write
impl StringWriteable for DataRef<String, ReadWrite> {
    fn set_string(&mut self, value: &str) -> Result<(), NulError> {
        let value_c = try!(CString::new(value));
        unsafe { XPLMSetDatab(self.dataref(), value_c.as_ptr() as *mut libc::c_void, 0,
            array_length(value_c.as_bytes_with_nul().len())) };
        Ok(())
    }
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

/// Trait for types that have associated type IDs in X-Plane
pub trait DataType : Clone {
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
