//! Sources pinned to VDA5050/VDA5050@0b2ae43154bcd1ab2be4e2445df8b406dcad77b2:
//! https://github.com/VDA5050/VDA5050/blob/0b2ae43154bcd1ab2be4e2445df8b406dcad77b2/VDA5050_EN.md
//! Sections 6.5 (last will), 7.1.2-7.1.4 (wire conventions), 7.2 (header),
//! and 7.7 (connection). Schema: tests/schemas/connection.schema at that revision.

use chrono::{TimeZone, Timelike, Utc};
use serde_json::{Value, json};
use vda5050_v3_types::{Connection, common::Header, connection::ConnectionState};

mod support;

const FIXTURE: &str = include_str!("fixtures/connection.json");

fn body() -> Value {
    serde_json::from_str(FIXTURE).unwrap()
}

fn schema() -> &'static jsonschema::Validator {
    support::schema("connection")
}

fn message() -> Connection {
    Connection {
        header: Header {
            header_id: 0,
            timestamp: Utc
                .with_ymd_and_hms(2017, 4, 15, 11, 40, 3)
                .unwrap()
                .with_nanosecond(123_000_000)
                .unwrap(),
            version: "3.0.0".into(),
            manufacturer: "Example".into(),
            serial_number: "robot-1".into(),
        },
        connection_state: ConnectionState::Online,
    }
}

#[test]
fn types_work_without_optional_features() {
    // Section 7.2: uint32 counter and timestamp usable directly by logic code.
    let mut value = message();
    value.header.header_id = u32::MAX;
    assert_eq!(value.clone(), value);
    assert_eq!(value.header.timestamp.timestamp_millis(), 1_492_256_403_123);
    assert_eq!(value.connection_state, ConnectionState::Online);
}

#[test]
fn fixture_and_all_four_states_satisfy_pinned_schema() {
    let validator = schema();
    for state in ["ONLINE", "OFFLINE", "HIBERNATING", "CONNECTION_BROKEN"] {
        let mut value = body();
        value["connectionState"] = json!(state);
        validator.validate(&value).unwrap();
    }
}

#[cfg(feature = "serde")]
mod wire {
    use super::*;

    #[test]
    fn all_states_round_trip_with_flat_header_and_schema_valid_output() {
        // Sections 7.2 and 7.7: root header properties and exact enum strings.
        let validator = schema();
        for (wire, state) in [
            ("ONLINE", ConnectionState::Online),
            ("OFFLINE", ConnectionState::Offline),
            ("HIBERNATING", ConnectionState::Hibernating),
            ("CONNECTION_BROKEN", ConnectionState::ConnectionBroken),
        ] {
            let mut expected = body();
            expected["connectionState"] = json!(wire);
            let decoded: Connection = serde_json::from_value(expected.clone()).unwrap();
            assert_eq!(decoded.connection_state, state);
            assert_eq!(decoded.header, message().header);
            let encoded = serde_json::to_value(&decoded).unwrap();
            assert_eq!(encoded, expected);
            assert!(encoded.get("header").is_none());
            validator.validate(&encoded).unwrap();
        }
    }

    #[test]
    fn every_required_field_rejects_omission_null_and_wrong_type() {
        // Section 7.7 and schema.required: all six fields are non-nullable.
        let validator = schema();
        support::assert_required::<Connection>(
            &body(),
            validator,
            &[
                "/headerId",
                "/timestamp",
                "/version",
                "/manufacturer",
                "/serialNumber",
                "/connectionState",
            ],
        );
    }

