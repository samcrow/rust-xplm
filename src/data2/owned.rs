
use super::*;
use super::array_length;

use xplm_sys::data_access::*;

extern crate libc;

use std::cmp::min;
use std::ffi::{CString, NulError};
use std::marker::PhantomData;
use std::ptr;
use std::boxed::Box;

///
/// A dataref created and owned by this plugin
///
/// The DataAccess type parameter for owned datarefs is the access that other plugins have.
/// This plugin can always write datarefs that it owns.
///
pub struct Owned<D, A> {
    /// The associated data, allocated in a Box.
    /// refcons in the callbacks are pointers to this.
    inner: *mut InnerOwnedData<D, A>
}

impl<D, A> Owned<D, A> where D: DataType, A: DataAccess {
    /// Creates a dataref with the provided name, set to the provided value
    ///
    /// Returns the dataref on success, or an error if the provided name was invalid.
    #[allow(non_upper_case_globals)]
    pub fn create(name: &str, initial_value: D) -> Result<Owned<D, A>, NulError> {
        let name_c = try!(CString::new(name));
        let inner = Box::into_raw(Box::new(InnerOwnedData {
            dataref: ptr::null_mut(),
            value: initial_value,
            access_phantom: PhantomData,
        }));
        // Select the correct callbacks based on the data type and writeability
        let read_i = match D::data_type() as u32 {
            xplmType_Int => Some(get_data_i
                 as unsafe extern "C" fn(*mut ::libc::c_void) -> ::libc::c_int),
            _ => None,
        };
        let write_i = match (D::data_type() as u32, A::writeable()) {
            (xplmType_Int, true) => Some(set_data_i
                 as unsafe extern "C" fn(*mut ::libc::c_void, ::libc::c_int)),
            _ => None,
        };
        let read_f = match D::data_type() as u32 {
            xplmType_Float => Some(get_data_f
                 as unsafe extern "C" fn(*mut ::libc::c_void) -> ::libc::c_float),
            _ => None,
        };
        let write_f = match (D::data_type() as u32, A::writeable()) {
            (xplmType_Float, true) => Some(set_data_f
                 as unsafe extern "C" fn(*mut ::libc::c_void, ::libc::c_float)),
            _ => None,
        };
        let read_d = match D::data_type() as u32 {
            xplmType_Double => Some(get_data_d
                 as unsafe extern "C" fn(*mut ::libc::c_void) -> ::libc::c_double),
            _ => None,
        };
        let write_d = match (D::data_type() as u32, A::writeable()) {
            (xplmType_Double, true) => Some(set_data_d
                 as unsafe extern "C" fn(*mut ::libc::c_void, ::libc::c_double)),
            _ => None,
        };
        let read_vi = match D::data_type() as u32 {
            xplmType_IntArray => Some(get_data_vi
                 as unsafe extern "C" fn(*mut ::libc::c_void, *mut ::libc::c_int, ::libc::c_int,
                 ::libc::c_int) -> ::libc::c_int),
            _ => None,
        };
        let write_vi = match (D::data_type() as u32, A::writeable()) {
            (xplmType_IntArray, true) => Some(set_data_vi
                as unsafe extern "C" fn(*mut ::libc::c_void, *mut ::libc::c_int, ::libc::c_int,
                ::libc::c_int)),
            _ => None,
        };
        let read_vf = match D::data_type() as u32 {
            xplmType_FloatArray => Some(get_data_vf
                 as unsafe extern "C" fn(*mut ::libc::c_void, *mut ::libc::c_float, ::libc::c_int,
                 ::libc::c_int) -> ::libc::c_int),
            _ => None,
        };
        let write_vf = match (D::data_type() as u32, A::writeable()) {
            (xplmType_FloatArray, true) => Some(set_data_vf
                as unsafe extern "C" fn(*mut ::libc::c_void, *mut ::libc::c_float, ::libc::c_int,
                ::libc::c_int)),
            _ => None,
        };
        let read_b = match D::data_type() as u32 {
            xplmType_Data => Some(get_data_b
                 as unsafe extern "C" fn(*mut ::libc::c_void, *mut ::libc::c_void, ::libc::c_int,
                 ::libc::c_int) -> ::libc::c_int),
            _ => None,
        };
        let write_b = match (D::data_type() as u32, A::writeable()) {
            (xplmType_Data, true) => Some(set_data_b
                as unsafe extern "C" fn(*mut ::libc::c_void, *mut ::libc::c_void, ::libc::c_int,
                ::libc::c_int)),
            _ => None,
        };

        unsafe {
            // Register a dataref, and provide the address of the inner data as a refcon
            let dataref = XPLMRegisterDataAccessor(name_c.as_ptr(), D::data_type(),
                A::writeable() as i32,
                read_i, write_i, read_f, write_f, read_d, write_d, read_vi, write_vi, read_vf,
                write_vf, read_b, write_b,
                inner as *mut ::libc::c_void, inner as *mut ::libc::c_void);
            assert!(!dataref.is_null());
            (*inner).dataref = dataref;
        }
        Ok(Owned {
            inner: inner,
        })
    }
}

impl<D, A> Drop for Owned<D, A> {
    fn drop(&mut self) {
        // Deallocate inner
        let inner_box = unsafe { Box::from_raw(self.inner) };
        drop(inner_box);
    }
}

