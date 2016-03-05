// Copyright (c) 2015 rust-xplm developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

//! Allows scheduling flight loop callbacks
//!

use xplm_sys::processing::*;
use libc;

use std::mem;

/// Phases of execution in which a callback can run
#[derive(Debug,Clone)]
pub enum Phase {
    /// The callback is run before X-Plane runs the flight model
    BeforeFlightModel,
    /// The callback is run after X-Plane runs the flight model
    AfterFlightModel,
}

/// Information on when the next callback should run
#[derive(Debug,Clone)]
enum NextCallbackEnum {
    /// The callback will run after a number of flight loops
    AfterLoops(u32),
    /// The callback will run after a number of seconds
    AfterSeconds(f32),
    /// The callback will be suspended. It will not be called again until it is rescheduled.
    Suspend,
}

impl NextCallbackEnum {
    /// Converts this next callback into a float value in the format returned by an
    /// XPLMFlightLoop_f function
    pub fn as_float(&self) -> f32 {
        match self {
            &NextCallbackEnum::AfterLoops(loops) => -(loops as f32),
            &NextCallbackEnum::AfterSeconds(seconds) => seconds,
            &NextCallbackEnum::Suspend => 0f32,
        }
    }
}

/// Information on when the next callback should run
#[derive(Debug,Clone)]
pub struct NextCallback(NextCallbackEnum);

impl NextCallback {
    /// Requests the next callback after a number of flight loops.
    /// If the provided number of loops is zero, the next callback will be on the next flight loop.
    pub fn after_loops(loops: u32) -> NextCallback {
        if loops > 0 {
            NextCallback(NextCallbackEnum::AfterLoops(loops))
        } else {
            NextCallback(NextCallbackEnum::AfterLoops(1))
        }
    }
    /// Requests that the next callback be after a number of seconds.
    /// If the provided number of seconds is zero or negative, the next callback will be
    /// on the next flight loop.
    pub fn after_seconds(seconds: f32) -> NextCallback {
        if seconds > 0f32 {
            NextCallback(NextCallbackEnum::AfterSeconds(seconds))
        } else {
            NextCallback(NextCallbackEnum::AfterLoops(1))
        }
    }
    /// Requests that the callback be suspended. The callback will not be called again until it
    /// is rescheduled.
    pub fn suspend() -> NextCallback {
        NextCallback(NextCallbackEnum::Suspend)
    }
}

/// A trait for things that can receive flight loop callbacks
pub trait FlightLoopCallback {
    /// Called for a flight loop callback.
    ///
    /// Returns a request for the next callback.
    fn callback(&mut self) -> NextCallback;
}
/// FlightLoopCallback implementation for closures
impl<F> FlightLoopCallback for F
    where F: Fn() -> NextCallback
{
    fn callback(&mut self) -> NextCallback {
        self()
    }
}

/// A handle to something that calls a flight loop callback
///
/// A `FlightLoop` object stores a callback that will be called by X-Plane every time the flight
/// model is calculated.
///
/// When a `FlightLoop` object goes out of scope, the callback will be unregistered from X-Plane
/// and deleted.
///
/// # Examples
///
/// ## A flight loop callback executed every time the flight model is calculated
/// ```no_run
/// let callback = || {
///     println!("Flight loop callback running");
///     NextCallback::after_loops(1)
/// };
/// let flight_loop = FlightLoop::new(Phase::AfterFlightModel, callback);
/// flight_loop.schedule(NextCallback::AfterLoops(1));
/// ```
///
#[allow(missing_debug_implementations)]
pub struct FlightLoop {
    /// The ID of this loop
    id: XPLMFlightLoopID,
    /// The callback that will be called.
    /// This value is heap-allocated in a Box.
    /// On destruction, the callback is unregistered and the Box is recreated and dropped
    callback: *mut FlightLoopCallback,
}

impl FlightLoop {
    /// Creates a flight loop callback that will execute the given callback in the given
    /// phase.
    ///
    /// The callback will not be called until `schedule()` is called.
    pub fn new<C>(phase: Phase, callback: C) -> FlightLoop
        where C: 'static + FlightLoopCallback
    {

        let callback_box = Box::new(callback);
        let callback_ptr = Box::into_raw(callback_box);

        let mut params = XPLMCreateFlightLoop_t::default();
        params.structSize = mem::size_of::<XPLMCreateFlightLoop_t>() as i32;
        params.phase = match phase {
            Phase::BeforeFlightModel => 0,
            Phase::AfterFlightModel => 1,
        };
        params.callbackFunc = Some(global_callback::<C>);
        params.refcon = callback_ptr as *mut libc::c_void;

        let loop_id = unsafe { XPLMCreateFlightLoop(&mut params) };

        FlightLoop {
            id: loop_id,
            callback: callback_ptr,
        }
    }

    /// Schedules the callback to be executed
    pub fn schedule(&self, time: NextCallback) {
        unsafe { XPLMScheduleFlightLoop(self.id, time.0.as_float(), 1) };
    }
}

impl Drop for FlightLoop {
    /// Destroys the flight loop and the callback
    fn drop(&mut self) {
        unsafe { XPLMDestroyFlightLoop(self.id) };
        let callback_box = unsafe { Box::from_raw(self.callback) };
        drop(callback_box);
    }
}

/// The global flight loop callback
unsafe extern "C" fn global_callback<C>(_: ::libc::c_float,
                                        _: ::libc::c_float,
                                        _: ::libc::c_int,
                                        refcon: *mut ::libc::c_void)
                                        -> ::libc::c_float
    where C: FlightLoopCallback
{
    let callback = refcon as *mut C;
    let next = (*callback).callback();
    next.0.as_float()
}
