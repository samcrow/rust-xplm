

use xplm_sys::data_access::*;

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
pub fn array_length(length: usize) -> i32 {
    if length > (i32::max_value() as usize) {
        i32::max_value()
    }
    else {
        length as i32
    }
}
