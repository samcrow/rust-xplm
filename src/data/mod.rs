// Copyright (c) 2015 rust-xplm developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.


/// Provides access to existing datarefs
mod borrowed;
/// Allows creation of datarefs
mod owned;
/// Allows creation of and access to shared data
mod shared;

extern crate libc;

use xplm_sys::data_access::*;

use std::ffi::NulError;
use std::fmt;
use std::error::Error;

// Use types
pub use self::borrowed::Borrowed;
pub use self::owned::Owned;
pub use self::shared::Shared;

/// A trait for objects that can be read to get a value
pub trait Readable<T> {
    /// Returns the value stored in this object
    fn get(&self) -> T;
}

/// A trait for objects in which a value can be stored
pub trait Writeable<T> {
    /// Sets the value in this object
    fn set(&mut self, value: T);
}
/// A trait for objects in which an array of values can be stored
pub trait ArrayReadable<T> : Readable<Vec<T>> {
    /// Returns the number of values in this array
    fn len(&self) -> usize;
}

/// A trait for objects that can be written like arrays
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
pub trait StringReadable : Readable<String> {
    /// Returns the length of this string value in bytes
    fn len(&self) -> usize;
}
///
/// A trait for objects that can be written as Strings
///
pub trait StringWriteable : Writeable<String> + StringReadable {
    /// Sets the values in this array from a string slice
    /// If the string contains one or more null bytes, an error
    /// is returned.
    fn set_string(&mut self, value: &str) -> Result<(), NulError>;
}

/// Possible errors encountered when finding a dataref
#[derive(Debug, Clone)]
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

impl fmt::Display for SearchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl Error for SearchError {
    fn description(&self) -> &str {
        match self {
            &SearchError::InvalidName(_) => "Invalid name",
            &SearchError::NotFound => "Not found",
            &SearchError::WrongDataType => "Wrong data type",
            &SearchError::WrongDataAccess => "Wrong data access",
        }
    }
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
    fn writeable() -> bool {
        true
    }
}

/// Marks a dataref that can only be read
#[derive(Debug,Clone)]
pub struct ReadOnly;
impl DataAccess for ReadOnly {
    fn writeable() -> bool {
        false
    }
}

/// Fits a length into an i32.
/// If the provided value is greater than i32::max_value, returns i32::max_value().
/// Otherwise, returns the value as an i32.
fn array_length(length: usize) -> i32 {
    if length > (i32::max_value() as usize) {
        i32::max_value()
    } else {
        length as i32
    }
}
