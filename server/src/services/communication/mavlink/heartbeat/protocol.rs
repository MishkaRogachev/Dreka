use std::collections::HashMap;

use mavlink::common::MavType;
use mavlink::ardupilotmega::{PlaneMode, CopterMode};

use crate::models::vehicles::VehicleMode;

fn wrap_mode<T: num_traits::ToPrimitive>(value: T) -> u32 {
    num_traits::cast::ToPrimitive::to_u32(&value).unwrap()
}

pub fn apm_plane_modes() -> HashMap<u32, VehicleMode> {
    HashMap::from([
        (wrap_mode(PlaneMode::PLANE_MODE_MANUAL), VehicleMode::Manual),
        (wrap_mode(PlaneMode::PLANE_MODE_CIRCLE), VehicleMode::Circle),
        (wrap_mode(PlaneMode::PLANE_MODE_STABILIZE), VehicleMode::Stabilize),
        (wrap_mode(PlaneMode::PLANE_MODE_TRAINING), VehicleMode::Training),
        (wrap_mode(PlaneMode::PLANE_MODE_ACRO), VehicleMode::Acro),
        (wrap_mode(PlaneMode::PLANE_MODE_FLY_BY_WIRE_A), VehicleMode::FBWA),
        (wrap_mode(PlaneMode::PLANE_MODE_FLY_BY_WIRE_B), VehicleMode::FBWB),
        (wrap_mode(PlaneMode::PLANE_MODE_CRUISE), VehicleMode::Cruise),
        (wrap_mode(PlaneMode::PLANE_MODE_AUTOTUNE), VehicleMode::Autotune),
        (wrap_mode(PlaneMode::PLANE_MODE_AUTO), VehicleMode::Mission),
        (wrap_mode(PlaneMode::PLANE_MODE_RTL), VehicleMode::RTL),
        (wrap_mode(PlaneMode::PLANE_MODE_LOITER), VehicleMode::Loiter),
        (wrap_mode(PlaneMode::PLANE_MODE_GUIDED), VehicleMode::Guided),
        (wrap_mode(PlaneMode::PLANE_MODE_INITIALIZING), VehicleMode::Initializing),
        (wrap_mode(PlaneMode::PLANE_MODE_QSTABILIZE), VehicleMode::QStabilize),
        (wrap_mode(PlaneMode::PLANE_MODE_QHOVER), VehicleMode::QHover),
        (wrap_mode(PlaneMode::PLANE_MODE_QLOITER), VehicleMode::QLoiter),
        (wrap_mode(PlaneMode::PLANE_MODE_QLAND), VehicleMode::QLand),
        (wrap_mode(PlaneMode::PLANE_MODE_QRTL), VehicleMode::QRTL),
        (wrap_mode(PlaneMode::PLANE_MODE_QAUTOTUNE), VehicleMode::QAutotune),
        (wrap_mode(PlaneMode::PLANE_MODE_QACRO), VehicleMode::QAcro),
        (wrap_mode(PlaneMode::PLANE_MODE_TAKEOFF), VehicleMode::Takeoff),
        (wrap_mode(PlaneMode::PLANE_MODE_AVOID_ADSB), VehicleMode::Avoidance),
        (wrap_mode(PlaneMode::PLANE_MODE_THERMAL), VehicleMode::Thermal),
    ])
}

