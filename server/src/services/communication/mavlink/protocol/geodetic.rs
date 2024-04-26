use mavlink::common::MavFrame;

use crate::models::spatial::{Geodetic, GeodeticFrame};

use super::telemetry::{decode_lat_lon, encode_lat_lon};

impl Geodetic {
    pub fn to_mavlink(&self) -> (MavFrame, i32, i32, f32) {
        (
            match self.frame {
                GeodeticFrame::None => MavFrame::MAV_FRAME_GLOBAL, // Use global as default
                GeodeticFrame::Wgs84RelativeHome => MavFrame::MAV_FRAME_GLOBAL_RELATIVE_ALT,
                GeodeticFrame::Wgs84AboveSeaLevel => MavFrame::MAV_FRAME_GLOBAL,
                GeodeticFrame::Wgs84AboveTerrain => MavFrame::MAV_FRAME_GLOBAL_TERRAIN_ALT,
            },
            encode_lat_lon(self.latitude),
            encode_lat_lon(self.longitude),
            self.altitude
        )
    }

    pub fn from_mavlink(x: i32, y: i32, z: f32, frame: MavFrame) -> Geodetic {
        Geodetic {
            latitude: decode_lat_lon(x),
            longitude: decode_lat_lon(y),
            altitude: z,
            frame: {
                match frame {
                    MavFrame::MAV_FRAME_GLOBAL => GeodeticFrame::Wgs84AboveSeaLevel,
                    MavFrame::MAV_FRAME_GLOBAL_RELATIVE_ALT => GeodeticFrame::Wgs84RelativeHome,
                    MavFrame::MAV_FRAME_GLOBAL_TERRAIN_ALT => GeodeticFrame::Wgs84AboveTerrain,
                    _ => GeodeticFrame::None,
                }
            }
        }
    }
}
