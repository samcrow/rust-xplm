use std::os::raw::*;
use xplm_sys;

/// A callback that can be called while X-Plane draws graphics
pub trait DrawCallback: 'static {
    /// Draws
    ///
    /// Return true to allow X-Plane to draw in this phase. Return false to prevent X-Plane from
    /// drawing in this phase. This has no effect in After* phases.
    fn draw(&mut self) -> bool;
}

impl<F> DrawCallback for F where F: 'static + FnMut() -> bool {
    fn draw(&mut self) -> bool {
        self()
    }
}

/// Sets up a draw callback
pub struct Draw {
    /// The callback to execute
    _callback: Box<DrawCallback>,
    /// The draw phase (used when unregistering)
    phase: Phase,
    /// The callback pointer (used when unregistering)
    callback_ptr: *mut c_void,
    /// The C callback (used when unregistering)
    c_callback: xplm_sys::XPLMDrawCallback_f,
}

impl Draw {
    /// Creates a new drawing callback
    pub fn new<C: DrawCallback>(phase: Phase, callback: C) -> Result<Self, Error> {
        let (xplm_phase, before) = phase.to_xplm();
        let callback_box = Box::new(callback);
        let callback_ptr: *const _ = &*callback_box;
        let status = unsafe {
            xplm_sys::XPLMRegisterDrawCallback(Some(draw_callback::<C>), xplm_phase, before, callback_ptr as *mut _)
        };
        if status == 1 {
            Ok(Draw {
                _callback: callback_box,
                phase: phase,
                callback_ptr: callback_ptr as *mut _,
                c_callback: Some(draw_callback::<C>),
            })
        } else {
            Err(Error::UnsupportedPhase(phase))
        }
    }
}

impl Drop for Draw {
    /// Unregisters this draw callback
    fn drop(&mut self) {
        let (phase, before) = self.phase.to_xplm();
        unsafe {
            xplm_sys::XPLMUnregisterDrawCallback(self.c_callback, phase, before, self.callback_ptr);
        }
    }
}

/// The draw callback provided to X-Plane
///
/// This is instantiated separately for each callback type.
unsafe extern "C" fn draw_callback<C: DrawCallback>(_phase: xplm_sys::XPLMDrawingPhase, _before: c_int, refcon: *mut c_void) -> c_int {
    let callback_ptr = refcon as *mut C;
    let allow_draw = (*callback_ptr).draw();
    allow_draw as c_int
}

/// Phases in which drawing can occur
#[derive(Debug, Copy, Clone)]
pub enum Phase {
    /// The first 3D scene drawing phase
    Initial3D,
    /// Before X-Plane draws terrain and water
    BeforeTerrain,
    /// After X-Plane draws terrain and water
    AfterTerrain,
    /// Before X-Plane draws airports
    BeforeAirports,
    /// After X-Plane draws airports
    AfterAirports,
    /// Before X-Plane draws vectors (including roads)
    BeforeVectors,
    /// After X-Plane draws vectors (including roads)
    AfterVectors,
    /// Before X-Plane draws scenery objects
    BeforeObjects,
    /// After X-Plane draws scenery objects
    AfterObjects,
    /// Before X-Plane draws aircraft
    BeforeAircraft,
    /// After X-Plane draws aircraft
    AfterAircraft,
    /// The final 3D scene drawing phase
    Final3D,
    /// The first 2D drawing phase
    Initial2D,
    /// Before X-Plane draws the cockpit panel
    BeforePanel,
    /// After X-Plane draws the cockpit panel
    AfterPanel,
    /// Before X-Plane draws panel gauges
    BeforeGauges,
    /// After X-Plane draws panel gauges
    AfterGauges,
    /// Before X-Plane draws user interface windows
    BeforeWindows,
    /// After X-Plane draws user interface windows
    AfterWindows,
    /// The final 2D drawing phase
    Final2D,
    /// Before X-Plane draws 3D content in the local map window
    #[cfg(feature = "xplm200")]
    BeforeLocalMap3D,
    /// After X-Plane draws 3D content in the local map window
    #[cfg(feature = "xplm200")]
    AfterLocalMap3D,
    /// Before X-Plane draws 2D content in the local map window
    #[cfg(feature = "xplm200")]
    BeforeLocalMap2D,
    /// After X-Plane draws 2D content in the local map window
    #[cfg(feature = "xplm200")]
    AfterLocalMap2D,
    /// Before X-Plane draws 2D content in the local map profile view
    #[cfg(feature = "xplm200")]
    BeforeLocalMapProfile,
    /// After X-Plane draws 2D content in the local map profile view
    #[cfg(feature = "xplm200")]
    AfterLocalMapProfile,
}

