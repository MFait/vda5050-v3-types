use crate::common::{BoundingBoxReference, LoadDimensions};

object_only_deserialize! {
/// Supported load positions and load sets.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct LoadSpecification {
    /// Absence/empty means no load handling device; omission is preserved.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub load_positions: Option<Vec<String>>,
    /// Load capabilities, if reported.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub load_sets: Option<Vec<LoadSet>>,
}
}

object_only_deserialize! {
/// Capability for a named load set; cross-field relationships are unchecked.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct LoadSet {
    /// Unique set name; uniqueness is not validated.
    pub set_name: String,
    /// Load type.
    pub load_type: String,
    /// Applicable positions; absent/empty means all devices.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub load_positions: Option<Vec<String>>,
    /// Same bounding-box reference model as state loads.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub bounding_box_reference: Option<BoundingBoxReference>,
    /// Same nonnegative dimension model as state loads (section 7.10 reference).
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub load_dimensions: Option<LoadDimensions>,
    /// Finite nonnegative maximum mass in kilograms.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", with = "crate::factsheet::wire::maximum_weight::optional"))]
    pub maximum_weight: Option<f64>,
    /// Finite nonnegative minimum handling height in meters.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", with = "crate::factsheet::wire::minimum_loadhandling_height::optional"))]
    pub minimum_loadhandling_height: Option<f64>,
    /// Finite nonnegative maximum handling height in meters.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", with = "crate::factsheet::wire::maximum_loadhandling_height::optional"))]
    pub maximum_loadhandling_height: Option<f64>,
    /// Finite minimum handling depth in meters.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", with = "crate::factsheet::wire::minimum_loadhandling_depth::optional"))]
    pub minimum_loadhandling_depth: Option<f64>,
    /// Finite maximum handling depth in meters.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", with = "crate::factsheet::wire::maximum_loadhandling_depth::optional"))]
    pub maximum_loadhandling_depth: Option<f64>,
    /// Finite minimum tilt in radians; no explicit angle bound.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", with = "crate::factsheet::wire::minimum_loadhandling_tilt::optional"))]
    pub minimum_loadhandling_tilt: Option<f64>,
    /// Finite maximum tilt in radians; no explicit angle bound.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", with = "crate::factsheet::wire::maximum_loadhandling_tilt::optional"))]
    pub maximum_loadhandling_tilt: Option<f64>,
    /// Finite nonnegative maximum speed in m/s.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", with = "crate::factsheet::wire::maximum_speed::optional"))]
    pub maximum_speed: Option<f64>,
    /// Finite nonnegative maximum acceleration in m/s squared.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", with = "crate::factsheet::wire::maximum_acceleration::optional"))]
    pub maximum_acceleration: Option<f64>,
    /// Finite maximum deceleration in m/s squared.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", with = "crate::factsheet::wire::maximum_deceleration::optional"))]
    pub maximum_deceleration: Option<f64>,
    /// Finite nonnegative approximate pick time in seconds.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", with = "crate::factsheet::wire::pick_time::optional"))]
    pub pick_time: Option<f64>,
    /// Finite nonnegative approximate drop time in seconds.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", with = "crate::factsheet::wire::drop_time::optional"))]
    pub drop_time: Option<f64>,
    /// Free-form explanation.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub description: Option<String>,
}
}
