use crate::common::{BoundingBoxReference, LoadDimensions};

object_only_deserialize! {
/// Reported load information. Every field is independently optional.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct Load {
    /// Load identifier; empty means identification supported but not yet known.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub load_id: Option<String>,
    /// Load type.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub load_type: Option<String>,
    /// Robot-specific carrying position.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub load_position: Option<String>,
    /// Reference point of the load's bounding box.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub bounding_box_reference: Option<BoundingBoxReference>,
    /// Dimensions in meters.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub load_dimensions: Option<LoadDimensions>,
    /// Finite nonnegative mass in kilograms.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::load_weight::optional"))]
    pub weight: Option<f64>,
}
}
