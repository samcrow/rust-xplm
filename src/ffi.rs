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

use std::iter;
use std::os::raw::c_char;
use std::str;
use std::str::Utf8Error;
use std::string::FromUtf8Error;

/// A fixed-length array of characters that can be passed to C functions and converted into a
/// String
#[derive(Debug)]
pub struct StringBuffer {
    /// The bytes in this buffer
    bytes: Vec<u8>,
}

#[allow(dead_code)]
impl StringBuffer {
    /// Creates a new StringBuffer with the provided length in bytes. All bytes in the string are
    /// set to null bytes (`\0`).
    pub fn new(length: usize) -> StringBuffer {
        StringBuffer {
            bytes: iter::repeat(b'\0').take(length).collect(),
        }
    }

    /// Returns a mutable pointer to the data in this buffer
    pub unsafe fn as_mut_ptr(&mut self) -> *mut c_char {
        self.bytes.as_mut_ptr() as *mut c_char
    }

    /// Returns the bytes in this buffer
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Returns a mutable slice into the bytes in this buffer
    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        &mut self.bytes
    }

    /// Returns a reference to the string in this buffer
    ///
    /// The returned string will not contain any null bytes.
    ///
    /// An error is returned if the data in this buffer is not valid UTF-8.
    pub fn as_str(&self) -> Result<&str, Utf8Error> {
        let chars_before_null = self.bytes.iter().take_while(|&&c| c != b'\0').count();
        str::from_utf8(&self.bytes[..chars_before_null])
    }

    /// Converts this buffer into a String
    ///
    /// The returned string will not contain any null bytes.
    ///
    /// An error is returned if the data in this buffer is not valid UTF-8.
    pub fn into_string(self) -> Result<String, FromUtf8Error> {
        let chars_before_null = self.bytes.into_iter().take_while(|&c| c != b'\0');
        String::from_utf8(chars_before_null.collect())
    }
}
