
use std::marker::PhantomData;
use std::ffi::CString;

use xplm_sys::data_access::*;

use super::*;

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
/// When a `DataRef` is created, its type and writeability are checked against the type and
/// writeability specified by X-Plane. A `DataRef` can only be created if X-Plane knows about
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
    /// Returns a DataRef object or an error
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

impl<D, A> DataRef<D, A> for Borrowed<D, A> where D: DataType, A: DataAccess {
    fn dataref(&self) -> XPLMDataRef { self.dataref }
}
impl<'a, D, A> DataRef<D, A> for &'a Borrowed<D, A> where D: DataType, A: DataAccess {
    fn dataref(&self) -> XPLMDataRef { self.dataref }
}
