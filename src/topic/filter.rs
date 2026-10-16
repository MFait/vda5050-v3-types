use super::{
    Topic, TopicError, TopicKind, TopicLevel,
    layout::{check_level, check_serial},
};
use std::{fmt, str::FromStr};

/// Checked five-level subscription filter with whole-level `+` selectors.
///
/// Interface is explicit and version is always `v3`. Manufacturer, serial and
/// kind can be exact or `+`. Exact values use the same rules as [`Topic`].
/// `#`, wildcard interface/version, partial wildcards, shared-subscription
/// prefixes and arbitrary broker layouts are unsupported helper policies.
///
/// This type constructs filter text only: it performs no matching, network or
/// broker operations. With Serde enabled it uses a checked string representation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "String", into = "String"))]
pub struct TopicFilter {
    interface_name: String,
    manufacturer: Option<String>,
    serial_number: Option<String>,
    kind: Option<TopicKind>,
}

impl TopicFilter {
    /// Construct a filter; `None` selects `+` at that level.
    ///
    /// `Some` values must be valid exact levels, not literal wildcards.
    /// The complete filter must fit MQTT's 65,535-byte UTF-8 string limit.
    pub fn new(
        interface: impl Into<String>,
        manufacturer: Option<&str>,
        serial_number: Option<&str>,
        kind: Option<TopicKind>,
    ) -> Result<Self, TopicError> {
        let interface_name = interface.into();
        check_level(&interface_name, TopicLevel::InterfaceName)?;
        if let Some(value) = manufacturer {
            check_level(value, TopicLevel::Manufacturer)?;
        }
        if let Some(value) = serial_number {
            check_level(value, TopicLevel::SerialNumber)?;
            check_serial(value)?;
        }
        let length = interface_name.len()
            + manufacturer.unwrap_or("+").len()
            + serial_number.unwrap_or("+").len()
            + kind.map_or("+", TopicKind::as_str).len()
            + "v3".len()
            + 4;
        if length > u16::MAX as usize {
            return Err(TopicError::TooLong);
        }
        Ok(Self {
            interface_name,
            manufacturer: manufacturer.map(str::to_owned),
            serial_number: serial_number.map(str::to_owned),
            kind,
        })
    }

    /// Explicit interface name.
    pub fn interface_name(&self) -> &str {
        &self.interface_name
    }

    /// Fixed protocol major-version level.
    pub const fn major_version(&self) -> &'static str {
        "v3"
    }

    /// Exact manufacturer, or `None` for `+`.
    pub fn manufacturer(&self) -> Option<&str> {
        self.manufacturer.as_deref()
    }

    /// Exact serial number, or `None` for `+`.
    pub fn serial_number(&self) -> Option<&str> {
        self.serial_number.as_deref()
    }

    /// Exact message kind, or `None` for `+`.
    pub const fn kind(&self) -> Option<TopicKind> {
        self.kind
    }
}

impl fmt::Display for TopicFilter {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}/v3/{}/{}/{}",
            self.interface_name,
            self.manufacturer().unwrap_or("+"),
            self.serial_number().unwrap_or("+"),
            self.kind.map_or("+", TopicKind::as_str)
        )
    }
}

impl FromStr for TopicFilter {
    type Err = TopicError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.len() > u16::MAX as usize {
            return Err(TopicError::TooLong);
        }
        let mut levels = value.split('/');
        let [
            Some(interface),
            Some(version),
            Some(manufacturer),
            Some(serial),
            Some(kind),
        ] = std::array::from_fn(|_| levels.next())
        else {
            return Err(TopicError::InvalidLevelCount);
        };
        if levels.next().is_some() {
            return Err(TopicError::InvalidLevelCount);
        }
        check_level(version, TopicLevel::MajorVersion)?;
        if version != "v3" {
            return Err(TopicError::UnsupportedMajorVersion);
        }
        let kind = if kind == "+" {
            None
        } else {
            check_level(kind, TopicLevel::Kind)?;
            Some(kind.parse().map_err(TopicError::UnknownKind)?)
        };
        Self::new(
            interface,
            (manufacturer != "+").then_some(manufacturer),
            (serial != "+").then_some(serial),
            kind,
        )
    }
}

impl From<&Topic> for TopicFilter {
    fn from(topic: &Topic) -> Self {
        Self {
            interface_name: topic.interface_name().to_owned(),
            manufacturer: Some(topic.manufacturer().to_owned()),
            serial_number: Some(topic.serial_number().to_owned()),
            kind: Some(topic.kind()),
        }
    }
}

impl TryFrom<String> for TopicFilter {
    type Error = TopicError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl From<TopicFilter> for String {
    fn from(filter: TopicFilter) -> Self {
        filter.to_string()
    }
}
