
extern crate libc;

use std::marker::PhantomData;
use std::ffi::{CString, NulError};
use std::ptr;

use xplm_sys::data_access::*;

use super::*;
use super::array_length;
use ffi::StringBuffer;

///
/// Provides safe access to a dataref that X-Plane or another plugin has already
/// created
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
/// When a `Borrowed` is created, its type and writeability are checked against the type and
/// writeability specified by X-Plane. A `Borrowed` can only be created if X-Plane knows about
/// a dataref with the same name and compatible type and writeabiltiy.
///
/// # Examples
///
/// ## Read-only f32 dataref
///
/// ```no_run
/// let time_ref: Borrowed<f32, ReadOnly> = Borrowed::find("sim/time/total_running_time_sec").unwrap();
/// let time = time_ref.get();
/// ```
///
/// ## Read-only String dataref
///
/// ```no_run
/// let dataref: Borrowed<String, ReadOnly> = Borrowed::find("sim/version/sim_build_string").unwrap();
/// let sim_build_time = dataref.get();
/// ```
///
/// ## Writeable i32 dataref
///
/// ```no_run
/// let dataref: Borrowed<i32, ReadWrite> = Borrowed::find("sim/cockpit2/autopilot/flight_director_mode").unwrap();
/// let mode = dataref.get();
/// dataref.set(3);
/// ```
///
#[derive(Debug,Clone)]
pub struct Borrowed<D, A> {
    /// Dataref handle
    dataref: XPLMDataRef,
    /// Phantom data for data type
    type_phantom: PhantomData<D>,
    /// Phantom data for access
    access_phantom: PhantomData<A>,
}

