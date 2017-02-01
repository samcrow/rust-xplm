use std::os::raw::*;
use xplm_sys;

/// A callback that can be called while X-Plane draws graphics
pub trait DrawCallback: 'static {
    /// Draws
    fn draw(&mut self);
}

impl<F> DrawCallback for F where F: 'static + FnMut() {
    fn draw(&mut self) {
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
        let xplm_phase = phase.to_xplm();
        let callback_box = Box::new(callback);
        let callback_ptr: *const _ = &*callback_box;
        let status = unsafe {
            xplm_sys::XPLMRegisterDrawCallback(Some(draw_callback::<C>), xplm_phase, 0, callback_ptr as *mut _)
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
        let phase = self.phase.to_xplm();
        unsafe {
            xplm_sys::XPLMUnregisterDrawCallback(self.c_callback, phase, 0, self.callback_ptr);
        }
    }
}

/// The draw callback provided to X-Plane
///
/// This is instantiated separately for each callback type.
unsafe extern "C" fn draw_callback<C: DrawCallback>(_phase: xplm_sys::XPLMDrawingPhase, _before: c_int, refcon: *mut c_void) -> c_int {
    let callback_ptr = refcon as *mut C;
    (*callback_ptr).draw();
    // Always allow X-Plane to draw
    1
}

/// Phases in which drawing can occur
#[derive(Debug, Copy, Clone)]
pub enum Phase {
    /// After X-Plane draws terrain and water
    AfterTerrain,
    /// After X-Plane draws airports
    AfterAirports,
    /// After X-Plane draws scenery objects
    AfterObjects,
    /// After X-Plane draws aircraft
    AfterAircraft,
    /// After X-Plane draws the cockpit panel
    AfterPanel,
    /// After X-Plane draws panel gauges
    AfterGauges,
    /// After X-Plane draws user interface windows
    AfterWindows,
    /// After X-Plane draws 3D content in the local map window
    #[cfg(feature = "xplm200")]
    AfterLocalMap3D,
    /// After X-Plane draws 2D content in the local map window
    #[cfg(feature = "xplm200")]
    AfterLocalMap2D,
    /// After X-Plane draws 2D content in the local map profile view
    #[cfg(feature = "xplm200")]
    AfterLocalMapProfile,
}

impl Phase {
    /// Converts this phase into an XPLMDrawingPhase and a 0 for after or 1 for before
    fn to_xplm(&self) -> xplm_sys::XPLMDrawingPhase {
        use self::Phase::*;
        let phase = match *self {
            AfterTerrain => xplm_sys::xplm_Phase_Terrain,
            AfterAirports => xplm_sys::xplm_Phase_Airports,
            AfterObjects => xplm_sys::xplm_Phase_Objects,
            AfterAircraft => xplm_sys::xplm_Phase_Airplanes,
            AfterPanel => xplm_sys::xplm_Phase_Panel,
            AfterGauges => xplm_sys::xplm_Phase_Gauges,
            AfterWindows => xplm_sys::xplm_Phase_Window,
            #[cfg(feature = "xplm200")]
            AfterLocalMap2D => xplm_sys::xplm_Phase_LocalMap2D,
            #[cfg(feature = "xplm200")]
            AfterLocalMap3D => xplm_sys::xplm_Phase_LocalMap3D,
            #[cfg(feature = "xplm200")]
            AfterLocalMapProfile => xplm_sys::xplm_Phase_LocalMapProfile,
        };
        phase as xplm_sys::XPLMDrawingPhase
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
