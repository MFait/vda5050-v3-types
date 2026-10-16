use crate::common::BlockingType;

object_only_deserialize! {
/// Supported optional parameters and action capabilities.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct ProtocolFeatures {
    /// Supported/required optional parameters.
    pub optional_parameters: Vec<OptionalParameter>,
    /// Standard and vendor-defined action capabilities.
    pub mobile_robot_actions: Vec<MobileRobotAction>,
}
}

object_only_deserialize! {
/// Support declaration for an optional protocol parameter.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct OptionalParameter {
    /// Full parameter path; content is not validated.
    pub parameter: String,
    /// Declared support.
    pub support: ParameterSupport,
    /// Free-form explanation.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub description: Option<String>,
}
}

string_enum! {
/// Support values from section 7.10.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "SCREAMING_SNAKE_CASE"))]
pub enum ParameterSupport {
    /// Supported as specified.
    Supported,
    /// Required for this robot.
    Required,
}
}

object_only_deserialize! {
/// Action capability, not an executable `common::Action`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct MobileRobotAction {
    /// Standard or vendor-defined action name.
    pub action_type: String,
    /// Free-form explanation.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub action_description: Option<String>,
    /// Allowed contexts.
    pub action_scopes: Vec<ActionScope>,
    /// Parameter definitions; absent means no parameters.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub action_parameters: Option<Vec<ActionParameter>>,
    /// Free-form result description.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub action_result: Option<String>,
    /// Supported blocking types, if reported.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub blocking_types: Option<Vec<BlockingType>>,
    /// Whether startPause can pause the action.
    pub pause_allowed: bool,
    /// Whether cancelOrder can cancel the action.
    pub cancel_allowed: bool,
}
}

string_enum! {
/// Contexts in which an action may be used.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "SCREAMING_SNAKE_CASE"))]
pub enum ActionScope {
    /// Instant action.
    Instant,
    /// Node action.
    Node,
    /// Edge action.
    Edge,
    /// Zone action.
    Zone,
}
}

object_only_deserialize! {
/// Parameter capability, distinct from `common::ActionParameter` values.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct ActionParameter {
    /// Parameter name.
    pub key: String,
    /// Permitted JSON value category.
    pub value_data_type: ValueDataType,
    /// Free-form explanation.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub description: Option<String>,
    /// Whether optional, if specified; absence is preserved.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub is_optional: Option<bool>,
}
}

string_enum! {
/// Parameter value categories from section 7.10.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "SCREAMING_SNAKE_CASE"))]
pub enum ValueDataType {
    /// JSON boolean.
    Bool,
    /// JSON number.
    Number,
    /// JSON integer.
    Integer,
    /// JSON string.
    String,
    /// JSON object.
    Object,
    /// JSON array.
    Array,
}
}
