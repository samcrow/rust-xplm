// Copyright (c) 2015 rust-xplm developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

//! Provides a callback that runs after all plugins have initialized

use flight_loop::{FlightLoopCallback, FlightLoop, NextCallback, Phase};

/// Trait for objects that can receive a deferred init callback
pub trait DeferredInitCallback {
    /// Called during deferred initialization
    fn deferred_init(&mut self);
}

impl<F> DeferredInitCallback for F where F: FnMut() {
    fn deferred_init(&mut self) {
        self()
    }
}

/// Wraps a DeferredInitCallback and implements FlightLoopCallback
struct DeferredCallbackAdapter<C> {
    callback: C,
}

impl<C> FlightLoopCallback for DeferredCallbackAdapter<C> where C: DeferredInitCallback {
    fn callback(&mut self) -> NextCallback {
        self.callback.deferred_init();
        NextCallback::suspend()
    }
}

/// Sets up a deferred initialization callback
///
/// If an instance is destroyed before the deferred init callback is called, the callback will
/// never be called.
#[allow(missing_debug_implementations)]
pub struct DeferredInit {
    /// The flight loop holder
    _flight_loop: FlightLoop,
}

impl DeferredInit {
    /// Creates a new deferred init holder. The provided callback will be called.
    pub fn new<C>(callback: C) -> DeferredInit where C: DeferredInitCallback + 'static {
        let adapter = DeferredCallbackAdapter { callback: callback };
        let flight_loop = FlightLoop::new(Phase::BeforeFlightModel, adapter);
        flight_loop.schedule(NextCallback::after_loops(1));
        DeferredInit { _flight_loop: flight_loop }
    }
}
