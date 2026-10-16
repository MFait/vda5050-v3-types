//! Robot state body and supporting models (VDA 5050 v3.0.0, section 7.8).

mod diagnostics;
mod graph;
mod load;
mod power;

pub use crate::common::{BoundingBoxReference, LoadDimensions};
pub use diagnostics::{Error, ErrorLevel, Info, InfoLevel, Reference, Translation};
pub use graph::{
    EdgeRequest, EdgeRequestType, EdgeState, Map, MapStatus, NodePosition, NodeState,
    RequestStatus, ZoneRequest, ZoneRequestType, ZoneSetState, ZoneSetStatus,
};
pub use load::Load;
pub use power::PowerSupply;

use crate::common::{Header, IntermediatePath, MobileRobotPosition, PlannedPath, Velocity};

object_only_deserialize! {
/// Robot state snapshot.
///
/// Wire checks do not validate robot capabilities, graph consistency, unique
/// identifiers, lifecycle transitions, request grants or physical behavior.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct State {
    /// Shared header, flattened into the wire object.
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub header: Header,
    /// Stored maps, if reported.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub maps: Option<Vec<Map>>,
    /// Stored zone sets, if reported.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub zone_sets: Option<Vec<ZoneSetState>>,
    /// Current or previous order ID; empty if none is available.
    pub order_id: String,
    /// Last accepted order update, specified as uint32.
    pub order_update_id: u32,
    /// Last/current node ID; empty if unavailable.
    pub last_node_id: String,
    /// Last/current node sequence; ignored by the protocol if lastNodeId is empty.
    pub last_node_sequence_id: u32,
    /// Nodes still to be traversed; empty when idle.
    pub node_states: Vec<NodeState>,
    /// Edges still to be traversed; empty when idle.
    pub edge_states: Vec<EdgeState>,
    /// Robot-planned NURBS path in the active order.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub planned_path: Option<PlannedPath>,
    /// Near-term polyline with estimated arrival times.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub intermediate_path: Option<IntermediatePath>,
    /// Current map position, when the robot supports localization.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub mobile_robot_position: Option<MobileRobotPosition>,
    /// Velocity in robot coordinates.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub velocity: Option<Velocity>,
    /// Omit if load state is unknown; an empty list means known to be unloaded.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub loads: Option<Vec<Load>>,
    /// Reported driving state; no physical behavior is verified.
    pub driving: bool,
    /// Whether the robot is paused, if reported.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub paused: Option<bool>,
    /// Whether a new base is requested, if reported.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub new_base_request: Option<bool>,
    /// Active zone requests, if reported.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub zone_requests: Option<Vec<ZoneRequest>>,
    /// Active edge requests, if reported.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub edge_requests: Option<Vec<EdgeRequest>>,
    /// Finite distance in meters; neither source asserts a nonnegative bound.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::distance_since_last_node::optional"))]
    pub distance_since_last_node: Option<f64>,
    /// Action states belonging to the active order.
    pub action_states: Vec<ActionState>,
    /// Received instant-action states.
    pub instant_action_states: Vec<ActionState>,
    /// Zone-action states; capability-dependent presence is not validated.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub zone_action_states: Option<Vec<ActionState>>,
    /// Power-supply measurements.
    pub power_supply: PowerSupply,
    /// Reported operating mode.
    pub operating_mode: OperatingMode,
    /// Active errors; empty means no active errors.
    pub errors: Vec<Error>,
    /// Visualization/debugging information, not fleet-control logic.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub information: Option<Vec<Info>>,
    /// Reported safety-related state; not a safety-control implementation.
    pub safety_state: SafetyState,
}
}

#[cfg(feature = "json")]
impl_json_helpers!(State);

object_only_deserialize! {
/// State of an order, instant or zone action.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct ActionState {
    /// Action identifier.
    pub action_id: String,
    /// Action type for information/visualization only.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub action_type: Option<String>,
    /// Human-readable descriptor.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub action_descriptor: Option<String>,
    /// Reported status; transitions are not validated.
    pub action_status: ActionStatus,
    /// Human-readable result; errors are reported separately.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub action_result: Option<String>,
}
}

string_enum! {
/// Closed action states from section 7.8.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "SCREAMING_SNAKE_CASE"))]
pub enum ActionStatus {
    /// Waiting for the trigger.
    Waiting,
    /// Preparing the triggered action.
    Initializing,
    /// Currently executing.
    Running,
    /// Execution paused.
    Paused,
    /// Failed but eligible for retry.
    Retriable,
    /// Completed successfully.
    Finished,
    /// Could not be performed.
    Failed,
}
}

string_enum! {
/// Closed operating modes from sections 6.6.6 and 7.8.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "SCREAMING_SNAKE_CASE"))]
pub enum OperatingMode {
    /// Robot is starting up.
    Startup,
    /// Automatic operation.
    Automatic,
    /// Semiautomatic operation.
    Semiautomatic,
    /// Operation has been intervened in.
    Intervened,
    /// Manual operation.
    Manual,
    /// Service mode.
    Service,
    /// Teach-in mode.
    TeachIn,
}
}

object_only_deserialize! {
/// Reported safety-related information, not a safety guarantee.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct SafetyState {
    /// Active emergency-stop type.
    pub active_emergency_stop: EmergencyStop,
    /// Whether a protective field is violated.
    pub field_violation: bool,
}
}

string_enum! {
/// Emergency-stop acknowledgment type from section 7.8.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "SCREAMING_SNAKE_CASE"))]
pub enum EmergencyStop {
    /// Must be acknowledged manually on the robot.
    Manual,
    /// Facility stop acknowledged remotely.
    Remote,
    /// No emergency stop is active.
    None,
}
}
