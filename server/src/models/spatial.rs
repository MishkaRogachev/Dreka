use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum GeodeticFrame {
    None,

    Wgs84RelativeHome,
    Wgs84AboveSeaLevel,
    Wgs84AboveTerrain
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Geodetic {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f32,
    pub frame: GeodeticFrame
}

impl Default for Geodetic {
    fn default() -> Geodetic {
        Geodetic {
            latitude: 0.0,
            longitude: 0.0,
            altitude: 0.0,
            frame: GeodeticFrame::None
        }
    }
}
