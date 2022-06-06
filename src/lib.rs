//! Package to manipulate D°M'S'' coordinates
//! mainly in navigation applications.
//!
//! Homepage: <https://github.com/gwbres/dms-coordinates>
use thiserror::Error;
use serde_derive::{Serialize, Deserialize};

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
#[derive(Serialize, Deserialize)]
pub enum Bearing {
    North,
    NorthEast,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
    East,
}

impl std::fmt::Display for Bearing {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Bearing::North => write!(f, "N"),
            Bearing::NorthEast => write!(f, "NE"),
            Bearing::NorthWest => write!(f, "NW"),
            Bearing::South => write!(f, "S"),
            Bearing::SouthEast => write!(f, "SE"),
            Bearing::SouthWest => write!(f, "SW"),
            Bearing::East => write!(f, "E"),
            Bearing::West => write!(f, "W"),
        }
    }
}

impl Bearing {
    pub fn is_northern (&self) -> bool {
        match self {
            Bearing::North | Bearing::NorthEast | Bearing::NorthWest => true,
            _ => false,
        }
    }
    pub fn is_southern (&self) -> bool {
        match self {
            Bearing::South | Bearing::SouthEast | Bearing::SouthWest => true,
            _ => false,
        }
    }
    pub fn is_eastern (&self) -> bool {
        match self {
            Bearing::East | Bearing::NorthEast | Bearing::SouthEast => true,
            _ => false,
        }
    }
    pub fn is_western (&self) -> bool {
        match self {
            Bearing::West | Bearing::NorthWest | Bearing::SouthWest => true,
            _ => false,
        }
    }
}

const R : f64 = initial_conditions::EARTH_RADIUS; // [m]

/// Returns distance (m) between two decimal degrees coordinates
/// coord1: (lat,lon), coord2: (lat, lon)
pub fn projected_distance (coord1: (f64,f64), coord2: (f64,f64)) -> f64 {
    let dphi = map_3d::deg2rad(coord2.0) - map_3d::deg2rad(coord1.0);
    let d_lambda = map_3d::deg2rad(coord2.1) - map_3d::deg2rad(coord1.1);
    let a: f64 = (dphi / 2.0_f64).sin().powf(2.0_f64)
        + map_3d::deg2rad(coord1.0).cos() * map_3d::deg2rad(coord2.0).cos()
            * (d_lambda/2.0_f64).sin().powf(2.0_f64);
    let c = 2.0_f64 * a.powf(0.5_f64).atan2((1.0-a).powf(0.5_f64));
    R * c
}

/// `DMS` Structure to manipulate
/// describes an angle ranging from 0° to 90°
/// and an associated bearing
#[derive(PartialEq, Clone, Debug)]
#[derive(Serialize, Deserialize)]
pub struct DMS {
    pub degrees: i32,
    pub minutes: i32,
    pub seconds: f64,
    pub bearing: Bearing,
}

impl std::fmt::Display for DMS {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}°{}'{}''{}", 
            self.degrees, 
            self.minutes, 
            self.seconds,
            self.bearing)
    }
}

impl Default for DMS {
    fn default() -> DMS { DMS::from_decimal_degrees(0.0_f64, false) }
}

impl Into<f32> for DMS {
    fn into (self) -> f32 {
        self.to_decimal_degrees() as f32
    }
}

impl Into<f64> for DMS {
    fn into (self) -> f64 {
        self.to_decimal_degrees()
    }
}

impl std::ops::Add for DMS {
    type Output = DMS;
    fn add (self, rhs: Self) -> Self {
        let (d0,m0,s0) = self.to_azimuth();
        let (d1,m1,s1) = rhs.to_azimuth();
        let mut degrees = d0 + d1; 
        let mut minutes = m0 + m1; 
        let mut seconds = s0 + s1;
        if seconds > 60.0 {
            minutes += 1;
            seconds -= 60.0;
        }
        if minutes > 60 {
            degrees += 1;
            minutes -= 60
        }
        DMS::from_azimuth((degrees,minutes,seconds))
    }
}

