// Copyright (c) 2015 rust-xplm developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

//! Provides access to scenery through terrain probes
//!

use xplm_sys::scenery::*;
use std::mem;
use position::{Vec3, Local, LatLon, LatLonAlt, world_to_local, local_to_world};

/// The data returned from a terrain probe
#[derive(Debug,Clone)]
pub struct ProbeResult {
    /// Position of the terrain
    pub position: Local,
    /// Normal vector of the terrain
    pub normal: Vec3,
    /// Velocity of the terrain, in meters per second
    pub velocity: Vec3,
    /// If the terrain is water
    pub is_water: bool,
}

/// A terrain probe
///
///
#[derive(Debug)]
pub struct Probe {
    /// The probe reference
    probe: XPLMProbeRef,
}

impl Probe {
    /// Creates a new terrain probe
    pub fn new() -> Probe {
        Probe { probe: unsafe { XPLMCreateProbe(xplm_ProbeY as i32) } }
    }

    /// Probes terain at the specified location in local coordinates
    #[allow(non_upper_case_globals)]
    pub fn probe(&self, position: &Local) -> Option<ProbeResult> {
        let mut result = XPLMProbeInfo_t::default();
        result.structSize = mem::size_of::<XPLMProbeInfo_t>() as i32;
        let status = unsafe {
            XPLMProbeTerrainXYZ(self.probe,
                                position.x as f32,
                                position.y as f32,
                                position.z as f32,
                                &mut result)
        };
        match status as u32 {
            xplm_ProbeHitTerrain => Some(convert_result(&result)),
            _ => None,
        }
    }

    /// Probes terrain at the specified latitude and longitude.
    ///
    /// On success, returns a LatLonAlt with the provided latitude/longitude
    /// and the altitude of the provided location.
    pub fn probe_altitude(&self, position: &LatLon) -> Option<LatLonAlt> {
        let position_lla = LatLonAlt::with_altitude(position, 0.0);
        let local = world_to_local(&position_lla);
        match self.probe(&local) {
            Some(result) => {
                let probed_altitude = local_to_world(&result.position).altitude;
                Some(LatLonAlt::with_altitude(position, probed_altitude))
            }
            None => None,
        }
    }
}

impl Drop for Probe {
    fn drop(&mut self) {
        unsafe { XPLMDestroyProbe(self.probe) };
    }
}

/// Converts an XPLMProbeInfo_t into a ProbeResult
fn convert_result(xplm_result: &XPLMProbeInfo_t) -> ProbeResult {
    ProbeResult {
        position: Local {
            x: xplm_result.locationX as f64,
            y: xplm_result.locationY as f64,
            z: xplm_result.locationZ as f64,
        },
        normal: Vec3 {
            x: xplm_result.normalX as f64,
            y: xplm_result.normalY as f64,
            z: xplm_result.normalZ as f64,
        },
        velocity: Vec3 {
            x: xplm_result.velocityX as f64,
            y: xplm_result.velocityY as f64,
            z: xplm_result.velocityZ as f64,
        },
        is_water: 1 == xplm_result.is_wet,
    }
}
