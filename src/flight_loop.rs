//! # Flight loop callbacks
//!
//! X-Plane can call plugin code at timed intervals or when it runs its flight model.
//!
//! A FlightLoop object must persist for callbacks to occur. When the FlightLoop is dropped,
//! its callbacks will stop.
//!
//! # Examples
//!
//! Closure handler:
//!
//! ```no_run
//! use xplm::flight_loop::{FlightLoop, LoopState};
//!
//! let handler = |loop_state: &mut LoopState| {
//!     println!("Flight loop callback running");
//! };
//!
//! let mut flight_loop = FlightLoop::new(handler);
//! flight_loop.schedule_immediate();
//! ```
//!
//! Struct handler:
//!
//! ```no_run
//! use xplm::flight_loop::{FlightLoop, FlightLoopCallback, LoopState};
//!
//! struct LoopHandler;
//!
//! impl FlightLoopCallback for LoopHandler {
//!     fn flight_loop(&mut self, state: &mut LoopState) {
//!         println!("Flight loop callback running");
//!     }
//! }
//!
//! let mut flight_loop = FlightLoop::new(LoopHandler);
//! flight_loop.schedule_immediate();
//! ```
//!

use xplm_sys;

use std::f32;
use std::fmt;
use std::mem;
use std::ops::DerefMut;
use std::os::raw::*;
use std::time::Duration;

/// Tracks a flight loop callback, which can be called by X-Plane periodically for calculations
///
#[derive(Debug)]
pub struct FlightLoop {
    /// The loop data, allocated in a Box
    data: Box<LoopData>,
}

impl FlightLoop {
    /// Creates a new flight loop
    ///
    /// Provide the callback to be called
    ///
    /// The callback will not be called until it is scheduled
    pub fn new<C: FlightLoopCallback>(callback: C) -> Self {
        let mut data = Box::new(LoopData::new(callback));
        let data_ptr: *mut LoopData = data.deref_mut();
        // Create a flight loop
        let mut config = xplm_sys::XPLMCreateFlightLoop_t {
            structSize: mem::size_of::<xplm_sys::XPLMCreateFlightLoop_t>() as c_int,
            phase: xplm_sys::xplm_FlightLoop_Phase_AfterFlightModel as i32,
            callbackFunc: Some(flight_loop_callback::<C>),
            refcon: data_ptr as *mut c_void,
        };
        data.loop_id = unsafe { Some(xplm_sys::XPLMCreateFlightLoop(&mut config)) };
        FlightLoop { data }
    }

    /// Schedules the flight loop callback to be executed in the next flight loop
    ///
    /// After the flight loop callback is first called, it will continue to be called
    /// every flight loop unless it cancels itself or changes its schedule.
    pub fn schedule_immediate(&mut self) {
        self.data.set_interval(LoopResult::Loops(1))
    }

    /// Schedules the flight loop callback to be executed after a specified number of flight loops
    ///
    /// After the callback is first called, it will continue to be called with the provided loop
    /// interval.
    pub fn schedule_after_loops(&mut self, loops: u32) {
        self.data.set_interval(LoopResult::Loops(loops));
    }

    /// Schedules the flight loop callback to be executed after the specified delay
    ///
    /// After the callback is first called, it will continue to be called with that interval.
    pub fn schedule_after(&mut self, time: Duration) {
        let seconds_f = (time.as_secs() as f32) + (1e-9_f32 * time.subsec_nanos() as f32);
        self.data.set_interval(LoopResult::Seconds(seconds_f));
    }

    /// Deactivates the flight loop
    pub fn deactivate(&mut self) {
        self.data.set_interval(LoopResult::Deactivate);
    }
}

/// Data stored as part of a FlightLoop and used as a refcon
struct LoopData {
    /// The loop result, or None if the loop has not been scheduled
    loop_result: Option<LoopResult>,
    /// The loop ID
    loop_id: Option<xplm_sys::XPLMFlightLoopID>,
    /// The callback (stored here but not used)
    callback: Box<dyn FlightLoopCallback>,
}

impl fmt::Debug for LoopData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("LoopData")
            .field("loop_result", &self.loop_result)
            .field("loop_id", &self.loop_id)
            .field("callback", &String::from("[callback]"))
            .finish()
    }
}

impl LoopData {
    /// Creates a new LoopData with a callback
    pub fn new<C: FlightLoopCallback>(callback: C) -> Self {
        LoopData {
            loop_result: None,
            loop_id: None,
            callback: Box::new(callback),
        }
    }

    fn set_interval(&mut self, loop_result: LoopResult) {
        let loop_id = self.loop_id.expect("Loop ID not set");
        unsafe { xplm_sys::XPLMScheduleFlightLoop(loop_id, loop_result.clone().into(), 1) };
        self.loop_result = Some(loop_result);
    }
}

