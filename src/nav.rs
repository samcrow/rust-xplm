// Copyright (c) 2015 rust-xplm developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

//! Navigation database access
//!

use std::ptr;

use xplm_sys::navigation::*;
use position::{LatLonAlt, Positioned};
use frequency::Frequency;
use ffi::StringBuffer;

const INVALID_NAV: XPLMNavRef = -1;

/// Represents a non-directional beacon
#[derive(Debug, Clone)]
pub struct NDB {
    /// Position
    pub position: LatLonAlt,
    /// Frequency
    pub frequency: Frequency,
    /// Code
    pub code: String,
    /// Name
    pub name: String,
}

impl Positioned for NDB {
    fn position(&self) -> LatLonAlt {
        self.position.clone()
    }
}
/// Represents a VOR
#[derive(Debug, Clone)]
pub struct VOR {
    /// Position
    pub position: LatLonAlt,
    /// Frequency
    pub frequency: Frequency,
    /// Code
    pub code: String,
    /// Name
    pub name: String,
}

impl Positioned for VOR {
    fn position(&self) -> LatLonAlt {
        self.position.clone()
    }
}
/// Represents an airport
#[derive(Debug, Clone)]
pub struct Airport {
    /// Position
    pub position: LatLonAlt,
    /// Code
    pub code: String,
    /// Name
    pub name: String,
}

impl Positioned for Airport {
    fn position(&self) -> LatLonAlt {
        self.position.clone()
    }
}
/// Represents an ILS localizer (the glideslope is a separate Glideslope object)
#[derive(Debug, Clone)]
pub struct ILSLocalizer {
    /// Position
    pub position: LatLonAlt,
    /// Heading, true, degrees
    pub heading: f64,
    /// Frequency
    pub frequency: Frequency,
    /// Code
    pub code: String,
    /// Name
    pub name: String,
}

impl Positioned for ILSLocalizer {
    fn position(&self) -> LatLonAlt {
        self.position.clone()
    }
}
/// Represents a standalone localizer
#[derive(Debug, Clone)]
pub struct Localizer {
    /// Position
    pub position: LatLonAlt,
    /// Heading, true, degrees
    pub heading: f64,
    /// Frequency
    pub frequency: Frequency,
    /// Code
    pub code: String,
    /// Name
    pub name: String,
}

impl Positioned for Localizer {
    fn position(&self) -> LatLonAlt {
        self.position.clone()
    }
}
/// Represents an ILS glideslope
#[derive(Debug, Clone)]
pub struct Glideslope {
    /// Position
    pub position: LatLonAlt,
    /// Heading, true, degrees
    pub heading: f64,
    /// Frequency
    pub frequency: Frequency,
    /// Code
    pub code: String,
    /// Name
    pub name: String,
}

impl Positioned for Glideslope {
    fn position(&self) -> LatLonAlt {
        self.position.clone()
    }
}
/// Represents an outer marker
#[derive(Debug, Clone)]
pub struct OuterMarker {
    /// Position
    pub position: LatLonAlt,
    /// Code
    pub code: String,
    /// Name
    pub name: String,
}

impl Positioned for OuterMarker {
    fn position(&self) -> LatLonAlt {
        self.position.clone()
    }
}
/// Represents a middle marker
#[derive(Debug, Clone)]
pub struct MiddleMarker {
    /// Position
    pub position: LatLonAlt,
    /// Code
    pub code: String,
    /// Name
    pub name: String,
}

impl Positioned for MiddleMarker {
    fn position(&self) -> LatLonAlt {
        self.position.clone()
    }
}
/// Represents an inner marker
#[derive(Debug, Clone)]
pub struct InnerMarker {
    /// Position
    pub position: LatLonAlt,
    /// Code
    pub code: String,
    /// Name
    pub name: String,
}

impl Positioned for InnerMarker {
    fn position(&self) -> LatLonAlt {
        self.position.clone()
    }
}
/// Represents a fix
#[derive(Debug, Clone)]
pub struct Fix {
    /// Position
    pub position: LatLonAlt,
    /// Code
    pub code: String,
    /// Name
    pub name: String,
}