pub fn apm_copter_modes() -> HashMap<u32, VehicleMode> {
    HashMap::from([
        (wrap_mode(CopterMode::COPTER_MODE_STABILIZE), VehicleMode::Stabilize),
        (wrap_mode(CopterMode::COPTER_MODE_ACRO), VehicleMode::Acro),
        (wrap_mode(CopterMode::COPTER_MODE_ALT_HOLD), VehicleMode::AltHold),
        (wrap_mode(CopterMode::COPTER_MODE_AUTO), VehicleMode::Mission),
        (wrap_mode(CopterMode::COPTER_MODE_GUIDED), VehicleMode::Guided),
        (wrap_mode(CopterMode::COPTER_MODE_LOITER), VehicleMode::Loiter),
        (wrap_mode(CopterMode::COPTER_MODE_RTL), VehicleMode::RTL),
        (wrap_mode(CopterMode::COPTER_MODE_CIRCLE), VehicleMode::Circle),
        (wrap_mode(CopterMode::COPTER_MODE_LAND), VehicleMode::Land),
        (wrap_mode(CopterMode::COPTER_MODE_DRIFT), VehicleMode::Drift),
        (wrap_mode(CopterMode::COPTER_MODE_SPORT), VehicleMode::Sport),
        (wrap_mode(CopterMode::COPTER_MODE_FLIP), VehicleMode::Flip),
        (wrap_mode(CopterMode::COPTER_MODE_AUTOTUNE), VehicleMode::Autotune),
        (wrap_mode(CopterMode::COPTER_MODE_POSHOLD), VehicleMode::PosHold),
        (wrap_mode(CopterMode::COPTER_MODE_BRAKE), VehicleMode::Break),
        (wrap_mode(CopterMode::COPTER_MODE_THROW), VehicleMode::Throw),
        (wrap_mode(CopterMode::COPTER_MODE_AVOID_ADSB), VehicleMode::Avoidance),
        (wrap_mode(CopterMode::COPTER_MODE_GUIDED_NOGPS), VehicleMode::Guided),
        (wrap_mode(CopterMode::COPTER_MODE_SMART_RTL), VehicleMode::SmartRTL),
        (wrap_mode(CopterMode::COPTER_MODE_FLOWHOLD), VehicleMode::FlowHold),
        (wrap_mode(CopterMode::COPTER_MODE_FOLLOW), VehicleMode::Follow),
        (wrap_mode(CopterMode::COPTER_MODE_ZIGZAG), VehicleMode::ZigZag),
        (wrap_mode(CopterMode::COPTER_MODE_AUTOROTATE), VehicleMode::Autorotate),
        (wrap_mode(CopterMode::COPTER_MODE_AUTO_RTL), VehicleMode::RTL),
    ])
}


pub fn apm_modes(mav_type: MavType) -> HashMap<u32, VehicleMode>  {
    match mav_type {
        MavType::MAV_TYPE_FIXED_WING | MavType::MAV_TYPE_KITE | MavType::MAV_TYPE_FLAPPING_WING |
        MavType::MAV_TYPE_VTOL_TAILSITTER_DUOROTOR | MavType::MAV_TYPE_VTOL_TILTROTOR | MavType::MAV_TYPE_VTOL_FIXEDROTOR |
        MavType::MAV_TYPE_VTOL_TAILSITTER | MavType::MAV_TYPE_VTOL_TILTWING | MavType::MAV_TYPE_VTOL_RESERVED5 =>
            return apm_plane_modes(),
        MavType::MAV_TYPE_TRICOPTER | MavType::MAV_TYPE_QUADROTOR | MavType::MAV_TYPE_HEXAROTOR | MavType::MAV_TYPE_OCTOROTOR |
        MavType::MAV_TYPE_COAXIAL | MavType::MAV_TYPE_HELICOPTER =>
            return apm_copter_modes(),
        _ => return HashMap::new()
    }
}

pub fn available_apm_modes(mav_type: MavType) -> Vec<VehicleMode> {
    match mav_type {
        MavType::MAV_TYPE_FIXED_WING | MavType::MAV_TYPE_KITE | MavType::MAV_TYPE_FLAPPING_WING =>
            return vec!(
                VehicleMode::Mission,
                VehicleMode::RTL,
                VehicleMode::Loiter,
                VehicleMode::Circle,
                VehicleMode::Guided,
                VehicleMode::Takeoff
            ),
        MavType::MAV_TYPE_VTOL_TAILSITTER_DUOROTOR | MavType::MAV_TYPE_VTOL_TILTROTOR | MavType::MAV_TYPE_VTOL_FIXEDROTOR |
        MavType::MAV_TYPE_VTOL_TAILSITTER | MavType::MAV_TYPE_VTOL_TILTWING | MavType::MAV_TYPE_VTOL_RESERVED5 =>
            return vec!(
                VehicleMode::Mission,
                VehicleMode::RTL,
                VehicleMode::Loiter,
                VehicleMode::Circle,
                VehicleMode::Guided,
                VehicleMode::Takeoff,
                VehicleMode::QStabilize,
                VehicleMode::QHover,
                VehicleMode::QLoiter,
                VehicleMode::QLand,
                VehicleMode::QRTL,
                VehicleMode::QAutotune,
                VehicleMode::QAcro
            ),
        MavType::MAV_TYPE_TRICOPTER | MavType::MAV_TYPE_QUADROTOR | MavType::MAV_TYPE_HEXAROTOR | MavType::MAV_TYPE_OCTOROTOR |
        MavType::MAV_TYPE_COAXIAL | MavType::MAV_TYPE_HELICOPTER =>
        return vec!(
            VehicleMode::Mission,
            VehicleMode::RTL,
            VehicleMode::AltHold,
            VehicleMode::Guided,
            VehicleMode::Loiter,
            VehicleMode::Circle,
            VehicleMode::PosHold
        ),
        _ => return Vec::new()
    }
}