impl Drop for LoopData {
    fn drop(&mut self) {
        if let Some(loop_id) = self.loop_id {
            unsafe { xplm_sys::XPLMDestroyFlightLoop(loop_id) }
        }
    }
}

/// Trait for objects that can receive flight loop callbacks
pub trait FlightLoopCallback: 'static {
    /// Called periodically by X-Plane according to the provided scheduling
    ///
    /// In this callback, processing can be done. Drawing cannot be done.
    ///
    /// The provided LoopState can be used to get information and change the scheduling of
    /// callbacks. If the scheduling is not changed, this callback will continue to be called
    /// with the same schedule.
    fn flight_loop(&mut self, state: &mut LoopState);
}

/// Closures can be used as FlightLoopCallbacks
impl<F> FlightLoopCallback for F
where
    F: 'static + FnMut(&mut LoopState),
{
    fn flight_loop(&mut self, state: &mut LoopState) {
        self(state)
    }
}

/// Information available during a flight loop callback
///
/// By default, a flight loop callback will continue to be called on its initial schedule.
/// The scheduling functions only need to be called if the callback scheduling should change.
#[derive(Debug)]
pub struct LoopState<'a> {
    /// Time since last callback call
    since_call: Duration,
    /// Time since last flight loop
    since_loop: Duration,
    /// Callback counter
    counter: i32,
    /// The loop result
    result: &'a mut LoopResult,
}

impl<'a> LoopState<'a> {
    /// Returns the duration since the last time this callback was called
    pub fn since_last_call(&self) -> Duration {
        self.since_call
    }
    /// Returns the duration since the last flight loop
    ///
    /// If this callback is not called every flight loop, this may be different from the
    /// value returned from `time_since_last_call`.
    pub fn since_last_loop(&self) -> Duration {
        self.since_loop
    }
    /// Returns the value of a counter that increments every time the callback is called
    pub fn counter(&self) -> i32 {
        self.counter
    }
    /// Deactivates this flight loop. It will not be called again until it is scheduled.
    pub fn deactivate(&mut self) {
        *self.result = LoopResult::Deactivate;
    }
    /// Configures this callback to be called on the next flight loop
    pub fn call_next_loop(&mut self) {
        *self.result = LoopResult::Loops(1);
    }
    /// Configures this callback to be called after the specified number of loops
    pub fn call_after_loops(&mut self, loops: u32) {
        *self.result = LoopResult::Loops(loops);
    }
    /// Configures this callback to be called after the provided duration
    pub fn call_after(&mut self, time: Duration) {
        let seconds_f = (time.as_secs() as f32) + (1e-9_f32 * time.subsec_nanos() as f32);
        *self.result = LoopResult::Seconds(seconds_f);
    }
}

/// Loop results, which determine when the callback will be called next
#[derive(Debug, Clone)]
enum LoopResult {
    /// Callback will be called after the provided number of seconds
    Seconds(f32),
    /// Callback will be called after the provided number of loops
    Loops(u32),
    /// Callback will not be called again until it is rescheduled
    Deactivate,
}

/// Converts a LoopResult into an f32 suitable for returning from a flight loop callback
impl From<LoopResult> for f32 {
    fn from(lr: LoopResult) -> Self {
        match lr {
            LoopResult::Deactivate => 0f32,
            LoopResult::Seconds(secs) => secs,
            LoopResult::Loops(loops) => -1.0f32 * (loops as f32),
        }
    }
}

/// The flight loop callback that X-Plane calls
///
/// This expands to a separate callback for every type C.
unsafe extern "C" fn flight_loop_callback<C: FlightLoopCallback>(
    since_last_call: c_float,
    since_loop: c_float,
    counter: c_int,
    refcon: *mut c_void,
) -> c_float {
    // Get the loop data
    let loop_data = refcon as *mut LoopData;
    // Create a state
    let mut state = LoopState {
        since_call: secs_to_duration(since_last_call),
        since_loop: secs_to_duration(since_loop),
        counter,
        result: (*loop_data).loop_result.as_mut().unwrap(),
    };
    let callback_ptr: *mut dyn FlightLoopCallback = (*loop_data).callback.as_mut();
    let callback = callback_ptr as *mut C;
    (*callback).flight_loop(&mut state);

    // Return the next loop time
    f32::from(state.result.clone())
}

fn secs_to_duration(time: f32) -> Duration {
    let seconds = time.trunc() as u64;
    let nanoseconds = (time.fract() * 1e9_f32) as u32;
    Duration::new(seconds, nanoseconds)
}