impl<D, A> Borrowed<D, A> where D: DataType, A: DataAccess {
    ///
    /// Finds a dataref with the provided name.
    /// Returns a Borrowed object or an error
    ///
    pub fn find(name: &str) -> Result<Borrowed<D, A>, SearchError> {
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
                Ok(Borrowed {
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
impl<A> Readable<i32> for Borrowed<i32, A> {
    fn get(&self) -> i32 {
        unsafe { XPLMGetDatai(self.dataref) }
    }
}
// Integer write
impl Writeable<i32> for Borrowed<i32, ReadWrite> {
    fn set(&mut self, value: i32) {
        unsafe { XPLMSetDatai(self.dataref, value) }
    }
}
// Float read
impl<A> Readable<f32> for Borrowed<f32, A> {
    fn get(&self) -> f32 {
        unsafe { XPLMGetDataf(self.dataref) }
    }
}
// Float write
impl Writeable<f32> for Borrowed<f32, ReadWrite> {
    fn set(&mut self, value: f32) {
        unsafe { XPLMSetDataf(self.dataref, value) }
    }
}
// Double read
impl<A> Readable<f64> for Borrowed<f64, A> {
    fn get(&self) -> f64 {
        unsafe { XPLMGetDatad(self.dataref) }
    }
}
// Double write
impl Writeable<f64> for Borrowed<f64, ReadWrite> {
    fn set(&mut self, value: f64) {
        unsafe { XPLMSetDatad(self.dataref, value) }
    }
}

// Integer array read
impl<A> Readable<Vec<i32>> for Borrowed<Vec<i32>, A> {
    fn get(&self) -> Vec<i32> {
        let length = self.len();
        let mut values = Vec::with_capacity(length);
        unsafe {
            values.set_len(length);
            XPLMGetDatavi(self.dataref, values.as_mut_ptr(), 0, array_length(length));
        }
        values
    }
}
impl<A> ArrayReadable<i32> for Borrowed<Vec<i32>, A> {
    fn len(&self) -> usize {
        let size_int = unsafe { XPLMGetDatavi(self.dataref, ptr::null_mut(), 0, 0) };
        size_int as usize
    }
}
// Integer array write
impl Writeable<Vec<i32>> for Borrowed<Vec<i32>, ReadWrite> {
    fn set(&mut self, value: Vec<i32>) {
        self.set_from_slice(&value)
    }
}
impl ArrayWriteable<i32> for Borrowed<Vec<i32>, ReadWrite> {
    fn set_from_slice(&mut self, value: &[i32]) {
        unsafe {
            XPLMSetDatavi(self.dataref, value.as_ptr() as *mut i32,
                0, array_length(value.len()));
        }
    }
}

// Float array read
impl<A> Readable<Vec<f32>> for Borrowed<Vec<f32>, A> {
    fn get(&self) -> Vec<f32> {
        let length = self.len();
        let mut values = Vec::with_capacity(length);
        unsafe {
            values.set_len(length);
            XPLMGetDatavf(self.dataref, values.as_mut_ptr(), 0, array_length(length));
        }
        values
    }
}
impl<A> ArrayReadable<f32> for Borrowed<Vec<f32>, A> {
    fn len(&self) -> usize {
        let size_int = unsafe { XPLMGetDatavf(self.dataref, ptr::null_mut(), 0, 0) };
        size_int as usize
    }
}
// Float array write
impl Writeable<Vec<f32>> for Borrowed<Vec<f32>, ReadWrite> {
    fn set(&mut self, value: Vec<f32>) {
        self.set_from_slice(&value)
    }
}
impl ArrayWriteable<f32> for Borrowed<Vec<f32>, ReadWrite> {
    fn set_from_slice(&mut self, value: &[f32]) {
        unsafe {
            XPLMSetDatavf(self.dataref, value.as_ptr() as *mut f32,
                0, array_length(value.len()));
        }
    }
}

// Byte array read
impl<A> Readable<Vec<u8>> for Borrowed<Vec<u8>, A> {
    fn get(&self) -> Vec<u8> {
        let length = self.len();
        let mut values = Vec::with_capacity(length);
        unsafe {
            values.set_len(length);
            XPLMGetDatab(self.dataref, values.as_mut_ptr() as *mut libc::c_void,
                0, array_length(length));
        }
        values
    }
}
impl<A> ArrayReadable<u8> for Borrowed<Vec<u8>, A> {
    fn len(&self) -> usize {
        let size_int = unsafe { XPLMGetDatab(self.dataref, ptr::null_mut(), 0, 0) };
        size_int as usize
    }
}
// Byte array write
impl Writeable<Vec<u8>> for Borrowed<Vec<u8>, ReadWrite> {
    fn set(&mut self, value: Vec<u8>) {
        self.set_from_slice(&value)
    }
}
impl ArrayWriteable<u8> for Borrowed<Vec<u8>, ReadWrite> {
    fn set_from_slice(&mut self, value: &[u8]) {
        unsafe {
            XPLMSetDatab(self.dataref, value.as_ptr() as *mut libc::c_void,
                0, array_length(value.len()));
        }
    }
}
// String read
impl<A> Readable<String> for Borrowed<String, A> {
    fn get(&self) -> String {
        // Create a byte array of the right length
        let length = self.len();
        let mut buffer = StringBuffer::new(length);
        // Copy data in
        unsafe { XPLMGetDatab(self.dataref, buffer.as_mut_ptr() as *mut libc::c_void,
            0, array_length(length)) };
        // Convert into a string
        buffer.as_string()
    }
}
impl<A> StringReadable for Borrowed<String, A> {
    fn len(&self) -> usize {
        let size_int = unsafe { XPLMGetDatab(self.dataref, ptr::null_mut(), 0, 0) };
        size_int as usize
    }
}
impl Writeable<String> for Borrowed<String, ReadWrite> {
    fn set(&mut self, value: String) {
        match CString::new(value) {
            Ok(value_c) => unsafe {
                XPLMSetDatab(self.dataref, value_c.as_ptr() as *mut libc::c_void,
                    0, array_length(value_c.to_bytes().len()));
            },
            // NulError -> do nothing
            Err(_) => {},
        }
    }
}
// String write
impl StringWriteable for Borrowed<String, ReadWrite> {
    fn set_string(&mut self, value: &str) -> Result<(), NulError> {
        let value_c = try!(CString::new(value));
        unsafe { XPLMSetDatab(self.dataref, value_c.as_ptr() as *mut libc::c_void, 0,
            array_length(value_c.as_bytes_with_nul().len())) };
        Ok(())
    }
}