    #[test]
    fn header_id_enforces_specification_uint32_not_unbounded_schema_integer() {
        // Sections 7.2 and 7.7 explicitly say uint32; schema has no bounds.
        let validator = schema();
        for id in [0, u32::MAX] {
            let mut value = body();
            value["headerId"] = json!(id);
            let decoded: Connection = serde_json::from_value(value).unwrap();
            assert_eq!(decoded.header.header_id, id);
            let encoded = serde_json::to_value(decoded).unwrap();
            assert_eq!(encoded["headerId"], json!(id));
            validator.validate(&encoded).unwrap();
        }
        for id in [json!(-1), json!(u64::from(u32::MAX) + 1)] {
            let mut value = body();
            value["headerId"] = id;
            assert!(validator.is_valid(&value));
            assert!(serde_json::from_value::<Connection>(value).is_err());
        }
        for id in [json!(1.5), json!("1")] {
            let mut value = body();
            value["headerId"] = id;
            assert!(!validator.is_valid(&value));
            assert!(serde_json::from_value::<Connection>(value).is_err());
        }
        // JSON Schema integers include 1.0; our agreed integer-token policy is stricter.
        let value = FIXTURE.replacen("\"headerId\": 0", "\"headerId\": 1.0", 1);
        assert!(validator.is_valid(&serde_json::from_str::<Value>(&value).unwrap()));
        assert!(serde_json::from_str::<Connection>(&value).is_err());
    }

    #[test]
    fn timestamp_preserves_specification_example_and_calendar_boundaries() {
        // Section 7.2 example and exact millisecond UTC representation.
        let validator = schema();
        for timestamp in [
            "2017-04-15T11:40:03.123Z",
            "1970-01-01T00:00:00.000Z",
            "1969-12-31T23:59:59.999Z",
            "2000-02-29T23:59:59.999Z",
            "2026-12-31T23:59:59.000Z",
            "9999-12-31T23:59:59.999Z",
            "0000-01-01T00:00:00.000Z",
        ] {
            let mut value = body();
            value["timestamp"] = json!(timestamp);
            let decoded: Connection = serde_json::from_value(value.clone()).unwrap();
            assert_eq!(serde_json::to_value(decoded).unwrap(), value);
            validator.validate(&value).unwrap();
        }
    }

    #[test]
    fn broader_schema_timestamps_are_not_the_specification_wire_format() {
        // Section 7.2: .fffZ is stricter than the schema's RFC 3339 date-time.
        let validator = schema();
        for timestamp in [
            "2017-04-15T11:40:03Z",
            "2017-04-15T11:40:03.12Z",
            "2017-04-15T11:40:03.123456Z",
            "2017-04-15T11:40:03.123+00:00",
            "2017-04-15T13:40:03.123+02:00",
        ] {
            let mut value = body();
            value["timestamp"] = json!(timestamp);
            assert!(validator.is_valid(&value), "{timestamp}");
            assert!(
                serde_json::from_value::<Connection>(value).is_err(),
                "{timestamp}"
            );
        }
    }

    #[test]
    fn timestamp_rejects_invalid_calendar_values_and_noncanonical_spelling() {
        for timestamp in [
            "",
            "not-a-date",
            "2025-02-29T11:40:03.123Z",
            "2017-13-15T11:40:03.123Z",
            "2017-04-31T11:40:03.123Z",
            "2017-04-15T24:40:03.123Z",
            "2017-04-15T11:60:03.123Z",
            "2017-04-15 11:40:03.123Z",
            "2017-04-15t11:40:03.123z",
            "2017-04-15T11:40:03,123Z",
            "２０１７-04-15T11:40:03.123Z",
            "+10000-01-01T00:00:00.000Z",
        ] {
            let mut value = body();
            value["timestamp"] = json!(timestamp);
            assert!(
                serde_json::from_value::<Connection>(value).is_err(),
                "{timestamp}"
            );
        }
    }

    #[test]
    fn encoding_never_silently_truncates_timestamp_precision_or_year() {
        // Section 7.2: four-digit year and exactly millisecond precision.
        let mut value = message();
        value.header.timestamp = value.header.timestamp.with_nanosecond(123_000_001).unwrap();
        let error = serde_json::to_string(&value).unwrap_err();
        assert!(error.to_string().contains("sub-millisecond"));
        for year in [-1, 10_000] {
            value.header.timestamp = Utc.with_ymd_and_hms(year, 1, 1, 0, 0, 0).unwrap();
            assert!(
                serde_json::to_string(&value)
                    .unwrap_err()
                    .to_string()
                    .contains("four digits")
            );
        }
    }

