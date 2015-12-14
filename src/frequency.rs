//!
//! Radio frequency representation
//!
use std::ops::{Add, Sub, Neg};

type Hertz = i64;

/// Stores a radio frequency.
///
/// Frequencies can be positive or negative.
#[derive(Debug,Clone)]
pub struct Frequency {
    /// The frequency in hertz
    hertz: Hertz,
}

impl Frequency {
    /// Creates a frequency from a number of hertz
    pub fn hertz(hertz: i64) -> Frequency {
        Frequency { hertz: hertz }
    }
    /// Creates a frequency from a number of kilohertz
    pub fn kilohertz(kilohertz: f32) -> Frequency {
        Frequency { hertz: (kilohertz / 1E3) as Hertz }
    }
    /// Creates a frequency from a number of Megahertz
    pub fn megahertz(megahertz: f32) -> Frequency {
        Frequency { hertz: (megahertz / 1E6) as Hertz }
    }
    /// Creates a frequency from a number of Gigahertz
    pub fn gigahertz(gigahertz: f32) -> Frequency {
        Frequency { hertz: (gigahertz / 1E9) as Hertz }
    }

    /// Returns this frequency as a number of hertz
    pub fn as_hertz(&self) -> i64 {
        self.hertz
    }
    /// Returns this frequency as a number of kilohertz
    pub fn as_kilohertz(&self) -> f32 {
        self.hertz as f32 / 1E3
    }
    /// Returns this frequency as a number of Megahertz
    pub fn as_megahertz(&self) -> f32 {
        self.hertz as f32 / 1E6
    }
    /// Returns this frequency as a number of gigahertz
    pub fn as_gigahertz(&self) -> f32 {
        self.hertz as f32 / 1E9
    }
}

impl<'a> Add for &'a Frequency {
    type Output = Frequency;
    fn add(self, other: &'a Frequency) -> Frequency {
        Frequency::hertz(self.hertz + other.hertz)
    }
}

impl Add for Frequency {
    type Output = Frequency;
    fn add(self, other: Frequency) -> Frequency {
        Frequency::hertz(self.hertz + other.hertz)
    }
}

impl<'a> Sub for &'a Frequency {
    type Output = Frequency;
    fn sub(self, other: &'a Frequency) -> Frequency {
        Frequency::hertz(self.hertz - other.hertz)
    }
}

impl Sub for Frequency {
    type Output = Frequency;
    fn sub(self, other: Frequency) -> Frequency {
        Frequency::hertz(self.hertz - other.hertz)
    }
}

impl<'a> Neg for &'a Frequency {
    type Output = Frequency;
    fn neg(self) -> Frequency {
        Frequency::hertz(-self.hertz)
    }
}

impl Neg for Frequency {
    type Output = Frequency;
    fn neg(self) -> Frequency {
        Frequency::hertz(-self.hertz)
    }
}