impl Phase {
    /// Converts this phase into an XPLMDrawingPhase and a 0 for after or 1 for before
    fn to_xplm(&self) -> (xplm_sys::XPLMDrawingPhase, c_int) {
        use self::Phase::*;
        let (phase, before) = match *self {
            Initial3D => (xplm_sys::xplm_Phase_FirstScene, 0),
            BeforeTerrain => (xplm_sys::xplm_Phase_Terrain, 1),
            AfterTerrain => (xplm_sys::xplm_Phase_Terrain, 0),
            BeforeAirports => (xplm_sys::xplm_Phase_Airports, 1),
            AfterAirports => (xplm_sys::xplm_Phase_Airports, 0),
            BeforeVectors => (xplm_sys::xplm_Phase_Vectors, 1),
            AfterVectors => (xplm_sys::xplm_Phase_Vectors, 0),
            BeforeObjects => (xplm_sys::xplm_Phase_Objects, 1),
            AfterObjects => (xplm_sys::xplm_Phase_Objects, 0),
            BeforeAircraft => (xplm_sys::xplm_Phase_Airplanes, 1),
            AfterAircraft => (xplm_sys::xplm_Phase_Airplanes, 0),
            Final3D => (xplm_sys::xplm_Phase_LastScene, 0),
            Initial2D => (xplm_sys::xplm_Phase_FirstCockpit, 0),
            BeforePanel => (xplm_sys::xplm_Phase_Panel, 1),
            AfterPanel => (xplm_sys::xplm_Phase_Panel, 0),
            BeforeGauges => (xplm_sys::xplm_Phase_Gauges, 1),
            AfterGauges => (xplm_sys::xplm_Phase_Gauges, 0),
            BeforeWindows => (xplm_sys::xplm_Phase_Window, 1),
            AfterWindows => (xplm_sys::xplm_Phase_Window, 0),
            Final2D => (xplm_sys::xplm_Phase_LastCockpit, 0),
            #[cfg(feature = "xplm200")]
            BeforeLocalMap2D => (xplm_sys::xplm_Phase_LocalMap2D, 1),
            #[cfg(feature = "xplm200")]
            AfterLocalMap2D => (xplm_sys::xplm_Phase_LocalMap2D, 0),
            #[cfg(feature = "xplm200")]
            BeforeLocalMap3D => (xplm_sys::xplm_Phase_LocalMap3D, 1),
            #[cfg(feature = "xplm200")]
            AfterLocalMap3D => (xplm_sys::xplm_Phase_LocalMap3D, 0),
            #[cfg(feature = "xplm200")]
            BeforeLocalMapProfile => (xplm_sys::xplm_Phase_LocalMapProfile, 1),
            #[cfg(feature = "xplm200")]
            AfterLocalMapProfile => (xplm_sys::xplm_Phase_LocalMapProfile, 0),
        };
        (phase as xplm_sys::XPLMDrawingPhase, before)
    }
}

quick_error! {
    /// Errors that may occur when creating a draw callback
    #[derive(Debug)]
    pub enum Error {
        /// X-Plane does not support the provided phase
        UnsupportedPhase(phase: Phase) {
            description("unsupported draw phase")
            display("Unsupported phase {:?}", phase)
        }
    }
}
