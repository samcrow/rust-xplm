// Copyright (c) 2015 rust-xplm developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

//!
//! Provides access to X-Plane datarefs
//!

use std::marker::PhantomData;
use std::ffi::NulError;
use std::ffi::CString;
use std::ptr;

use ffi::StringBuffer;
use xplm_sys::data_access::*;
use data::*;
use super::array_length;

use libc;


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

/// Provides safe access to a dataref
///
/// The type parameter `D` specifies the type of the dataref. The following types are supported:
///
/// * `i32`: X-Plane type `int`
/// * `f32`: X-Plane type `float`
/// * `f64`: X-Plane type `double`
/// * `Vec<i32>`: X-Plane type `int` array
/// * `Vec<f32>`: X-Plane type `float` array
/// * `Vec<u8>`: X-Plane type `byte` array
/// * `String`: X-Plane type `byte` array
///
/// The type parameter `A` specifies the writeability of the dataref. This should be either
/// `ReadWrite` for a writeable dataref or `ReadOnly` for a read-only dataref.
///
/// When a `DataRef` is created, its type and writeability are checked against the type and
/// writeability specified by X-Plane. A `DataRef` can only be created if X-Plane knows about
/// a dataref with the same name and compatible type and writeabiltiy.
///
/// # Examples
///
/// ## Read-only f32 dataref
///
/// ```no_run
/// let time_ref: DataRef<f32, ReadOnly> = DataRef::find("sim/time/total_running_time_sec").unwrap();
/// let time = time_ref.get();
/// ```
///
/// ## Read-only String dataref
///
/// ```no_run
/// let dataref: DataRef<String, ReadOnly> = DataRef::find("sim/version/sim_build_string").unwrap();
/// let sim_build_time = dataref.get();
/// ```
///
/// ## Writeable i32 dataref
///
/// ```no_run
/// let dataref: DataRef<i32, ReadWrite> = DataRef::find("sim/cockpit2/autopilot/flight_director_mode").unwrap();
/// let mode = dataref.get();
/// dataref.set(3);
/// ```
///
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
    /// Returns the value of this dataref
    pub fn get(&self) -> i32 {
        unsafe { XPLMGetDatai(self.dataref) }
    }
}
// Integer write
impl DataRef<i32, ReadWrite> {
    /// Sets the value of this dataref
    pub fn set(&mut self, value: i32) {
        unsafe { XPLMSetDatai(self.dataref, value) }
    }
}
// Float read
impl<A> DataRef<f32, A> where A: DataAccess {
    /// Returns the value of this dataref
    pub fn get(&self) -> f32 {
        unsafe { XPLMGetDataf(self.dataref) }
    }
}
// Float write
impl DataRef<f32, ReadWrite> {
    /// Sets the value of this dataref
    pub fn set(&mut self, value: f32) {
        unsafe { XPLMSetDataf(self.dataref, value) }
    }
}
// Double read
impl<A> DataRef<f64, A> where A: DataAccess {
    /// Returns the value of this dataref
    pub fn get(&self) -> f64 {
        unsafe { XPLMGetDatad(self.dataref) }
    }
}
// Double write
impl DataRef<f64, ReadWrite> {
    /// Sets the value of this dataref
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
        let mut buffer = StringBuffer::new(length);
        // Copy data in
        unsafe { XPLMGetDatab(self.dataref, buffer.as_mut_ptr() as *mut libc::c_void, 0, array_length(length)) };
        // Convert into a string
        buffer.as_string()
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
