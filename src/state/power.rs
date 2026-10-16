object_only_deserialize! {
/// Power-supply measurements (section 7.8).
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct PowerSupply {
    /// Finite charge percentage in `[0, 100]`.
    #[cfg_attr(feature = "serde", serde(with = "crate::serde_helpers::state_of_charge"))]
    pub state_of_charge: f64,
    /// Finite battery voltage in volts.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::battery_voltage::optional"))]
    pub battery_voltage: Option<f64>,
    /// Finite battery current in amperes; may be signed.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::battery_current::optional"))]
    pub battery_current: Option<f64>,
    /// Health percentage: specification int8, restricted to `[0, 100]`.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::battery_health::optional"))]
    pub battery_health: Option<i8>,
    /// Whether charging is in progress; availability semantics are not checked.
    pub charging: bool,
    /// Estimated distance in meters, specified as uint32.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub range: Option<u32>,
}
}
