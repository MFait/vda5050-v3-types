object_only_deserialize! {
/// Reported robot error, not a Rust parsing error.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct Error {
    /// Extensible error type; vendor-defined strings are preserved.
    pub error_type: String,
    /// References providing additional context.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub error_references: Option<Vec<Reference>>,
    /// Human-readable description.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub error_description: Option<String>,
    /// Translated descriptions; language-code semantics are not checked.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub error_description_translations: Option<Vec<Translation>>,
    /// Suggested resolution.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub error_hint: Option<String>,
    /// Translated hints.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub error_hint_translations: Option<Vec<Translation>>,
    /// Reported severity.
    pub error_level: ErrorLevel,
}
}

string_enum! {
/// Closed error severity values from section 7.8.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "SCREAMING_SNAKE_CASE"))]
pub enum ErrorLevel {
    /// No immediate attention required; active order can continue.
    Warning,
    /// Immediate attention required; active order can continue.
    Urgent,
    /// Active order cannot continue, but a new order can be accepted.
    Critical,
    /// User intervention required; active/new orders cannot be processed.
    Fatal,
}
}

object_only_deserialize! {
/// Key/value reference shared by errorReference and infoReference.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct Reference {
    /// Type of reference, such as orderId or actionId.
    pub reference_key: String,
    /// Value of the reference.
    pub reference_value: String,
}
}

object_only_deserialize! {
/// Text translated into the indicated language.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct Translation {
    /// ISO 639-1 language code; content is not validated.
    pub translation_key: String,
    /// Translated text.
    pub translation_value: String,
}
}

object_only_deserialize! {
/// Information for visualization/debugging, not fleet-control logic.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct Info {
    /// Open information type/name.
    pub info_type: String,
    /// Additional references.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub info_references: Option<Vec<Reference>>,
    /// Human-readable descriptor.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub info_descriptor: Option<String>,
    /// Intended display/debugging level.
    pub info_level: InfoLevel,
}
}

string_enum! {
/// Closed information levels from section 7.8.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "SCREAMING_SNAKE_CASE"))]
pub enum InfoLevel {
    /// Debugging information.
    Debug,
    /// Visualization information.
    Info,
}
}
