//! Package to manipulate 1D and 3D D°M'S'' coordinates,
//! mainly in navigation applications.   
//! Homepage: <https://github.com/gwbres/dms-coordinates>
mod dms;
mod dms3d;
mod bearing;

pub use crate::{
    dms::*,
    dms3d::*,
    bearing::*
};

use initial_conditions::EARTH_RADIUS;

/// Returns distance (m) between two decimal degrees coordinates
/// coord1: (lat,lon), coord2: (lat, lon)
pub fn projected_distance (coord1: (f64,f64), coord2: (f64,f64)) -> f64 {
    let dphi = map_3d::deg2rad(coord2.0) - map_3d::deg2rad(coord1.0);
    let d_lambda = map_3d::deg2rad(coord2.1) - map_3d::deg2rad(coord1.1);
    let a: f64 = (dphi / 2.0_f64).sin().powf(2.0_f64)
        + map_3d::deg2rad(coord1.0).cos() * map_3d::deg2rad(coord2.0).cos()
            * (d_lambda/2.0_f64).sin().powf(2.0_f64);
    let c = 2.0_f64 * a.powf(0.5_f64).atan2((1.0-a).powf(0.5_f64));
    EARTH_RADIUS * c
}
