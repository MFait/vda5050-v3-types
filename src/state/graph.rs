use crate::common::Trajectory;

object_only_deserialize! {
/// Map stored on the robot (section 7.8).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct Map {
    /// Map identifier.
    pub map_id: String,
    /// Stored version.
    pub map_version: String,
    /// Whether this version is enabled.
    pub map_status: MapStatus,
    /// Human-readable descriptor.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub map_descriptor: Option<String>,
}
}

string_enum! {
/// Status of a stored map version.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "SCREAMING_SNAKE_CASE"))]
pub enum MapStatus {
    /// Actively used.
    Enabled,
    /// Stored but not enabled.
    Disabled,
}
}

object_only_deserialize! {
/// Stored zone-set status, not the zoneSet message body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct ZoneSetState {
    /// Zone-set identifier; may be empty if the map has no defined zones.
    pub zone_set_id: String,
    /// Corresponding map.
    pub map_id: String,
    /// Whether the zone set is enabled.
    pub zone_set_status: ZoneSetStatus,
}
}

string_enum! {
/// Status of a stored zone set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "SCREAMING_SNAKE_CASE"))]
pub enum ZoneSetStatus {
    /// Actively used.
    Enabled,
    /// Stored but not enabled.
    Disabled,
}
}

object_only_deserialize! {
/// Node remaining in the order.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct NodeState {
    /// Node identifier; may recur at another sequence.
    pub node_id: String,
    /// Traversal sequence, specified as uint32.
    pub sequence_id: u32,
    /// Human-readable descriptor.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub node_descriptor: Option<String>,
    /// True for base, false for horizon.
    pub released: bool,
    /// Optional diagnostic position, without order tolerances.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub node_position: Option<NodePosition>,
}
}

object_only_deserialize! {
/// State node position; unlike order positions, contains no tolerances.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct NodePosition {
    /// Finite X-coordinate in meters.
    #[cfg_attr(feature = "serde", serde(with = "crate::serde_helpers::x"))]
    pub x: f64,
    /// Finite Y-coordinate in meters.
    #[cfg_attr(feature = "serde", serde(with = "crate::serde_helpers::y"))]
    pub y: f64,
    /// Optional orientation in `[-pi, pi]` radians.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::theta::optional"))]
    pub theta: Option<f64>,
    /// Project-specific map identifier.
    pub map_id: String,
}
}

object_only_deserialize! {
/// Edge remaining in the order.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct EdgeState {
    /// Edge identifier.
    pub edge_id: String,
    /// Traversal sequence, specified as uint32.
    pub sequence_id: u32,
    /// Human-readable descriptor.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub edge_descriptor: Option<String>,
    /// True for base, false for horizon.
    pub released: bool,
    /// Previously defined or ordered trajectory.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub trajectory: Option<Trajectory>,
}
}

object_only_deserialize! {
/// Request concerning a zone; lifecycle and trajectory applicability are unchecked.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct ZoneRequest {
    /// Identifier unique per robot among active requests.
    pub request_id: String,
    /// Request category.
    pub request_type: ZoneRequestType,
    /// Zone identifier local to the zone set.
    pub zone_id: String,
    /// Zone set containing the zone.
    pub zone_set_id: String,
    /// Reported request state.
    pub request_status: RequestStatus,
    /// Planned trajectory through the zone, if supplied.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub trajectory: Option<Trajectory>,
}
}

string_enum! {
/// Listed zone request types from section 7.8.
///
/// The trajectory prose mentions COORDINATED_REPLANNING, which is not a listed
/// enum value in either schema or specification. It is not accepted here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "SCREAMING_SNAKE_CASE"))]
pub enum ZoneRequestType {
    /// Request access.
    Access,
    /// Request replanning.
    Replanning,
}
}

object_only_deserialize! {
/// Request concerning an edge corridor.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct EdgeRequest {
    /// Identifier unique per robot among active requests.
    pub request_id: String,
    /// Edge request category.
    pub request_type: EdgeRequestType,
    /// Referenced edge.
    pub edge_id: String,
    /// Referenced edge sequence, specified as uint32.
    pub sequence_id: u32,
    /// Reported request state.
    pub request_status: RequestStatus,
}
}

string_enum! {
/// Closed edge request types from section 7.8.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "SCREAMING_SNAKE_CASE"))]
pub enum EdgeRequestType {
    /// Request deviation within the edge's corridor.
    Corridor,
}
}

string_enum! {
/// Request lifecycle values from section 7.8; transitions are not checked.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "SCREAMING_SNAKE_CASE"))]
pub enum RequestStatus {
    /// Awaiting a response.
    Requested,
    /// Permission granted.
    Granted,
    /// Permission revoked.
    Revoked,
    /// Lease expired.
    Expired,
}
}
