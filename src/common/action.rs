object_only_deserialize! {
/// Action shared by orders and instant actions (section 7.3).
///
/// Zone sets use [`crate::zone_set::ZoneAction`] without a sender-supplied action
/// ID, sharing this type's blocking enum and parameter models instead.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct Action {
    /// Function name, including vendor-defined actions; not a closed enum.
    pub action_type: String,
    /// Unique action identifier. UUIDs are suggested, not required.
    pub action_id: String,
    /// Optional human-readable information.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub action_descriptor: Option<String>,
    /// Execution blocking behavior; instant actions must use [`BlockingType::None`].
    pub blocking_type: BlockingType,
    /// Optional parameters. An absent list is distinct from an empty list.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub action_parameters: Option<Vec<ActionParameter>>,
    /// Whether failure may enter RETRIABLE. The protocol default is false.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub retriable: Option<bool>,
}
}

string_enum! {
/// Exact blocking values defined in section 7.3.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "SCREAMING_SNAKE_CASE"))]
pub enum BlockingType {
    /// Driving and other actions are allowed.
    None,
    /// Driving is allowed, but other actions are not.
    Single,
    /// Other actions are allowed, but driving is not.
    Soft,
    /// Neither driving nor other actions are allowed.
    Hard,
}
}

object_only_deserialize! {
/// Generic key/value action parameter (section 7.3.1).
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ActionParameter {
    /// Parameter name.
    pub key: String,
    /// Any JSON value except top-level null.
    pub value: ActionParameterValue,
}
}

/// JSON action value with checked, non-null construction (section 7.3.1).
///
/// Arrays/objects may contain null because their contents are unconstrained by
/// the schema. No mutable access is provided that could replace the root by null.
#[derive(Debug, Clone, PartialEq)]
pub struct ActionParameterValue(serde_json::Value);

/// Top-level null is not an allowed action parameter value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NullParameterValue;

impl std::fmt::Display for NullParameterValue {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("action parameter value must not be null")
    }
}

impl std::error::Error for NullParameterValue {}

impl TryFrom<serde_json::Value> for ActionParameterValue {
    type Error = NullParameterValue;

    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        if value.is_null() {
            Err(NullParameterValue)
        } else {
            Ok(Self(value))
        }
    }
}

impl ActionParameterValue {
    /// Read the underlying JSON value without permitting invariant violations.
    pub fn as_json(&self) -> &serde_json::Value {
        &self.0
    }

    /// Consume the wrapper and return its JSON value.
    pub fn into_json(self) -> serde_json::Value {
        self.0
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for ActionParameterValue {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for ActionParameterValue {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::Error as _;
        Self::try_from(serde_json::Value::deserialize(deserializer)?).map_err(D::Error::custom)
    }
}
