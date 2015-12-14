//!
//! Types that represent positions in X-Plane
//!

use data::ReadOnly;
use dataref::DataRef;
use xplm_sys::graphics::*;

use std::convert::From;

/// A generic 3-dimensional vector
///
/// This struct uses the same axes as `Local`, but its origin, units,
/// and meaning are context-dependent.
pub struct Vec3 {
    /// X coordinate, East-West. East is positive.
    pub x: f64,
    /// Y coordinate, up-down. Up is positive.
    pub y: f64,
    /// Z coordinate, North-South. South is positive.
    pub z: f64,
}

impl Vec3 {
    /// Returns a (0, 0, 0) vector
    pub fn origin() -> Vec3 {
        Vec3 { x: 0.0, y: 0.0, z: 0.0 }
    }
}

/// A position in local OpenGL coordinates
///
/// Local coordinates are in meters. The origin is on the surface of the planet
/// at some position, which can be accessed with the function `local_origin()`.
///
/// Local coordinates are documented in more detail at
/// http://www.xsquawkbox.net/xpsdk/mediawiki/Graphics#Coordinates_in_X-Plane .
///
#[derive(Debug,Clone)]
pub struct Local {
    /// X coordinate, East-West. East is positive.
    pub x: f64,
    /// Y coordinate, up-down. Up is positive.
    pub y: f64,
    /// Z coordinate, North-South. South is positive.
    pub z: f64,
}

impl Local {
    /// Returns the local coordinates of the origin point (0, 0, 0)
    pub fn origin() -> Local {
        Local { x: 0.0, y: 0.0, z: 0.0 }
    }
}

/// Stores a position as latitude and longitude
#[derive(Debug,Clone)]
pub struct LatLon {
    /// Latitude in degrees
    pub latitude: f64,
    /// Longitude in degrees
    pub longitude: f64,
}

/// Stores a position as latitude, longitude, and altitude
#[derive(Debug,Clone)]
pub struct LatLonAlt {
    /// Latitude in degrees
    pub latitude: f64,
    /// Longitude in degrees
    pub longitude: f64,
    /// Altitude in meters above mean sea level
    pub altitude: f64,
}

impl LatLonAlt {
    /// Creates a LatLonAlt with the latitude and longitude from a provided LatLon
    /// and with the specified altitude
    pub fn with_altitude(ll: &LatLon, altitude: f64) -> LatLonAlt {
        LatLonAlt {
            latitude: ll.latitude,
            longitude: ll.longitude,
            altitude: altitude,
        }
    }
}

impl From<LatLonAlt> for LatLon {
    /// Converts a LatLonAlt into a LatLon, discarding the altitude information
    fn from(lla: LatLonAlt) -> LatLon {
        LatLon {
            latitude: lla.latitude,
            longitude: lla.longitude,
        }
    }
}

/// A trait for things that have positions
pub trait Positioned {
    /// Returns the position of this item
    fn position(&self) -> LatLonAlt;
    /// Returns the position of this item in local coordinates
    fn local_position(&self) -> Local {
        world_to_local(&self.position())
    }
}

/// Origin latitude dataref
static mut origin_latitude: Option<DataRef<f32, ReadOnly>> = None;
/// Origin longitude dataref
static mut origin_longitude: Option<DataRef<f32, ReadOnly>> = None;

/// Returns the latitude and longitude of the origin of the local coordinate
/// system
pub fn local_origin() -> LatLon {
    unsafe {
        if origin_latitude.is_none() {
            origin_latitude = Some(
                DataRef::find("sim/flightmodel/position/lat_ref").unwrap());
        }
        if origin_longitude.is_none() {
            origin_longitude = Some(
                DataRef::find("sim/flightmodel/position/lon_ref").unwrap());
        }

        match (&origin_latitude, &origin_longitude) {
            (&Some(ref lat), &Some(ref lon)) => LatLon {
                latitude: lat.get() as f64,
                longitude: lon.get() as f64,
            },
            _ => unreachable!(),
        }
    }
}

/// Converts from latitude, longitude, and altitude into local coordinates
pub fn world_to_local(world: &LatLonAlt) -> Local {
    let mut local = Local::origin();
    unsafe {
        XPLMWorldToLocal(world.latitude, world.longitude, world.altitude,
            &mut local.x, &mut local.y, &mut local.z);
    }
    local
}

/// Converts from local coordinates into latitude, longitude, and altitude
///
/// Because world coordinates are less precise than local coordinates, converting a position
/// to world and then back to local may cause a loss of precision.
pub fn local_to_world(local: &Local) -> LatLonAlt {
    let mut world = LatLonAlt { latitude: 0.0, longitude: 0.0, altitude: 0.0 };
    unsafe {
        XPLMLocalToWorld(local.x, local.y, local.z,
            &mut world.latitude, &mut world.longitude, &mut world.altitude);
    }
    world
}
