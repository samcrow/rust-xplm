// Copyright (c) 2015 rust-xplm developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.


/// Provides access to existing datarefs
pub mod dataref;
/// Allows creation of datarefs
pub mod owned_data;
/// Allows creation of and access to shared data
pub mod shared;

use xplm_sys::data_access::*;

use std::ffi::NulError;


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
