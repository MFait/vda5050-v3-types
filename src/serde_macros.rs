macro_rules! optional_adapter {
    ($value:ty) => {
        pub(crate) mod optional {
            pub(crate) fn serialize<S>(
                value: &Option<$value>,
                serializer: S,
            ) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                match value {
                    Some(value) => super::serialize(value, serializer),
                    None => serializer.serialize_none(),
                }
            }

            pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Option<$value>, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                super::deserialize(deserializer).map(Some)
            }
        }
    };
}

macro_rules! field_adapter {
    ($name:ident, $adapter:ident, $field:literal $(, $optional:ident)?) => {
        pub(crate) mod $name {
            use serde::{Deserializer, Serializer, de::Error as _, ser::Error as _};

            pub(crate) fn serialize<S>(value: &f64, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                super::$adapter::serialize(value, serializer)
                    .map_err(|error| S::Error::custom(format_args!("{}: {error}", $field)))
            }

            pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<f64, D::Error>
            where
                D: Deserializer<'de>,
            {
                super::$adapter::deserialize(deserializer)
                    .map_err(|error| D::Error::custom(format_args!("{}: {error}", $field)))
            }

            $($optional!(f64);)?
        }
    };
}