impl std::ops::Sub for DMS {
    type Output = DMS;
    fn sub (self, rhs: Self) -> Self {
        let (d0,m0,s0) = self.to_azimuth();
        let (d1,m1,s1) = rhs.to_azimuth();
        let mut degrees = d0 - d1; 
        let mut minutes = m0 - m1; 
        let mut seconds = s0 - s1;
        if degrees < 0 {
            degrees = 360 - degrees;
            minutes -= 1;
        }
        if minutes < 0 {
            minutes = 60 - minutes;
            seconds -= 1.0;
        }
        DMS::from_azimuth((degrees,minutes,seconds))
    }
}

impl DMS {
    /// Builds a `D°M'S''` structure 
    pub fn new (degrees: i32, minutes: i32, seconds: f64, bearing: Bearing) -> DMS {
        DMS {
            degrees, 
            minutes, 
            seconds, 
            bearing,
        }
    }

    /// Buils a `D°M'S''` structure from given decimal coordinates. 
    /// Set `is_latitude` to `true` if this describes a latitude,
    /// otherwise longitude is assumed.
    pub fn from_decimal_degrees (ddeg: f64, is_latitude: bool) -> DMS {
        let d = ddeg.abs().trunc() as i32;
        let m = ((ddeg.abs() - d as f64) * 60.0).trunc() as i32;
        let s = (ddeg.abs() - d as f64 - (m as f64)/60.0_f64) * 3600.0_f64;
        let bearing = match is_latitude {
            true => {
                if ddeg < 0.0 {
                    Bearing::South
                } else {
                    Bearing::North
                }
            },
            false => {
                if ddeg < 0.0 {
                    Bearing::West
                } else {
                    Bearing::East
                }
            },
        };
        DMS {
            degrees: d,  
            minutes: m, 
            seconds: s,
            bearing,
        }
    }

    /// Converts Self to `Decimal Degrees` 
    pub fn to_decimal_degrees (&self) -> f64 {
        let ddeg: f64 = self.degrees as f64
            + self.minutes as f64 / 60.0_f64
            + self.seconds as f64 / 3600.0_f64;
        if self.bearing.is_southern() || self.bearing.is_western() {
            -ddeg
        } else {
            ddeg
        }
    }

    // Builds D°M'S'' structure from given Azimuth (in D° [0:360],M',S'')
    // by deducing appropriate angle & bearing
    pub fn from_azimuth (azimuth: (i32,i32,f64)) -> DMS {
        let degrees = azimuth.0;
        let minutes = azimuth.1;
        let seconds = azimuth.2;
        if degrees < 90 {
            DMS {
                degrees,
                minutes,
                seconds,
                bearing: Bearing::NorthEast,
            }
        } else if degrees < 180 {
            DMS {
                degrees: 180 - degrees,
                minutes,
                seconds,
                bearing: Bearing::SouthEast,
            }
        } else if degrees < 270 {
            DMS {
                degrees: degrees - 180,
                minutes,
                seconds,
                bearing: Bearing::SouthWest,
            }
        } else {
            DMS {
                degrees: 360 - degrees,
                minutes,
                seconds,
                bearing: Bearing::NorthWest,
            }
        }
    }
    
    // Converts Self to azimuth angle (D°[0:360],M',S''),
    // returns that angle in (degree,minutes,seconds) form
    pub fn to_azimuth (self) -> (i32,i32,f64) {
        let dms: DMS = match self.bearing {
            Bearing::SouthEast => DMS::from_azimuth((180,0,0.0)) - self,
            Bearing::SouthWest => DMS::from_azimuth((180,0,0.0)) + self,
            Bearing::NorthWest => DMS::from_azimuth((360,0,0.0)) - self,
            _ => self,
        };
        (dms.degrees,dms.minutes,dms.seconds)
    }
}

/// `3D D°M'S''` coordinates   
/// (latitude, longitude, optionnal altitude)
#[derive(PartialEq, Clone, Debug)]
#[derive(Serialize, Deserialize)]
pub struct DMS3d {
   pub latitude: DMS,
   pub longitude: DMS,
   pub altitude: Option<f64>,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to open file")]
    IoError(#[from] std::io::Error),
    #[error("gpx parsing error")]
    GpxParsingError,
    #[error("failed to wr file")]
    GpxWritingError(#[from] gpx::errors::GpxError),
}

impl std::fmt::Display for DMS3d {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "lat: \"{}\"  lon: \"{}\" alt: \"{}\"", 
            self.latitude.to_decimal_degrees(), 
            self.longitude.to_decimal_degrees(), 
            self.altitude.unwrap_or(0.0_f64))
    }
}

