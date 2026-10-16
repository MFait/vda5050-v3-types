//! Order body and graph models (VDA 5050 v3.0.0, section 7.3).

use crate::common::{Action, Header, Trajectory};

object_only_deserialize! {
/// An order graph, including its released base and unreleased horizon.
///
/// Encoding/decoding checks wire types and field-level numeric constraints,
/// not graph counts, sequence continuity, release consistency, action-ID
/// uniqueness, geometry or compatibility with an active order/robot.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct Order {
    /// Shared header, flattened into the wire object.
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub header: Header,
    /// Identifies messages belonging to the same order.
    pub order_id: String,
    /// Update counter, specified as uint32; progression is not validated.
    pub order_update_id: u32,
    /// Human-readable information, not intended for logical processing.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub order_description: Option<String>,
    /// Nodes in traversal order. Protocol count requirements are not checked.
    pub nodes: Vec<Node>,
    /// Edges in traversal order; may be empty for a single-node order.
    pub edges: Vec<Edge>,
}
}

#[cfg(feature = "json")]
impl_json_helpers!(Order);

object_only_deserialize! {
/// Node occurrence in an order. Repeated node IDs are distinguished by sequence.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct Node {
    /// Node identifier; may recur within an order.
    pub node_id: String,
    /// Shared node/edge traversal sequence, specified as uint32.
    pub sequence_id: u32,
    /// Optional human-readable information.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub node_descriptor: Option<String>,
    /// True for the base; false for the horizon.
    pub released: bool,
    /// Optional for robots that do not require map positions.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub node_position: Option<NodePosition>,
    /// Actions on this node, or an empty list.
    pub actions: Vec<Action>,
}
}

object_only_deserialize! {
/// Node position in the global project-specific map coordinate system.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct NodePosition {
    /// Finite X-position in meters.
    #[cfg_attr(feature = "serde", serde(with = "crate::serde_helpers::x"))]
    pub x: f64,
    /// Finite Y-position in meters.
    #[cfg_attr(feature = "serde", serde(with = "crate::serde_helpers::y"))]
    pub y: f64,
    /// Required orientation in radians, in `[-pi, pi]`, if specified.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::theta::optional"))]
    pub theta: Option<f64>,
    /// Position tolerance ellipse. Omission does not materialize protocol defaults.
    #[cfg_attr(feature = "serde", serde(rename = "allowedDeviationXY", default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub allowed_deviation_xy: Option<AllowedDeviationXy>,
    /// Finite orientation tolerance in `[0, pi]` radians; protocol default is zero.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::allowed_deviation_theta::optional"))]
    pub allowed_deviation_theta: Option<f64>,
    /// Map identifier, sharing the project's global coordinate origin.
    pub map_id: String,
}
}

object_only_deserialize! {
/// Ellipse centered on the node's position.
///
/// The schema restricts rotation to `[-1.570796327, 1.570796327]` radians;
/// section 7.3 does not state that bound. Axis ordering is not validated.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct AllowedDeviationXy {
    /// Finite nonnegative semi-major axis length in meters.
    #[cfg_attr(feature = "serde", serde(with = "crate::serde_helpers::ellipse_a"))]
    pub a: f64,
    /// Finite nonnegative semi-minor axis length in meters.
    #[cfg_attr(feature = "serde", serde(with = "crate::serde_helpers::ellipse_b"))]
    pub b: f64,
    /// Rotation from the positive horizontal axis, in radians.
    #[cfg_attr(feature = "serde", serde(with = "crate::serde_helpers::ellipse_theta"))]
    pub theta: f64,
}
}

object_only_deserialize! {
/// Directional connection between consecutive nodes in an order.
///
/// Adjacent nodes are determined by sequence, not start/end-node ID fields.
/// Robot-specific applicability and traversal behavior are not validated.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct Edge {
    /// Edge identifier; may recur within an order.
    pub edge_id: String,
    /// Shared node/edge traversal sequence, specified as uint32.
    pub sequence_id: u32,
    /// Human-readable information, not intended for logical processing.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub edge_descriptor: Option<String>,
    /// True for the base; false for the horizon.
    pub released: bool,
    /// Finite permitted maximum speed in meters per second.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::maximum_speed::optional"))]
    pub maximum_speed: Option<f64>,
    /// Finite permitted maximum robot height, including load, in meters.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::maximum_mobile_robot_height::optional"))]
    pub maximum_mobile_robot_height: Option<f64>,
    /// Finite permitted minimum load-handling-device height in meters.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::minimum_load_handling_device_height::optional"))]
    pub minimum_load_handling_device_height: Option<f64>,
    /// Desired orientation in radians, in `[-pi, pi]`.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::orientation::optional"))]
    pub orientation: Option<f64>,
    /// Orientation reference; the protocol default is TANGENTIAL.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub orientation_type: Option<OrientationType>,
    /// Robot-specific direction at a junction, not a closed enumeration.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub direction: Option<String>,
    /// Whether orientation must be reached before entry; protocol default false.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub reach_orientation_before_entering: Option<bool>,
    /// Finite rotation speed limit in radians per second; absent means no limit.
    ///
    /// Uses section 7.3's `maximumRotationSpeed`, not the pinned schema's typo
    /// `maxRotationSpeed`. The latter is an ignored unknown field, not an alias.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::maximum_rotation_speed::optional"))]
    pub maximum_rotation_speed: Option<f64>,
    /// Optional NURBS path; cross-field geometric checks are deferred.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub trajectory: Option<Trajectory>,
    /// Finite path length in meters.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::length::optional"))]
    pub length: Option<f64>,
    /// Optional boundaries for deviations from the trajectory.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub corridor: Option<Corridor>,
    /// Edge actions, or an empty list. All four blocking types are supported.
    pub actions: Vec<Action>,
}
}

string_enum! {
/// Reference used to interpret edge orientation (section 7.3).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "SCREAMING_SNAKE_CASE"))]
pub enum OrientationType {
    /// Global project-specific map coordinates.
    Global,
    /// Tangent to the edge trajectory; the protocol default.
    Tangential,
}
}

object_only_deserialize! {
/// Boundaries for deviation from an edge's trajectory (section 7.3).
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct Corridor {
    /// Finite nonnegative width to the left of the trajectory, in meters.
    #[cfg_attr(feature = "serde", serde(with = "crate::serde_helpers::left_width"))]
    pub left_width: f64,
    /// Finite nonnegative width to the right of the trajectory, in meters.
    #[cfg_attr(feature = "serde", serde(with = "crate::serde_helpers::right_width"))]
    pub right_width: f64,
    /// Reference for the boundaries; protocol default KINEMATIC_CENTER.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub corridor_reference_point: Option<CorridorReferencePoint>,
    /// Whether fleet-control approval is required; protocol default false.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub release_required: Option<bool>,
    /// Behavior on release expiry/revocation; protocol default STOP.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub release_loss_behavior: Option<ReleaseLossBehavior>,
}
}

string_enum! {
/// Reference point for corridor widths (section 7.3).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "SCREAMING_SNAKE_CASE"))]
pub enum CorridorReferencePoint {
    /// Robot's kinematic center; the protocol default.
    KinematicCenter,
    /// Robot contour.
    Contour,
}
}

string_enum! {
/// Robot behavior on loss of corridor release (section 7.3).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "SCREAMING_SNAKE_CASE"))]
pub enum ReleaseLossBehavior {
    /// Stop and await manual intervention; the protocol default.
    Stop,
    /// Return to the edge's predefined trajectory.
    Return,
}
}
