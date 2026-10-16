use chrono::{DateTime, Utc};

object_only_deserialize! {
/// Robot-planned NURBS path within its active order (sections 6.8 and 7.8).
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct PlannedPath {
    /// Curve defining the path.
    pub trajectory: Trajectory,
    /// IDs of order nodes traversed within this path, if supplied.
    #[cfg_attr(
        feature = "serde",
        serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::serde_helpers::non_null_option"
        )
    )]
    pub traversed_nodes: Option<Vec<String>>,
}
}

object_only_deserialize! {
/// NURBS trajectory shared by planned paths and order/state models.
///
/// Sections 7.3 and 7.8 specify degree/control-point counts, clamping and knot
/// multiplicities. Those cross-field constraints are not currently validated.
/// Absence preserves the wire omission; protocol defaults are not materialized.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct Trajectory {
    /// Degree in `1..=u32::MAX`. The protocol default when absent is 1.
    #[cfg_attr(
        feature = "serde",
        serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::degree::optional"
        )
    )]
    pub degree: Option<u32>,
    /// Finite knots in `[0, 1]`. Absence requests the protocol's uniform knots.
    #[cfg_attr(
        feature = "serde",
        serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::knots::optional"
        )
    )]
    pub knot_vector: Option<Vec<f64>>,
    /// Curve control points, including the start and end point.
    pub control_points: Vec<ControlPoint>,
}
}

object_only_deserialize! {
/// NURBS control point in project coordinates (sections 7.3 and 7.8).
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ControlPoint {
    /// Finite X-coordinate in meters.
    #[cfg_attr(feature = "serde", serde(with = "crate::serde_helpers::x"))]
    pub x: f64,
    /// Finite Y-coordinate in meters.
    #[cfg_attr(feature = "serde", serde(with = "crate::serde_helpers::y"))]
    pub y: f64,
    /// Finite, strictly positive weight. The protocol default when absent is 1.
    #[cfg_attr(
        feature = "serde",
        serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::weight::optional"
        )
    )]
    pub weight: Option<f64>,
}
}

object_only_deserialize! {
/// Near-term path represented as a polyline (sections 6.8 and 7.8).
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct IntermediatePath {
    /// Segment endpoints. Geometry and correspondence to current position
    /// are not validated.
    pub polyline: Vec<Waypoint>,
}
}

object_only_deserialize! {
/// Endpoint of a polyline segment (section 7.8).
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Waypoint {
    /// Finite X-coordinate in meters.
    #[cfg_attr(feature = "serde", serde(with = "crate::serde_helpers::x"))]
    pub x: f64,
    /// Finite Y-coordinate in meters.
    #[cfg_attr(feature = "serde", serde(with = "crate::serde_helpers::y"))]
    pub y: f64,
    /// Optional absolute orientation in radians, in `[-pi, pi]`.
    #[cfg_attr(
        feature = "serde",
        serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::theta::optional"
        )
    )]
    pub theta: Option<f64>,
    /// Estimated arrival/traversal time, using the header timestamp wire format.
    #[cfg_attr(feature = "serde", serde(with = "crate::timestamp"))]
    pub eta: DateTime<Utc>,
}
}