impl Default for DMS3d {
    fn default() -> Self {
        Self {
            latitude: DMS::from_decimal_degrees(0.0_f64, true), 
            longitude: DMS::from_decimal_degrees(0.0_f64, false), 
            altitude: None
        }
    }
}

impl DMS3d {
    /// Builds new `3D D°M'S''`  coordinates
    pub fn new (latitude: DMS, longitude: DMS, altitude: Option<f64>) -> DMS3d {
        DMS3d {
            latitude: latitude,
            longitude: longitude,
            altitude: altitude,
        }
    }
    /// Builds a `3D D°M'S''` from given coordinates in decimal degrees
    pub fn from_decimal_degrees (lat: f64, lon: f64, altitude: Option<f64>) -> DMS3d {
        DMS3d {
            latitude: DMS::from_decimal_degrees(lat, true),
            longitude: DMS::from_decimal_degrees(lon, false),
            altitude: altitude
        }
    }

    /// Builds 3D D°M'S'' object from given Cartesian coordinates
    pub fn from_cartesian (xyz: rust_3d::Point3D) -> DMS3d {
        DMS3d {
            latitude: DMS::from_decimal_degrees(map_3d::rad2deg((xyz.z / R).asin()), true),
            longitude: DMS::from_decimal_degrees(map_3d::rad2deg(xyz.y.atan2(xyz.x)), false),
            altitude: Some(xyz.z),
        }
    }

    /// Returns distance [m] between Self and given coordinates
    pub fn distance (&self, other: DMS3d) -> f64 {
        projected_distance(
            (self.latitude.to_decimal_degrees(),self.longitude.to_decimal_degrees()),
            (other.latitude.to_decimal_degrees(),other.longitude.to_decimal_degrees())
        )
    }

    /// Returns azimuth (angle where 0 <= angle < 360), 
    /// between Self & given point.
    /// Azimuth is the angle between North Pole & target
    pub fn azimuth (&self, rhs: Self) -> f64 {
        let (phi1, phi2) = (self.latitude.to_decimal_degrees(),
            rhs.latitude.to_decimal_degrees());
        let (lambda1, lambda2) = (self.longitude.to_decimal_degrees(),
            rhs.longitude.to_decimal_degrees());
        let dlambda = lambda2 - lambda1;
        let y = dlambda.sin() * phi2.cos();
        let x = phi1.cos() * phi2.sin() - phi1.sin() * phi2.cos() * dlambda.cos();
        map_3d::rad2deg(y.atan2(x))
    }

    // Converts Self to Cartesian Coordinates (x,y,z)
    // where x=0,y=0,z=0 is Earth Center.
    pub fn to_cartesian (&self) -> rust_3d::Point3D {
        let (lat, lon) = (
            self.latitude.to_decimal_degrees(),
            self.longitude.to_decimal_degrees());
        rust_3d::Point3D {
            x: R * lat.cos() * lon.cos(),
            y: R * lat.cos() * lon.sin(),
            z: R * lat.sin(),
        }
    }
    
    /// Writes self into .gpx file
    pub fn to_gpx (&self, fp: &str) -> Result<(), gpx::errors::GpxError> {
        let mut gpx : gpx::Gpx = Default::default();
        gpx.version = gpx::GpxVersion::Gpx11;
        let mut wpt = gpx::Waypoint::new(
            geo_types::Point::new(
                self.latitude.to_decimal_degrees(), 
                self.longitude.to_decimal_degrees()));
        wpt.elevation = self.altitude;
        gpx.waypoints.push(wpt);
        gpx::write(&gpx, std::fs::File::create(fp).unwrap())
    }

    /// Builds a 3D D°M'S'' object from a .gpx file 
    pub fn from_gpx (fp: &str) -> Result<Option<DMS3d>, Error> {
        let fd = std::fs::File::open(fp)?;
        let content: Result<gpx::Gpx, gpx::errors::GpxError> = gpx::read(fd);
        match content {
            Ok(mut gpx) => {
                if let Some(wpt) = gpx.waypoints.pop() {
                    Ok(Some(DMS3d::from_decimal_degrees(
                        wpt.point().x(),
                        wpt.point().y(),
                    wpt.elevation))
                )
                } else {
                    Ok(None)
                }
            },
            Err(_) => Err(Error::GpxParsingError)
        }
    }
}