///
/// Implement DataRef<D, ReadWrite> so that this plugin can write datarefs it owns regardless
/// of other plugins' access.
///
/// This implementation goes through X-Plane. A faster set of implementations would access
/// the values directly.
///
impl<D, A> DataRef<D, ReadWrite> for Owned<D, A> {
    fn dataref(&self) -> XPLMDataRef {
        unsafe { (*self.inner).dataref }
    }
}

///
/// Stores the actual data
///
struct InnerOwnedData<D, A> {
    /// Dataref handle
    dataref: XPLMDataRef,
    /// The current value of the dataref
    value: D,
    /// Phantom data for access
    access_phantom: PhantomData<A>,
}


impl<D, A> Drop for InnerOwnedData<D, A> {
    fn drop(&mut self) {
        unsafe { XPLMUnregisterDataAccessor(self.dataref); }
    }
}


// Callbacks
unsafe extern "C" fn get_data_i(refcon: *mut ::libc::c_void) -> ::libc::c_int {
    let data = refcon as *const InnerOwnedData<i32, ReadOnly>;
    (*data).value
}
unsafe extern "C" fn set_data_i(refcon: *mut ::libc::c_void, value: ::libc::c_int) {
    let data = refcon as *mut InnerOwnedData<i32, ReadWrite>;
    (*data).value = value;
}
unsafe extern "C" fn get_data_f(refcon: *mut ::libc::c_void) -> ::libc::c_float {
    let data = refcon as *const InnerOwnedData<f32, ReadOnly>;
    (*data).value
}
unsafe extern "C" fn set_data_f(refcon: *mut ::libc::c_void, value: ::libc::c_float) {
    let data = refcon as *mut InnerOwnedData<f32, ReadWrite>;
    (*data).value = value;
}
unsafe extern "C" fn get_data_d(refcon: *mut ::libc::c_void) -> ::libc::c_double {
    let data = refcon as *const InnerOwnedData<f64, ReadOnly>;
    (*data).value
}
unsafe extern "C" fn set_data_d(refcon: *mut ::libc::c_void, value: ::libc::c_double) {
    let data = refcon as *mut InnerOwnedData<f64, ReadWrite>;
    (*data).value = value;
}
unsafe extern "C" fn get_data_vi(refcon: *mut ::libc::c_void, values: *mut ::libc::c_int,
                                           offset: ::libc::c_int, max: ::libc::c_int)
                                            -> ::libc::c_int {
    let data = refcon as *const InnerOwnedData<Vec<i32>, ReadOnly>;
    handle_read(&(*data).value, values, offset as usize, max as usize)
}
unsafe extern "C" fn set_data_vi(refcon: *mut ::libc::c_void, values: *mut ::libc::c_int,
                                           offset: ::libc::c_int, max: ::libc::c_int) {
    let data = refcon as *mut InnerOwnedData<Vec<i32>, ReadWrite>;
    handle_write(&mut (*data).value, values as *const ::libc::c_int,offset as usize, max as usize);
}
unsafe extern "C" fn get_data_vf(refcon: *mut ::libc::c_void, values: *mut ::libc::c_float,
                                           offset: ::libc::c_int, max: ::libc::c_int)
                                            -> ::libc::c_int {
    let data = refcon as *mut InnerOwnedData<Vec<f32>, ReadOnly>;
    handle_read(&(*data).value, values, offset as usize, max as usize)
}
unsafe extern "C" fn set_data_vf(refcon: *mut ::libc::c_void, values: *mut ::libc::c_float,
                                           offset: ::libc::c_int, max: ::libc::c_int) {
    let data = refcon as *mut InnerOwnedData<Vec<f32>, ReadWrite>;
    handle_write(&mut (*data).value, values as *const ::libc::c_float, offset as usize,
    max as usize);
}
unsafe extern "C" fn get_data_b(refcon: *mut ::libc::c_void, values: *mut ::libc::c_void,
                                           offset: ::libc::c_int, max: ::libc::c_int)
                                            -> ::libc::c_int {
    let data = refcon as *mut InnerOwnedData<Vec<u8>, ReadOnly>;
    handle_read(&(*data).value, values as *mut u8, offset as usize, max as usize)
}
unsafe extern "C" fn set_data_b(refcon: *mut ::libc::c_void, values: *mut ::libc::c_void,
                                           offset: ::libc::c_int, max: ::libc::c_int) {
    let data = refcon as *mut InnerOwnedData<Vec<u8>, ReadWrite>;
    handle_write(&mut (*data).value, values as *const u8, offset as usize, max as usize);
}

/// Handles a read request
unsafe fn handle_read<T>(data: &[T], out_values: *mut T, offset: usize, max: usize) -> i32
    where T: Clone {
    if out_values.is_null() {
        array_length(data.len())
    }
    else {
        let upper_bound = min(data.len(), offset + max);
        if upper_bound <= offset {
            0
        }
        else {
            for i in offset..upper_bound {
                let non_offset = i - offset;
                *(out_values.offset(non_offset as isize)) = data[i].clone();
            }
            array_length(upper_bound - offset)
        }
    }
}
/// Handles a write request
unsafe fn handle_write<T>(data: &mut [T], in_values: *const T, offset: usize, max: usize)
    where T: Clone {
    if in_values.is_null() { return; }
    let upper_bound = min(data.len(), offset + max);
    for i in offset..upper_bound {
        let non_offset = i - offset;
        data[i] = (*(in_values.offset(non_offset as isize))).clone();
    }
}