    #[test]
    fn unknown_enum_values_and_incorrect_case_are_errors() {
        // Sections 7.1.3 and 7.7: case-sensitive, closed enum.
        let validator = schema();
        for state in ["online", "CONNECTIONBROKEN", "UNKNOWN", ""] {
            let mut value = body();
            value["connectionState"] = json!(state);
            assert!(!validator.is_valid(&value));
            assert!(serde_json::from_value::<Connection>(value).is_err());
        }
    }

    #[test]
    fn unknown_fields_are_ignored_not_preserved() {
        // Schema permits additional properties; this is our compatibility policy.
        let mut value = body();
        value["vendorExtension"] = json!({"nested": [null, 42]});
        schema().validate(&value).unwrap();
        let decoded: Connection = serde_json::from_value(value).unwrap();
        assert_eq!(decoded, message());
        assert_eq!(serde_json::to_value(decoded).unwrap(), body());
    }

    #[test]
    fn wire_header_is_flat_and_unicode_content_is_preserved() {
        // Sections 7.1.2 and 7.2: UTF-8, and the header is not a JSON object.
        let mut value = body();
        value["manufacturer"] = json!("Hersteller \u{00e4}\u{65e5}");
        let decoded: Connection = serde_json::from_value(value.clone()).unwrap();
        assert_eq!(serde_json::to_value(decoded).unwrap(), value);
        let nested = json!({"header": message_header_json(), "connectionState": "ONLINE"});
        assert!(serde_json::from_value::<Connection>(nested).is_err());
    }

    fn message_header_json() -> Value {
        let mut header = body();
        header.as_object_mut().unwrap().remove("connectionState");
        header
    }

    #[test]
    fn duplicate_fields_and_nonobject_bodies_are_errors() {
        for value in ["null", "[]", "42", "\"ONLINE\""] {
            assert!(serde_json::from_str::<Connection>(value).is_err());
        }
        for extra in ["\"headerId\": 1,", "\"connectionState\": \"OFFLINE\","] {
            let duplicate = FIXTURE.replacen('{', &format!("{{{extra}"), 1);
            assert!(serde_json::from_str::<Connection>(&duplicate).is_err());
        }
    }

    #[test]
    fn semantic_validation_boundary_is_explicit() {
        // Section 6.5: last wills can have old timestamps and counters.
        let mut value = body();
        value["timestamp"] = json!("1991-03-11T11:40:03.123Z");
        value["connectionState"] = json!("CONNECTION_BROKEN");
        let decoded: Connection = serde_json::from_value(value.clone()).unwrap();
        assert_eq!(serde_json::to_value(decoded).unwrap(), value);
        // Sections 7.2/7.7 describe version syntax; schema only requires a string.
        // Version/identity content validation is deliberately not implemented.
        value["version"] = json!("not-a-version");
        value["serialNumber"] = json!("");
        schema().validate(&value).unwrap();
        assert!(serde_json::from_value::<Connection>(value).is_ok());
    }
}

#[cfg(feature = "json")]
#[test]
fn json_helpers_round_trip_text_and_bytes_and_propagate_errors() {
    let decoded = Connection::from_json(FIXTURE).unwrap();
    assert_eq!(decoded, message());
    assert_eq!(
        Connection::from_json_slice(FIXTURE.as_bytes()).unwrap(),
        decoded
    );
    assert_eq!(
        serde_json::from_str::<Value>(&decoded.to_json().unwrap()).unwrap(),
        body()
    );
    let bytes = decoded.to_json_vec().unwrap();
    assert_eq!(Connection::from_json_slice(&bytes).unwrap(), decoded);
    for payload in ["{", "{}"] {
        let helper = Connection::from_json(payload).unwrap_err();
        let direct = serde_json::from_str::<Connection>(payload).unwrap_err();
        assert_eq!(helper.to_string(), direct.to_string());
        assert_eq!(helper.classify(), direct.classify());
    }
    assert!(Connection::from_json_slice(&[0xff]).is_err());
    let mut invalid = decoded;
    invalid.header.timestamp = invalid.header.timestamp.with_nanosecond(1).unwrap();
    assert!(invalid.to_json().is_err());
    assert!(invalid.to_json_vec().is_err());
}