impl Positioned for Fix {
    fn position(&self) -> LatLonAlt {
        self.position.clone()
    }
}
/// Represents a DME
#[derive(Debug, Clone)]
pub struct DME {
    /// Position
    pub position: LatLonAlt,
    /// Frequency
    pub frequency: Frequency,
    /// Code
    pub code: String,
    /// Name
    pub name: String,
}

impl Positioned for DME {
    fn position(&self) -> LatLonAlt {
        self.position.clone()
    }
}

/// Contains a navaid of any of the supported types
#[derive(Debug, Clone)]
pub enum Navaid {
    Airport(Airport),
    NDB(NDB),
    VOR(VOR),
    ILSLocalizer(ILSLocalizer),
    Localizer(Localizer),
    Glideslope(Glideslope),
    OuterMarker(OuterMarker),
    MiddleMarker(MiddleMarker),
    InnerMarker(InnerMarker),
    Fix(Fix),
    DME(DME),
}

/// Returns an iterator over all available navaids in the database
pub fn all_navaids() -> NavaidIterator {
    NavaidIterator {
        next: unsafe { XPLMGetFirstNavAid() },
        type_filter: None,
    }
}

/// Returns an iterator over all airports
pub fn all_airports() -> NavaidIterator {
    all_navaids_of_type(XPLMNavType::xplm_Nav_Airport)
}
/// Returns an iterator over all NDBs
pub fn all_ndbs() -> NavaidIterator {
    all_navaids_of_type(XPLMNavType::xplm_Nav_NDB)
}
/// Returns an iterator over all VORs
pub fn all_vors() -> NavaidIterator {
    all_navaids_of_type(XPLMNavType::xplm_Nav_VOR)
}
/// Returns an iterator over all ILS glideslopes
pub fn all_ils_glideslopes() -> NavaidIterator {
    all_navaids_of_type(XPLMNavType::xplm_Nav_GlideSlope)
}
/// Returns an iterator over all ILS localizers
pub fn all_ils_localizers() -> NavaidIterator {
    all_navaids_of_type(XPLMNavType::xplm_Nav_ILS)
}
/// Returns an iterator over all standalone localizers
pub fn all_localizers() -> NavaidIterator {
    all_navaids_of_type(XPLMNavType::xplm_Nav_Localizer)
}
/// Returns an iterator over all outer markers
pub fn all_outer_markers() -> NavaidIterator {
    all_navaids_of_type(XPLMNavType::xplm_Nav_OuterMarker)
}
/// Returns an iterator over all middle markers
pub fn all_middle_markers() -> NavaidIterator {
    all_navaids_of_type(XPLMNavType::xplm_Nav_MiddleMarker)
}
/// Returns an iterator over all inner markers
pub fn all_inner_markers() -> NavaidIterator {
    all_navaids_of_type(XPLMNavType::xplm_Nav_InnerMarker)
}
/// Returns an iterator over all fixes
pub fn all_fixes() -> NavaidIterator {
    // Apparently, there is a bug in XPLMFindFirstNavAidOfType() with fixes:
    // http://www.xsquawkbox.net/xpsdk/mediawiki/XPLMFindFirstNavAidOfType
    all_navaids_of_type(XPLMNavType::xplm_Nav_Fix)
}
/// Returns an iterator over all DMEs
pub fn all_dmes() -> NavaidIterator {
    all_navaids_of_type(XPLMNavType::xplm_Nav_DME)
}

fn all_navaids_of_type(nav_type: XPLMNavType) -> NavaidIterator {
    NavaidIterator {
        next: unsafe { XPLMFindFirstNavAidOfType(nav_type) },
        type_filter: Some(nav_type),
    }
}

/// An iterator over available navaids
#[derive(Debug)]
pub struct NavaidIterator {
    /// The reference of the next navaid to provide, or -1 if no more are available
    next: XPLMNavRef,
    /// The navaid type to provide, or None to provide all available navaids
    type_filter: Option<XPLMNavType>,
}

impl Iterator for NavaidIterator {
    type Item = Navaid;
    fn next(&mut self) -> Option<Navaid> {
        if self.next == INVALID_NAV {
            None
        } else {
            // Get information on next
            let navaid_option = get_navaid_info(self.next);
            match navaid_option {
                Some((navaid, nav_type)) => {
                    let type_matches = match self.type_filter {
                        Some(allowed_type) => nav_type == allowed_type,
                        None => true,
                    };

                    if type_matches {
                        // Found
                        // Find next
                        self.next = unsafe { XPLMGetNextNavAid(self.next) };
                        Some(navaid)
                    } else {
                        // No more navaids
                        self.next = INVALID_NAV;
                        None
                    }
                }
                None => {
                    // No more navaids
                    self.next = INVALID_NAV;
                    None
                }
            }
        }
    }
}

