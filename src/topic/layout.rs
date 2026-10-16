use super::{TopicKind, UnknownTopicKind};
use std::{fmt, str::FromStr};

/// A level in the suggested five-level topic layout.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopicLevel {
    /// Configurable interface name, e.g. `vda5050`.
    InterfaceName,
    /// Protocol major version; exactly `v3` for this helper.
    MajorVersion,
    /// Robot manufacturer.
    Manufacturer,
    /// Robot serial number.
    SerialNumber,
    /// Exact message-kind name.
    Kind,
}

impl fmt::Display for TopicLevel {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::InterfaceName => "interfaceName",
            Self::MajorVersion => "majorVersion",
            Self::Manufacturer => "manufacturer",
            Self::SerialNumber => "serialNumber",
            Self::Kind => "topic",
        })
    }
}

/// Failure to construct or parse a suggested-layout v3 topic or filter.
///
/// Errors identify the failed rule without retaining the input topic.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TopicError {
    /// A complete topic or filter must have exactly five levels.
    InvalidLevelCount,
    /// The named level must not be empty (helper policy).
    EmptyLevel(TopicLevel),
    /// A level contains `/`, `+`, `#`, `$` or NUL.
    ForbiddenCharacter {
        /// Level containing the character.
        level: TopicLevel,
        /// Rejected character.
        character: char,
    },
    /// A serial-number character is outside ASCII letters, digits, `_.:-`.
    InvalidSerialNumber {
        /// Rejected character.
        character: char,
    },
    /// The major-version level is not exactly `v3` (helper policy).
    UnsupportedMajorVersion,
    /// The last level is not an exact supported topic name.
    UnknownKind(UnknownTopicKind),
    /// The complete UTF-8 topic or filter exceeds MQTT's 65,535-byte string limit.
    TooLong,
}

impl fmt::Display for TopicError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLevelCount => formatter.write_str("expected exactly five topic levels"),
            Self::EmptyLevel(level) => write!(formatter, "{level}: expected a nonempty level"),
            Self::ForbiddenCharacter { level, character } => {
                write!(formatter, "{level}: forbidden character {character:?}")
            }
            Self::InvalidSerialNumber { character } => {
                write!(formatter, "serialNumber: invalid character {character:?}")
            }
            Self::UnsupportedMajorVersion => formatter.write_str("majorVersion: expected v3"),
            Self::UnknownKind(error) => write!(formatter, "topic: {error}"),
            Self::TooLong => formatter.write_str("topic exceeds 65535 UTF-8 bytes"),
        }
    }
}

impl std::error::Error for TopicError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::UnknownKind(error) => Some(error),
            _ => None,
        }
    }
}

/// Checked topic in the suggested `interface/v3/manufacturer/serial/kind` layout.
///
/// Construction and parsing require exactly five nonempty levels, reject `/`,
/// `+`, `#`, `$` and NUL within levels, enforce section 4.2's serial alphabet,
/// and limit the complete topic to 65,535 UTF-8 bytes. Other text is preserved
/// without trimming, normalization or escaping.
///
/// Nonempty levels, rejection of the discouraged `+`, `#`, `$`, and restriction
/// to `v3` are deliberate helper policies. No robot identity/header consistency
/// or deployment-specific hierarchy is validated. This is not a filter.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "String", into = "String"))]
pub struct Topic {
    interface_name: String,
    manufacturer: String,
    serial_number: String,
    kind: TopicKind,
}

impl Topic {
    /// Construct a checked topic; the major-version level is always `v3`.
    pub fn new(
        interface_name: impl Into<String>,
        manufacturer: impl Into<String>,
        serial_number: impl Into<String>,
        kind: TopicKind,
    ) -> Result<Self, TopicError> {
        let topic = Self {
            interface_name: interface_name.into(),
            manufacturer: manufacturer.into(),
            serial_number: serial_number.into(),
            kind,
        };
        check_level(&topic.interface_name, TopicLevel::InterfaceName)?;
        check_level(&topic.manufacturer, TopicLevel::Manufacturer)?;
        check_level(&topic.serial_number, TopicLevel::SerialNumber)?;
        check_serial(&topic.serial_number)?;
        let length = topic.interface_name.len()
            + topic.manufacturer.len()
            + topic.serial_number.len()
            + topic.kind.as_str().len()
            + topic.major_version().len()
            + 4; // separators
        if length > u16::MAX as usize {
            return Err(TopicError::TooLong);
        }
        Ok(topic)
    }

    /// Configured interface name.
    pub fn interface_name(&self) -> &str {
        &self.interface_name
    }

    /// Protocol major-version level, always `v3`.
    pub const fn major_version(&self) -> &'static str {
        "v3"
    }

    /// Robot manufacturer, exactly as supplied.
    pub fn manufacturer(&self) -> &str {
        &self.manufacturer
    }

    /// Robot serial number.
    pub fn serial_number(&self) -> &str {
        &self.serial_number
    }

    /// Message kind.
    pub const fn kind(&self) -> TopicKind {
        self.kind
    }
}

pub(super) fn check_serial(value: &str) -> Result<(), TopicError> {
    for character in value.chars() {
        if !character.is_ascii_alphanumeric() && !matches!(character, '_' | '.' | ':' | '-') {
            return Err(TopicError::InvalidSerialNumber { character });
        }
    }
    Ok(())
}

pub(super) fn check_level(value: &str, level: TopicLevel) -> Result<(), TopicError> {
    if value.is_empty() {
        return Err(TopicError::EmptyLevel(level));
    }
    if let Some(character) = value
        .chars()
        .find(|c| matches!(c, '/' | '+' | '#' | '$' | '\0'))
    {
        return Err(TopicError::ForbiddenCharacter { level, character });
    }
    Ok(())
}

impl fmt::Display for Topic {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}/v3/{}/{}/{}",
            self.interface_name, self.manufacturer, self.serial_number, self.kind
        )
    }
}

impl FromStr for Topic {
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
        check_level(kind, TopicLevel::Kind)?;
        let kind = kind.parse().map_err(TopicError::UnknownKind)?;
        Self::new(interface, manufacturer, serial, kind)
    }
}

impl TryFrom<String> for Topic {
    type Error = TopicError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl From<Topic> for String {
    fn from(topic: Topic) -> Self {
        topic.to_string()
    }
}
