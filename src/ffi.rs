// Copyright (c) 2015 rust-xplm developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

//! Foreign function interface utilities
//!

extern crate libc;

/// A fixed-length array of characters that can be passed to C functions and converted into a
/// String
#[derive(Debug)]
pub struct StringBuffer {
    /// The bytes in this buffer
    bytes: Vec<u8>,
}

impl StringBuffer {
    /// Creates a new StringBuffer with the provided length in bytes. All bytes in the string are
    /// set to null bytes (`\0`).
    pub fn new(length: usize) -> StringBuffer {
        let mut bytes = Vec::with_capacity(length);
        for _ in 0..length {
            bytes.push(b'\0');
        }
        StringBuffer { bytes: bytes }
    }

    /// Returns a mutable pointer to the data in this buffer
    pub unsafe fn as_mut_ptr(&mut self) -> *mut libc::c_char {
        self.bytes.as_mut_ptr() as *mut libc::c_char
    }

    /// Returns a String containing all bytes in this buffer, up to and not including the first
    /// null byte.
    pub fn as_string(&self) -> String {
        let mut end_index = self.bytes.len();
        for (i, &byte) in self.bytes.iter().enumerate() {
            if byte == b'\0' {
                end_index = i;
                break;
            }
        }
        String::from_utf8_lossy(&self.bytes[0..end_index]).into_owned()
    }
}

/// Reexported types from libc
///
/// These types are not normally used directly in plugins.
pub mod types {
    pub use libc::c_int;
    pub use libc::c_char;
    pub use libc::c_void;
}
