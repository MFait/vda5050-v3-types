object_only_deserialize! {
/// Current software/hardware and network configuration.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct MobileRobotConfiguration {
    /// Version entries, if reported.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub versions: Option<Vec<VersionInfo>>,
    /// Network configuration, if reported.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub network: Option<Network>,
    /// Charging preferences, if reported.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub battery_charging: Option<BatteryCharging>,
}
}

object_only_deserialize! {
/// Hardware or software version entry.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct VersionInfo {
    /// Component/version key.
    pub key: String,
    /// Version string.
    pub value: String,
}
}

object_only_deserialize! {
/// Network configuration strings; addresses are not parsed or contacted.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct Network {
    /// DNS servers.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub dns_servers: Option<Vec<String>>,
    /// NTP servers.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub ntp_servers: Option<Vec<String>>,
    /// Assigned local address.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub local_ip_address: Option<String>,
    /// Subnet mask.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub netmask: Option<String>,
    /// Default gateway.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub default_gateway: Option<String>,
}
}

object_only_deserialize! {
/// Charging preferences; relationships between thresholds are not validated.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct BatteryCharging {
    /// Finite critical percentage in `[0, 100]`.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", with = "crate::factsheet::wire::critical_low_charging_level::optional"))]
    pub critical_low_charging_level: Option<f64>,
    /// Finite desired minimum percentage in `[0, 100]`.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", with = "crate::factsheet::wire::minimum_desired_charging_level::optional"))]
    pub minimum_desired_charging_level: Option<f64>,
    /// Finite desired maximum percentage in `[0, 100]`.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", with = "crate::factsheet::wire::maximum_desired_charging_level::optional"))]
    pub maximum_desired_charging_level: Option<f64>,
    /// Desired minimum seconds, specified as uint32 despite schema number type.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub minimum_charging_time: Option<u32>,
}
}