impl std::ops::Add for DMS3d {
    type Output = DMS3d;
    fn add (self, rhs: Self) -> Self {
        let altitude : Option<f64> = match self.altitude {
            Some(altitude) => {
                match rhs.altitude {
                    Some(a) => Some(altitude + a),
                    None => Some(altitude),
                }
            },
            None => {
                match rhs.altitude {
                    Some(a) => Some(a),
                    None => None, 
                }
            },
        };
        DMS3d { 
            latitude : self.latitude + rhs.latitude,
            longitude: self.longitude + rhs.longitude, 
            altitude: altitude, 
        }
    }
}

impl std::ops::Sub for DMS3d {
    type Output = DMS3d;
    fn sub (self, rhs: Self) -> Self {
        let altitude : Option<f64> = match self.altitude {
            Some(altitude) => {
                match rhs.altitude {
                    Some(a) => Some(altitude - a),
                    None => Some(altitude),
                }
            },
            None => {
                match rhs.altitude {
                    Some(a) => Some(-a),
                    None => None, 
                }
            },
        };
        DMS3d { 
            latitude : self.latitude - rhs.latitude,
            longitude: self.longitude - rhs.longitude, 
            altitude: altitude,
        }
    }
}

impl From<rust_3d::Point3D> for DMS3d {
    fn from (item: rust_3d::Point3D) -> Self {
        Self::from_cartesian(item)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_northern() {
        assert_eq!(Bearing::North.is_northern(), true);
        assert_eq!(Bearing::NorthEast.is_northern(), true);
        assert_eq!(Bearing::NorthWest.is_northern(), true);
        assert_eq!(Bearing::South.is_northern(), false);
        assert_eq!(Bearing::SouthEast.is_northern(), false);
        assert_eq!(Bearing::SouthWest.is_northern(), false);
        assert_eq!(Bearing::East.is_northern(), false);
        assert_eq!(Bearing::West.is_northern(), false);
    }
    #[test]
    fn test_is_southern() {
        assert_eq!(Bearing::North.is_southern(), false);
        assert_eq!(Bearing::NorthEast.is_southern(), false);
        assert_eq!(Bearing::NorthWest.is_southern(), false);
        assert_eq!(Bearing::South.is_southern(), true);
        assert_eq!(Bearing::SouthEast.is_southern(), true);
        assert_eq!(Bearing::SouthWest.is_southern(), true);
        assert_eq!(Bearing::East.is_southern(), false);
        assert_eq!(Bearing::West.is_southern(), false);
    }
    #[test]
    fn test_is_eastern() {
        assert_eq!(Bearing::North.is_eastern(), false);
        assert_eq!(Bearing::NorthEast.is_eastern(), true);
        assert_eq!(Bearing::NorthWest.is_eastern(), false);
        assert_eq!(Bearing::South.is_eastern(), false);
        assert_eq!(Bearing::SouthEast.is_eastern(), true);
        assert_eq!(Bearing::SouthWest.is_eastern(), false);
        assert_eq!(Bearing::East.is_eastern(), true);
        assert_eq!(Bearing::West.is_eastern(), false);
    }
    #[test]
    fn test_is_western() {
        assert_eq!(Bearing::North.is_western(), false);
        assert_eq!(Bearing::NorthEast.is_western(), false);
        assert_eq!(Bearing::NorthWest.is_western(), true);
        assert_eq!(Bearing::South.is_western(), false);
        assert_eq!(Bearing::SouthEast.is_western(), false);
        assert_eq!(Bearing::SouthWest.is_western(), true);
        assert_eq!(Bearing::East.is_western(), false);
        assert_eq!(Bearing::West.is_western(), true);
    }
    #[test]
    fn test_dms_to_ddeg_conversion() {
        let dms = DMS::new(40, 43, 50.196_f64, Bearing::North); // NY (lat)
        let lat = dms.to_decimal_degrees();
        let expected_lat = 40.730; // NY
        assert!((lat - expected_lat).abs() < 1E-3);
        let dms = DMS::new(33, 51, 45.36_f64, Bearing::North); // SYDNEY (lat)
        let lat = dms.to_decimal_degrees();
        let expected_lat = -33.867; // SYDNEY 
        assert!((lat - expected_lat).abs() < 1E-2)
    }
    
    #[test]
    fn test_dms_from_ddeg() {
        let dms = DMS::from_decimal_degrees(-73.935242_f64, false); // NY (lon) 
        let secs = 6.8712_f64; // NY
        assert_eq!(dms.degrees, 73); // NY
        assert_eq!(dms.minutes, 56); // NY
        assert_eq!(dms.bearing, Bearing::West);
        assert!((dms.seconds - secs).abs() < 1E-3);
        let dms = DMS::from_decimal_degrees(151.209900_f64, false); // SYDNEY (lon) 
        let secs = 35.64_f64; // SYDNEY
        assert_eq!(dms.degrees, 151); // SYDNEY
        assert_eq!(dms.minutes, 12); // SYDNEY
        assert_eq!(dms.bearing, Bearing::East);
        assert!((dms.seconds - secs).abs() < 1E-3);
        let dms = DMS::from_decimal_degrees(-34.603722, true); // Buenos Aires (lon) 
        let secs = 13.3992_f64; // Buenos Aires 
        assert_eq!(dms.degrees, 34); 
        assert_eq!(dms.minutes, 36); 
        assert_eq!(dms.bearing, Bearing::South);
        assert!((dms.seconds - secs).abs() < 1E-3)
    }

    #[test]
    fn test_from_azimuth() {
        assert_eq!(
            DMS::from_azimuth((135,0,0.0)),
            DMS {
                degrees: 45,
                minutes: 0,
                seconds: 0.0,
                bearing: Bearing::SouthEast,
            });
        assert_eq!(
            DMS::from_azimuth((270,0,0.0)),
            DMS {
                degrees: 90,
                minutes: 0,
                seconds: 0.0,
                bearing: Bearing::NorthWest,
            });
    }

    #[test]
    fn test_3ddms_from_ddeg() {
        let dms = DMS3d::from_decimal_degrees(
            40.730610_f64, // NY
            -73.935242_f64, // NY
            Some(10.0)
        );
        assert_eq!(dms.latitude.degrees, 40); // NY
        assert_eq!(dms.latitude.minutes, 43); // NY
        assert_eq!(dms.latitude.bearing, Bearing::North);
        assert!((dms.latitude.seconds - 50.1960).abs() < 1E-3);
        assert_eq!(dms.longitude.degrees, 73); // NY
        assert_eq!(dms.longitude.minutes, 56); // NY
        assert_eq!(dms.longitude.bearing, Bearing::West);
        assert!((dms.longitude.seconds - 6.8712).abs() < 1E-3);
    }

    #[test]
    fn test_distance() {
        let dms1 = DMS3d::from_decimal_degrees( // NY
            40.730610_f64,
            -73.935242_f64,
            Some(10.0)
        );
        let dms2 = DMS3d::from_decimal_degrees( // Paris
            48.856614, 
            2.3522219,
            Some(10.0)
        );
        let expected_km = 5831.0_f64; 
        let d_km = dms1.distance(dms2) / 1000.0_f64;
        assert!((expected_km - d_km).abs() < 1.0);
    }

    #[test]
    fn test_azimuth() {
        let dms1 = DMS3d::from_decimal_degrees( // NY
            40.73,
            -73.93,
            None,
        );
        let dms2 = DMS3d::from_decimal_degrees( // Paris
            48.85, 
            2.2321,
            None,
        );
        let expected = 53.78;
        //assert!((expected - dms1.azimuth(dms2)) < 0.1)
        assert_eq!(dms1.azimuth(dms2), expected)
    }
    
    #[test]
    fn test_to_cartesian() {
        assert_eq!(
            DMS3d::from_decimal_degrees(
                40.73,
                -73.93,
                None).to_cartesian(),
            rust_3d::Point3D::new(0.0,0.0,0.0))
    }

    #[test]
    fn test_to_gpx() {
        let dms = DMS3d::from_decimal_degrees(
            40.730610_f64, // NY
            -73.935242_f64, // NY
            Some(10.0)
        );
        assert_eq!(dms.to_gpx("ny.gpx").is_ok(), true);
        let ny = DMS3d::from_gpx("ny.gpx")
            .unwrap()
            .unwrap();
        assert_eq!(ny.distance(dms), 0.0)
    }
}
