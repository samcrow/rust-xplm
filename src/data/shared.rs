use std::ffi::{CString, NulError};
use std::ptr;

use xplm_sys::data_access::*;

use super::*;

///
/// Provides access to a shared dataref
///
#[derive(Debug)]
pub struct Shared<D, A> {
    /// The shared dataref
    /// (Shared just wraps a dataref and shares/unshares it)
    dataref: Borrowed<D, A>,
    // Other arguments are used to unshare data
    /// The dataref name
    name: CString,
    /// The data type
    data_type: XPLMDataTypeID,
}

impl<D, A> Shared<D, A> where D: DataType, A: DataAccess {
    ///
    /// Finds a dataref with the provided name. If a shared dataref with the provided name already
    /// exists, it will be found. Otherwise, a shared dataref will be created.
    /// Returns a Shared object or an error
    ///
    pub fn find(name: &str) -> Result<Shared<D, A>, SearchError> {
        match CString::new(name) {
            Ok(name_c) => unsafe {
                // Check share
                let result = XPLMShareData(name_c.as_ptr(), D::data_type(), None, ptr::null_mut());
                match result {
                    1 => {
                        // Proceed
                        let borrowed = try!(Borrowed::find(name));
                        Ok(Shared {
                            dataref: borrowed,
                            name: name_c,
                            data_type: D::data_type(),
                        })
                    },
                    _ => Err(SearchError::WrongDataType),
                }
            },
            Err(e) => Err(SearchError::InvalidName(e)),
        }
    }
}

impl<D, A> Drop for Shared<D, A> {
    fn drop(&mut self) {
        // Unshare the data
        // If this is the last plugin to unshare it, the memory will be deallocated
        unsafe {
            XPLMUnshareData(self.name.as_ptr(), self.data_type, None, ptr::null_mut());
        }
    }
}

// Integer read
impl<A> Readable<i32> for Shared<i32, A> {
    fn get(&self) -> i32 {
        self.dataref.get()
    }
}
// Integer write
impl Writeable<i32> for Shared<i32, ReadWrite> {
    fn set(&mut self, value: i32) {
        self.dataref.set(value)
    }
}
// Float read
impl<A> Readable<f32> for Shared<f32, A> {
    fn get(&self) -> f32 {
        self.dataref.get()
    }
}
// Float write
impl Writeable<f32> for Shared<f32, ReadWrite> {
    fn set(&mut self, value: f32) {
        self.dataref.set(value)
    }
}
// Double read
impl<A> Readable<f64> for Shared<f64, A> {
    fn get(&self) -> f64 {
        self.dataref.get()
    }
}
// Double write
impl Writeable<f64> for Shared<f64, ReadWrite> {
    fn set(&mut self, value: f64) {
        self.dataref.set(value)
    }
}

// Integer array read
impl<A> Readable<Vec<i32>> for Shared<Vec<i32>, A> {
    fn get(&self) -> Vec<i32> {
        self.dataref.get()
    }
}
impl<A> ArrayReadable<i32> for Shared<Vec<i32>, A> {
    fn len(&self) -> usize {
        self.dataref.len()
    }
}
// Integer array write
impl Writeable<Vec<i32>> for Shared<Vec<i32>, ReadWrite> {
    fn set(&mut self, value: Vec<i32>) {
        self.dataref.set(value)
    }
}
impl ArrayWriteable<i32> for Shared<Vec<i32>, ReadWrite> {
    fn set_from_slice(&mut self, value: &[i32]) {
        self.dataref.set_from_slice(value)
    }
}

// Float array read
impl<A> Readable<Vec<f32>> for Shared<Vec<f32>, A> {
    fn get(&self) -> Vec<f32> {
        self.dataref.get()
    }
}
impl<A> ArrayReadable<f32> for Shared<Vec<f32>, A> {
    fn len(&self) -> usize {
        self.dataref.len()
    }
}
// Float array write
impl Writeable<Vec<f32>> for Shared<Vec<f32>, ReadWrite> {
    fn set(&mut self, value: Vec<f32>) {
        self.dataref.set(value)
    }
}
impl ArrayWriteable<f32> for Shared<Vec<f32>, ReadWrite> {
    fn set_from_slice(&mut self, value: &[f32]) {
        self.dataref.set_from_slice(value)
    }
}

// Byte array read
impl<A> Readable<Vec<u8>> for Shared<Vec<u8>, A> {
    fn get(&self) -> Vec<u8> {
        self.dataref.get()
    }
}
impl<A> ArrayReadable<u8> for Shared<Vec<u8>, A> {
    fn len(&self) -> usize {
        self.dataref.len()
    }
}
// Byte array write
impl Writeable<Vec<u8>> for Shared<Vec<u8>, ReadWrite> {
    fn set(&mut self, value: Vec<u8>) {
        self.dataref.set(value)
    }
}
impl ArrayWriteable<u8> for Shared<Vec<u8>, ReadWrite> {
    fn set_from_slice(&mut self, value: &[u8]) {
        self.dataref.set_from_slice(value)
    }
}
// String read
impl<A> Readable<String> for Shared<String, A> {
    fn get(&self) -> String {
        self.dataref.get()
    }
}
impl<A> StringReadable for Shared<String, A> {
    fn len(&self) -> usize {
        self.dataref.len()
    }
}
// String write
impl Writeable<String> for Shared<String, ReadWrite> {
    fn set(&mut self, value: String) {
        self.dataref.set(value)
    }
}
impl StringWriteable for Shared<String, ReadWrite> {
    fn set_string(&mut self, value: &str) -> Result<(), NulError> {
        self.dataref.set_string(value)
    }
}
