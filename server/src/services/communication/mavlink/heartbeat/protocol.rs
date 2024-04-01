use mavlink::common::{MavAutopilot, MavType};
use mavlink::ardupilotmega::{PlaneMode, CopterMode};

use crate::models::vehicles::VehicleMode;

pub fn decode_apm_plane_mode(custom_mode: u32) -> VehicleMode {
    let mode: Option<PlaneMode> = num_traits::cast::FromPrimitive::from_u32(custom_mode);
    if mode.is_none() {
        log::warn!("Unknown plane mode: {}", custom_mode);
        return VehicleMode::None;
    }
    match mode.unwrap() {
        PlaneMode::PLANE_MODE_MANUAL => VehicleMode::Manual,
        PlaneMode::PLANE_MODE_CIRCLE => VehicleMode::Circle,
        PlaneMode::PLANE_MODE_STABILIZE => VehicleMode::Stabilize,
        PlaneMode::PLANE_MODE_TRAINING => VehicleMode::Training,
        PlaneMode::PLANE_MODE_ACRO => VehicleMode::Acro,
        PlaneMode::PLANE_MODE_FLY_BY_WIRE_A => VehicleMode::FBWA,
        PlaneMode::PLANE_MODE_FLY_BY_WIRE_B => VehicleMode::FBWB,
        PlaneMode::PLANE_MODE_CRUISE => VehicleMode::Cruise,
        PlaneMode::PLANE_MODE_AUTOTUNE => VehicleMode::Autotune,
        PlaneMode::PLANE_MODE_AUTO => VehicleMode::Mission,
        PlaneMode::PLANE_MODE_RTL => VehicleMode::RTL,
        PlaneMode::PLANE_MODE_LOITER => VehicleMode::Loiter,
        PlaneMode::PLANE_MODE_GUIDED => VehicleMode::Guided,
        PlaneMode::PLANE_MODE_INITIALIZING => VehicleMode::Initializing,
        PlaneMode::PLANE_MODE_QSTABILIZE => VehicleMode::QStabilize,
        PlaneMode::PLANE_MODE_QHOVER => VehicleMode::QHover,
        PlaneMode::PLANE_MODE_QLOITER => VehicleMode::QLoiter,
        PlaneMode::PLANE_MODE_QLAND => VehicleMode::QLand,
        PlaneMode::PLANE_MODE_QRTL => VehicleMode::QRTL,
        PlaneMode::PLANE_MODE_QAUTOTUNE => VehicleMode::QAutotune,
        PlaneMode::PLANE_MODE_QACRO => VehicleMode::QAcro,
        PlaneMode::PLANE_MODE_TAKEOFF => VehicleMode::Takeoff,
        PlaneMode::PLANE_MODE_AVOID_ADSB => VehicleMode::Avoidance,
        PlaneMode::PLANE_MODE_THERMAL => VehicleMode::Thermal,
    }
}

pub fn decode_apm_copter_mode(custom_mode: u32) -> VehicleMode {
    let mode: Option<CopterMode> = num_traits::cast::FromPrimitive::from_u32(custom_mode);
    if mode.is_none() {
        log::warn!("Unknown copter mode: {}", custom_mode);
        return VehicleMode::None;
    }
    match mode.unwrap() {
        CopterMode::COPTER_MODE_STABILIZE => VehicleMode::Stabilize,
        CopterMode::COPTER_MODE_ACRO => VehicleMode::Acro,
        CopterMode::COPTER_MODE_ALT_HOLD => VehicleMode::AltHold,
        CopterMode::COPTER_MODE_AUTO => VehicleMode::Mission,
        CopterMode::COPTER_MODE_GUIDED => VehicleMode::Guided,
        CopterMode::COPTER_MODE_LOITER => VehicleMode::Loiter,
        CopterMode::COPTER_MODE_RTL => VehicleMode::RTL,
        CopterMode::COPTER_MODE_CIRCLE => VehicleMode::Circle,
        CopterMode::COPTER_MODE_LAND => VehicleMode::Land,
        CopterMode::COPTER_MODE_DRIFT => VehicleMode::Drift,
        CopterMode::COPTER_MODE_SPORT => VehicleMode::Sport,
        CopterMode::COPTER_MODE_FLIP => VehicleMode::Flip,
        CopterMode::COPTER_MODE_AUTOTUNE => VehicleMode::Autotune,
        CopterMode::COPTER_MODE_POSHOLD => VehicleMode::PosHold,
        CopterMode::COPTER_MODE_BRAKE => VehicleMode::Break,
        CopterMode::COPTER_MODE_THROW => VehicleMode::Throw,
        CopterMode::COPTER_MODE_AVOID_ADSB => VehicleMode::Avoidance,
        CopterMode::COPTER_MODE_GUIDED_NOGPS => VehicleMode::Guided,
        CopterMode::COPTER_MODE_SMART_RTL => VehicleMode::SmartRTL,
        CopterMode::COPTER_MODE_FLOWHOLD => VehicleMode::FlowHold,
        CopterMode::COPTER_MODE_FOLLOW => VehicleMode::Follow,
        CopterMode::COPTER_MODE_ZIGZAG => VehicleMode::ZigZag,
        CopterMode::COPTER_MODE_SYSTEMID => todo!(),
        CopterMode::COPTER_MODE_AUTOROTATE => VehicleMode::Autorotate,
        CopterMode::COPTER_MODE_AUTO_RTL => VehicleMode::RTL,
    }
}

pub fn decode_apm_mode(mav_type: MavType, custom_mode: u32) -> VehicleMode {
    match mav_type {
        MavType::MAV_TYPE_FIXED_WING | MavType::MAV_TYPE_KITE | MavType::MAV_TYPE_FLAPPING_WING |
        MavType::MAV_TYPE_VTOL_TAILSITTER_DUOROTOR | MavType::MAV_TYPE_VTOL_TILTROTOR | MavType::MAV_TYPE_VTOL_FIXEDROTOR |
        MavType::MAV_TYPE_VTOL_TAILSITTER | MavType::MAV_TYPE_VTOL_TILTWING | MavType::MAV_TYPE_VTOL_RESERVED5 =>
            return decode_apm_plane_mode(custom_mode),
        MavType::MAV_TYPE_TRICOPTER | MavType::MAV_TYPE_QUADROTOR | MavType::MAV_TYPE_HEXAROTOR | MavType::MAV_TYPE_OCTOROTOR |
        MavType::MAV_TYPE_COAXIAL | MavType::MAV_TYPE_HELICOPTER =>
            return decode_apm_copter_mode(custom_mode),
        _ => return VehicleMode::None
    }
}

pub fn available_apm_modes(mav_type: MavType) -> Vec<VehicleMode> {
    match mav_type {
        MavType::MAV_TYPE_FIXED_WING | MavType::MAV_TYPE_KITE | MavType::MAV_TYPE_FLAPPING_WING =>
            return vec!(
                VehicleMode::RTL,
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
                VehicleMode::RTL,
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
            VehicleMode::RTL,
            VehicleMode::Mission,
            VehicleMode::AltHold,
            VehicleMode::Guided,
            VehicleMode::Loiter,
            VehicleMode::Circle,
            VehicleMode::PosHold
        ),
        _ => return Vec::new()
    }
}