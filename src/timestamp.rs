use chrono::{DateTime, Datelike, NaiveDateTime, SecondsFormat, Utc};
use serde::{Deserialize, Deserializer, Serializer, de::Error as _, ser::Error as _};

const FORMAT_ERROR: &str = "expected UTC timestamp YYYY-MM-DDTHH:mm:ss.fffZ (VDA 5050 section 7.2)";

optional_adapter!(chrono::DateTime<chrono::Utc>);

pub(crate) fn serialize<S>(value: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if !(0..=9999).contains(&value.year()) {
        return Err(S::Error::custom("timestamp year must fit four digits"));
    }
    if value.timestamp_subsec_nanos() % 1_000_000 != 0 {
        return Err(S::Error::custom(
            "timestamp has sub-millisecond precision; refusing to truncate",
        ));
    }
    serializer.serialize_str(&value.to_rfc3339_opts(SecondsFormat::Millis, true))
}

pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = String::deserialize(deserializer)?;
    let bytes = value.as_bytes();
    if bytes.len() != 24
        || !bytes.iter().enumerate().all(|(index, byte)| match index {
            4 | 7 => *byte == b'-',
            10 => *byte == b'T',
            13 | 16 => *byte == b':',
            19 => *byte == b'.',
            23 => *byte == b'Z',
            _ => byte.is_ascii_digit(),
        })
    {
        return Err(D::Error::custom(FORMAT_ERROR));
    }
    NaiveDateTime::parse_from_str(&value, "%Y-%m-%dT%H:%M:%S%.3fZ")
        .map(|value| value.and_utc())
        .map_err(D::Error::custom)
}
