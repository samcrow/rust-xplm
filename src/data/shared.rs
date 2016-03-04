use std::ffi::CString;
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

impl<D, A> DataRef<D, A> for Shared<D, A> where D: DataType, A: DataAccess {
    fn dataref(&self) -> XPLMDataRef {
        self.dataref.dataref()
    }
}
impl<'a, D, A> DataRef<D, A> for &'a Shared<D, A> where D: DataType, A: DataAccess {
    fn dataref(&self) -> XPLMDataRef {
        self.dataref.dataref()
    }
}
