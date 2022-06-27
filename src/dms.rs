//! Angle representation in D°M'S" (sexagesimal)
//! with arithmetics for calculations involved in navigation.
use serde_derive::{Serialize, Deserialize};

/// `D°M'S"` represents one angle 
/// in Degrees D°, Minutes M' and fractionnal
/// Seconds S" (double precision)
#[derive(PartialEq, Copy, Clone, Debug)]
#[derive(Serialize, Deserialize)]
pub struct DMS {
    /// Degrees 0 <= D° < 360 
    pub degrees: u16,
    /// Minutes 0 <= M' < 60 
    pub minutes: u8,
    /// Seconds with fraction 0 <= S" < 60 
    pub seconds: f64,
}

pub enum Scale {
    /// Countries scale is 1°0'0"
    Country,
    /// Large cities scale is 0°6'0"
    LargeCity,
    /// Cities scale is 0°0'36"
    City,
    /// Neighborhood, Strees scale is 0°0'3.6" 
    Neighborhood,
    /// Single street / large buildings scale is 0°0'0.360"
    Street,
    /// Trees / small buildings scale is 0.036" 
    Tree,
    /// Human / single individual scale is 3.6E-3"
    Human,
    /// Roughly precise scale, used in commercial devices, is 360E-6"
    RoughSurveying,
    /// Extremely precise scale, used in tectnoic plate mapping for instance, is 36E-6"
    PreciseSurveying,
}

impl std::fmt::Display for DMS {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}°{}'{}\"", 
            self.degrees, 
            self.minutes, 
            self.seconds,
        )
    }
}

impl Default for DMS {
    /// Builds null coordinates
    fn default() -> DMS { 
        DMS {
            degrees: 0,
            minutes: 0,
            seconds: 0.0_f64,
        }
    }
}

impl Into<f64> for DMS {
    /// Converts Self into fractionnal seconds 
    fn into (self) -> f64 { 
        self.total_seconds() 
    }
}

impl Into<f32> for DMS {
    /// Converts Self into fractionnal seconds with precision loss 
    fn into (self) -> f32 { 
        self.total_seconds() as f32 
    }
}

impl Into<u64> for DMS {
    /// Converts Self into seconds base, loosing fractionnal part
    fn into (self) -> u64 { 
        self.total_seconds().floor() as u64 
    }
}

impl Into<u32> for DMS {
    /// Converts Self into seconds base, loosing fractionnal part
    fn into (self) -> u32 { 
        self.total_seconds().floor() as u32 
    }
}

impl Into<u16> for DMS {
    /// Converts Self into seconds base, loosing fractionnal part
    fn into (self) -> u16 { 
        self.total_seconds().floor() as u16 
    }
}

impl Into<u8> for DMS {
    /// Converts Self into seconds base, loosing fractionnal part
    fn into (self) -> u8 { 
        self.total_seconds().floor() as u8 
    }
}

impl std::ops::Add<DMS> for DMS {
    type Output = DMS;
    /// Adds `D°M'S"` coordinates together
    fn add (self, rhs: Self) -> Self {
        DMS::from_seconds(
            self.total_seconds() + rhs.total_seconds()
        )
    }
}

impl std::ops::Add<f32> for DMS {
    type Output = DMS;
    /// Adds `rhs` fractionnal seconds to Self
    fn add (self, rhs: f32) -> Self { 
        self + rhs as f64 
    }
}

impl std::ops::Add<f64> for DMS {
    type Output = DMS;
    /// Adds `rhs` fractionnal seconds to Self
    fn add (self, rhs: f64) -> Self { 
        DMS::from_seconds(self.total_seconds() + rhs)
    }
}

impl std::ops::Sub<DMS> for DMS {
    type Output = DMS;
    /// Substracts rhs `D°M'S"` to Self
    fn sub (self, rhs: Self) -> Self {
        DMS::from_seconds(self.total_seconds() - rhs.total_seconds()) 
    }
}

impl std::ops::Sub<f64> for DMS {
    type Output = DMS;
    /// Substracts `rhs` fractionnal seconds to self
    fn sub (self, rhs: f64) -> Self { 
        DMS::from_seconds(self.total_seconds() - rhs)
    }
}

impl std::ops::Sub<f32> for DMS {
    type Output = DMS;
    /// Substracts `rhs` fractionnal seconds to self
    fn sub (self, rhs: f32) -> Self { 
        DMS::from_seconds(self.total_seconds() - rhs as f64)
    }
}

/*
impl std::ops::Mul for DMS {
    type Output = DMS;
    fn mul (self, rhs: Self) -> Self {
        self.clone()
    }
}

impl std::ops::Mul<f64> for DMS {
    type Output = DMS;
    fn mul (self, rhs: i64) -> Self {
        let mut degrees = self.degrees as f64 * rhs; 
        let mut minutes = self.minutes as f64 * rhs; 
        let mut seconds = self.seconds as f64 * rhs;
        DMS {
            degrees: (degrees.floor() as i16)%360,
            minutes,
            seconds,
        }
    }
} */

impl DMS {
    /// Builds `D°M'S"` angle, from given D°, M', S" values.
    /// This method allows overflow, it will wrapp values to correct range
    /// itself.
    pub fn new (degrees: u16, minutes: u8, seconds: f64) -> DMS { 
        DMS::from_seconds(
            degrees as f64 * 3600.0 
            + minutes as f64 * 60.0
            + seconds)
    }

    /// Builds `D°M'S"` angle from total amount of seconds
    pub fn from_seconds (seconds: f64) -> DMS {
        let degrees = (seconds / 3600.0).floor();
        let minutes = ((seconds - degrees * 3600.0) /60.0).floor();
        let integer = ((seconds - degrees * 3600.0 - minutes*60.0).floor() as u8)%60;
        DMS {
            degrees: (degrees as u16)%360,
            minutes: minutes as u8,
            seconds: integer as f64 + seconds.fract(),
        }
    }
    
    /// Returns total of seconds (base unit) contained in Self
    pub fn total_seconds (&self) -> f64 {
        self.degrees as f64 * 3600.0
        + self.minutes as f64 * 60.0
            + self.seconds
    }
}
