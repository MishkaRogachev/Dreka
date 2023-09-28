use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, Debug, PartialEq, TS)]
#[ts(export)]
pub enum GeodeticFrame {
    None,
    Wgs84RelativeHome,
    Wgs84AboveSeaLevel,
    Wgs84AboveTerrain
}

#[derive(Serialize, Deserialize, Debug, PartialEq, TS)]
#[ts(export)]
pub struct Geodetic {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f32,
    pub frame: GeodeticFrame
}

impl Default for Geodetic {
    fn default() -> Geodetic {
        Geodetic {
            latitude: std::f64::NAN,
            longitude: std::f64::NAN,
            altitude: std::f32::NAN,
            frame: GeodeticFrame::None
        }
    }
}