/// Extracts information from an XPLMNavRef. If the returned navaid information
/// has a known type, returns the navaid and the type.
#[allow(non_upper_case_globals)]
fn get_navaid_info(nav_ref: XPLMNavRef) -> Option<(Navaid, XPLMNavType)> {
    let mut navaid_type: XPLMNavType = XPLMNavType::xplm_Nav_Unknown;
    let mut latitude = 0f32;
    let mut longitude = 0f32;
    let mut altitude = 0f32;
    let mut frequency = 0i32;
    let mut heading = 0f32;
    let mut code = StringBuffer::new(32);
    let mut name = StringBuffer::new(256);

    unsafe {
        XPLMGetNavAidInfo(nav_ref,
                          &mut navaid_type,
                          &mut latitude,
                          &mut longitude,
                          &mut altitude,
                          &mut frequency,
                          &mut heading,
                          code.as_mut_ptr(),
                          name.as_mut_ptr(),
                          ptr::null_mut());
    }

    let position = LatLonAlt {
        latitude: latitude as f64,
        longitude: longitude as f64,
        altitude: altitude as f64,
    };

    let navaid = match navaid_type {
        XPLMNavType::xplm_Nav_Airport => {
            Some(Navaid::Airport(Airport {
                position: position,
                code: code.as_string(),
                name: name.as_string(),
            }))
        }
        XPLMNavType::xplm_Nav_NDB => {
            Some(Navaid::NDB(NDB {
                position: position,
                frequency: Frequency::kilohertz(frequency as f32),
                code: code.as_string(),
                name: name.as_string(),
            }))
        }
        XPLMNavType::xplm_Nav_VOR => {
            Some(Navaid::VOR(VOR {
                position: position,
                frequency: Frequency::megahertz((frequency as f32) / 100.0),
                code: code.as_string(),
                name: name.as_string(),
            }))
        }
        XPLMNavType::xplm_Nav_ILS => {
            Some(Navaid::ILSLocalizer(ILSLocalizer {
                position: position,
                heading: heading as f64,
                frequency: Frequency::megahertz((frequency as f32) / 100.0),
                code: code.as_string(),
                name: name.as_string(),
            }))
        }
        XPLMNavType::xplm_Nav_Localizer => {
            Some(Navaid::Localizer(Localizer {
                position: position,
                heading: heading as f64,
                frequency: Frequency::megahertz((frequency as f32) / 100.0),
                code: code.as_string(),
                name: name.as_string(),
            }))
        }
        XPLMNavType::xplm_Nav_GlideSlope => {
            Some(Navaid::Glideslope(Glideslope {
                position: position,
                heading: heading as f64,
                frequency: Frequency::megahertz((frequency as f32) / 100.0),
                code: code.as_string(),
                name: name.as_string(),
            }))
        }
        XPLMNavType::xplm_Nav_OuterMarker => {
            Some(Navaid::OuterMarker(OuterMarker {
                position: position,
                code: code.as_string(),
                name: name.as_string(),
            }))
        }
        XPLMNavType::xplm_Nav_MiddleMarker => {
            Some(Navaid::MiddleMarker(MiddleMarker {
                position: position,
                code: code.as_string(),
                name: name.as_string(),
            }))
        }
        XPLMNavType::xplm_Nav_InnerMarker => {
            Some(Navaid::InnerMarker(InnerMarker {
                position: position,
                code: code.as_string(),
                name: name.as_string(),
            }))
        }
        XPLMNavType::xplm_Nav_Fix => {
            Some(Navaid::Fix(Fix {
                position: position,
                code: code.as_string(),
                name: name.as_string(),
            }))
        }
        XPLMNavType::xplm_Nav_DME => {
            Some(Navaid::DME(DME {
                position: position,
                frequency: Frequency::megahertz((frequency as f32) / 100.0),
                code: code.as_string(),
                name: name.as_string(),
            }))
        }
        _ => None,
    };
    navaid.map(|navaid| (navaid, navaid_type))
}
