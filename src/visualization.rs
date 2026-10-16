//! Visualization body (VDA 5050 v3.0.0, sections 6.7, 6.8 and 7.9).
//!
//! Header and reference-state counter are required. Each of the four body
//! objects is independently optional and non-nullable.

use crate::common::{Header, IntermediatePath, MobileRobotPosition, PlannedPath, Velocity};

object_only_deserialize! {
/// Position, velocity and robot-planned paths for visualization.
///
/// Uses the same nested models as the specification's state message.
/// Decoding does not verify that a referenced state exists, that paths begin
/// at the current robot position, or that they cover the active base.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct Visualization {
    /// Shared protocol header, flattened into the JSON root.
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub header: Header,
    /// Header ID of the state message being referenced (`uint32`, section 7.9).
    pub reference_state_header_id: u32,
    /// Robot-planned NURBS path within its current order.
    #[cfg_attr(
        feature = "serde",
        serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::serde_helpers::non_null_option"
        )
    )]
    pub planned_path: Option<PlannedPath>,
    /// Near-term polyline with estimated arrival times.
    #[cfg_attr(
        feature = "serde",
        serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::serde_helpers::non_null_option"
        )
    )]
    pub intermediate_path: Option<IntermediatePath>,
    /// Current position in project-specific map coordinates.
    #[cfg_attr(
        feature = "serde",
        serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::serde_helpers::non_null_option"
        )
    )]
    pub mobile_robot_position: Option<MobileRobotPosition>,
    /// Velocity in robot coordinates.
    #[cfg_attr(
        feature = "serde",
        serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::serde_helpers::non_null_option"
        )
    )]
    pub velocity: Option<Velocity>,
}
}

#[cfg(feature = "json")]
impl_json_helpers!(Visualization);
