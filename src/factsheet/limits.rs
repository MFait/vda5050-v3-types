object_only_deserialize! {
/// Protocol limits. Omitted or zero limits mean no explicit limit.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct ProtocolLimits {
    /// String/message length limits.
    pub maximum_string_lengths: MaximumStringLengths,
    /// Collection size limits.
    pub maximum_array_lengths: MaximumArrayLengths,
    /// Message timing.
    pub timing: Timing,
}
}

object_only_deserialize! {
/// Optional uint32 length limits; no limits are inserted when absent.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct MaximumStringLengths {
    /// Maximum message length.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub maximum_message_length: Option<u32>,
    /// Maximum topic serial-number length.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub maximum_topic_serial_length: Option<u32>,
    /// Maximum other topic-element length.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub maximum_topic_element_length: Option<u32>,
    /// Maximum identifier length.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub maximum_id_length: Option<u32>,
    /// Whether identifiers must be numerical; content of messages is not checked.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub id_numerical_only: Option<bool>,
    /// Maximum load-identifier length.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub maximum_load_id_length: Option<u32>,
}
}

object_only_deserialize! {
/// Optional uint32 collection limits; dotted wire keys are literal properties.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct MaximumArrayLengths {
    /// Maximum order nodes.
    #[cfg_attr(feature = "serde", serde(rename = "order.nodes", default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub order_nodes: Option<u32>,
    /// Maximum order edges.
    #[cfg_attr(feature = "serde", serde(rename = "order.edges", default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub order_edges: Option<u32>,
    /// Maximum actions per node.
    #[cfg_attr(feature = "serde", serde(rename = "node.actions", default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub node_actions: Option<u32>,
    /// Maximum actions per edge.
    #[cfg_attr(feature = "serde", serde(rename = "edge.actions", default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub edge_actions: Option<u32>,
    /// Maximum parameters per action; preserves the sources' plural spelling.
    #[cfg_attr(feature = "serde", serde(rename = "actions.actionsParameters", default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub actions_parameters: Option<u32>,
    /// Maximum instant actions per message.
    #[cfg_attr(feature = "serde", serde(rename = "instantActions", default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub instant_actions: Option<u32>,
    /// Maximum trajectory knots.
    #[cfg_attr(feature = "serde", serde(rename = "trajectory.knotVector", default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub trajectory_knot_vector: Option<u32>,
    /// Maximum trajectory control points.
    #[cfg_attr(feature = "serde", serde(rename = "trajectory.controlPoints", default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub trajectory_control_points: Option<u32>,
    /// Maximum zones per zone set.
    #[cfg_attr(feature = "serde", serde(rename = "zoneSet.zones", default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub zone_set_zones: Option<u32>,
    /// Maximum reported node states.
    #[cfg_attr(feature = "serde", serde(rename = "state.nodeStates", default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub state_node_states: Option<u32>,
    /// Maximum reported edge states.
    #[cfg_attr(feature = "serde", serde(rename = "state.edgeStates", default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub state_edge_states: Option<u32>,
    /// Maximum reported loads.
    #[cfg_attr(feature = "serde", serde(rename = "state.loads", default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub state_loads: Option<u32>,
    /// Maximum reported order action states.
    #[cfg_attr(feature = "serde", serde(rename = "state.actionStates", default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub state_action_states: Option<u32>,
    /// Maximum reported instant-action states.
    #[cfg_attr(feature = "serde", serde(rename = "state.instantActionStates", default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub state_instant_action_states: Option<u32>,
    /// Maximum reported zone-action states.
    #[cfg_attr(feature = "serde", serde(rename = "state.zoneActionStates", default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub state_zone_action_states: Option<u32>,
    /// Maximum reported errors.
    #[cfg_attr(feature = "serde", serde(rename = "state.errors", default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub state_errors: Option<u32>,
    /// Maximum reported information entries.
    #[cfg_attr(feature = "serde", serde(rename = "state.information", default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub state_information: Option<u32>,
    /// Maximum references per error.
    #[cfg_attr(feature = "serde", serde(rename = "error.errorReferences", default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub error_references: Option<u32>,
    /// Maximum references per information entry.
    #[cfg_attr(feature = "serde", serde(rename = "information.infoReferences", default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub information_references: Option<u32>,
}
}

object_only_deserialize! {
/// Message timing in seconds, specified as float32.
///
/// Values must be finite and nonnegative. Omitted defaults are not inserted.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct Timing {
    /// Minimum order-message interval.
    #[cfg_attr(feature = "serde", serde(with = "crate::factsheet::wire::minimum_order_interval"))]
    pub minimum_order_interval: f32,
    /// Minimum state-message interval.
    #[cfg_attr(feature = "serde", serde(with = "crate::factsheet::wire::minimum_state_interval"))]
    pub minimum_state_interval: f32,
    /// Default state-message interval, if specified.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", with = "crate::factsheet::wire::default_state_interval::optional"))]
    pub default_state_interval: Option<f32>,
    /// Visualization-message interval, if specified.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", with = "crate::factsheet::wire::visualization_interval::optional"))]
    pub visualization_interval: Option<f32>,
}
}
