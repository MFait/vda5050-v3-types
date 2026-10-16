object_only_deserialize! {
/// General robot classification (section 7.10).
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct TypeSpecification {
    /// Manufacturer's generalized series name.
    pub series_name: String,
    /// Human-readable series description.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub series_description: Option<String>,
    /// Extensible kinematics name, e.g. DIFFERENTIAL.
    pub mobile_robot_kinematics: String,
    /// Extensible class name, e.g. FORKLIFT.
    pub mobile_robot_class: String,
    /// Finite nonnegative maximum mass in kilograms.
    #[cfg_attr(feature = "serde", serde(with = "crate::factsheet::wire::maximum_load_mass"))]
    pub maximum_load_mass: f64,
    /// Extensible localization names.
    pub localization_types: Vec<String>,
    /// Extensible navigation names, ordered by priority.
    pub navigation_types: Vec<String>,
    /// Supported zone types, if reported.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub supported_zones: Option<Vec<ZoneType>>,
}
}

string_enum! {
/// Closed zone types listed in section 7.10.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "SCREAMING_SNAKE_CASE"))]
pub enum ZoneType {
    /// No entry.
    Blocked,
    /// Line-guided navigation.
    LineGuided,
    /// Release required.
    Release,
    /// Coordinated replanning.
    CoordinatedReplanning,
    /// Speed restriction.
    SpeedLimit,
    /// Zone actions.
    Action,
    /// Priority zone.
    Priority,
    /// Planning penalty.
    Penalty,
    /// Directed traversal.
    Directed,
    /// Bidirectional traversal.
    Bidirected,
}
}

object_only_deserialize! {
/// Physical properties; relationships between limits are not checked.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct PhysicalParameters {
    /// Finite nonnegative minimum continuous speed in m/s.
    #[cfg_attr(feature = "serde", serde(with = "crate::factsheet::wire::minimum_speed"))]
    pub minimum_speed: f64,
    /// Finite nonnegative maximum speed in m/s.
    #[cfg_attr(feature = "serde", serde(with = "crate::factsheet::wire::maximum_speed"))]
    pub maximum_speed: f64,
    /// Finite nonnegative minimum angular speed in rad/s.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", with = "crate::factsheet::wire::minimum_angular_speed::optional"))]
    pub minimum_angular_speed: Option<f64>,
    /// Finite nonnegative maximum angular speed in rad/s.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", with = "crate::factsheet::wire::maximum_angular_speed::optional"))]
    pub maximum_angular_speed: Option<f64>,
    /// Finite nonnegative acceleration in m/s squared.
    #[cfg_attr(feature = "serde", serde(with = "crate::factsheet::wire::maximum_acceleration"))]
    pub maximum_acceleration: f64,
    /// Finite deceleration in m/s squared; no explicit sign bound.
    #[cfg_attr(feature = "serde", serde(with = "crate::factsheet::wire::maximum_deceleration"))]
    pub maximum_deceleration: f64,
    /// Finite minimum height in meters.
    #[cfg_attr(feature = "serde", serde(with = "crate::factsheet::wire::minimum_height"))]
    pub minimum_height: f64,
    /// Finite maximum height in meters.
    #[cfg_attr(feature = "serde", serde(with = "crate::factsheet::wire::maximum_height"))]
    pub maximum_height: f64,
    /// Finite width in meters.
    #[cfg_attr(feature = "serde", serde(with = "crate::factsheet::wire::width"))]
    pub width: f64,
    /// Finite length in meters.
    #[cfg_attr(feature = "serde", serde(with = "crate::factsheet::wire::length"))]
    pub length: f64,
}
}
