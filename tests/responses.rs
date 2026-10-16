//! VDA5050/VDA5050@0b2ae43154bcd1ab2be4e2445df8b406dcad77b2:
//! https://github.com/VDA5050/VDA5050/blob/0b2ae43154bcd1ab2be4e2445df8b406dcad77b2/VDA5050_EN.md
//! Sections 7.2 and 7.5; unchanged responses.schema at the same revision.

use chrono::{TimeZone, Utc};
use serde_json::{Value, json};
use vda5050_v3_types::{
    Responses,
    common::Header,
    responses::{GrantType, Response},
};

mod support;

const FIXTURE: &str = include_str!("fixtures/responses.json");

fn body() -> Value {
    serde_json::from_str(FIXTURE).unwrap()
}

#[test]
fn types_work_without_optional_features() {
    let message = Responses {
        header: Header {
            header_id: 0,
            timestamp: Utc.with_ymd_and_hms(2026, 10, 16, 12, 0, 0).unwrap(),
            version: "3.0.0".into(),
            manufacturer: "Example".into(),
            serial_number: "robot-1".into(),
        },
        responses: vec![Response {
            request_id: "request-1".into(),
            grant_type: GrantType::Granted,
            lease_expiry: None,
        }],
    };
    assert_eq!(message.clone(), message);
}

#[test]
fn full_and_empty_fixtures_satisfy_pinned_schema() {
    let mut empty = body();
    empty["responses"] = json!([]);
    for value in [body(), empty] {
        support::schema("responses").validate(&value).unwrap();
    }
}

#[cfg(feature = "serde")]
mod wire {
    use super::*;
    use chrono::{DateTime, Timelike};

    fn round_trip(value: Value) -> Responses {
        support::schema("responses").validate(&value).unwrap();
        let decoded: Responses = serde_json::from_str(&value.to_string()).unwrap();
        let encoded: Value =
            serde_json::from_slice(&serde_json::to_vec(&decoded).unwrap()).unwrap();
        assert_eq!(encoded, value);
        support::schema("responses").validate(&encoded).unwrap();
        decoded
    }

    #[test]
    fn all_decisions_round_trip_with_exact_wire_names_and_flat_header() {
        let decoded = round_trip(body());
        for (response, decision) in decoded.responses.iter().zip([
            GrantType::Granted,
            GrantType::Queued,
            GrantType::Revoked,
            GrantType::Rejected,
        ]) {
            assert_eq!(response.grant_type, decision);
        }
        assert!(decoded.responses[0].lease_expiry.is_some());
        assert!(decoded.responses[1].lease_expiry.is_none());
        assert!(
            serde_json::to_value(decoded)
                .unwrap()
                .get("header")
                .is_none()
        );
        let mut empty = body();
        empty["responses"] = json!([]);
        assert!(round_trip(empty).responses.is_empty());
    }

    #[test]
    fn required_optional_and_null_rules_follow_schema() {
        support::assert_required::<Responses>(
            &body(),
            support::schema("responses"),
            &[
                "/headerId",
                "/timestamp",
                "/version",
                "/manufacturer",
                "/serialNumber",
                "/responses",
                "/responses/0/requestId",
                "/responses/0/grantType",
            ],
        );
        support::assert_optional::<Responses>(
            &body(),
            support::schema("responses"),
            &["/responses/0/leaseExpiry"],
        );
        let mut wrong = body();
        wrong["responses"][0]["leaseExpiry"] = json!(42);
        assert!(!support::schema("responses").is_valid(&wrong));
        assert!(serde_json::from_value::<Responses>(wrong).is_err());
    }

    #[test]
    fn unknown_decisions_are_rejected_standalone_and_nested() {
        for invalid in ["granted", "UNKNOWN", ""] {
            assert!(serde_json::from_value::<GrantType>(json!(invalid)).is_err());
            let mut value = body();
            value["responses"][0]["grantType"] = json!(invalid);
            assert!(!support::schema("responses").is_valid(&value));
            assert!(serde_json::from_value::<Responses>(value).is_err());
        }
    }

    #[test]
    fn objects_reject_positional_arrays_and_duplicates() {
        for invalid in [json!([]), json!([1]), Value::Null, json!(false)] {
            assert!(serde_json::from_value::<Responses>(invalid.clone()).is_err());
            assert!(serde_json::from_value::<Response>(invalid.clone()).is_err());
            let mut value = body();
            value["responses"][0] = invalid;
            assert!(!support::schema("responses").is_valid(&value));
            assert!(serde_json::from_value::<Responses>(value).is_err());
        }
        for duplicate in [
            r#"{"requestId":"a","requestId":"b","grantType":"GRANTED"}"#,
            r#"{"requestId":"a","grantType":"GRANTED","grantType":"QUEUED"}"#,
            r#"{"requestId":"a","grantType":"GRANTED","leaseExpiry":"2017-04-15T11:40:03.123Z","leaseExpiry":"2017-04-15T11:40:03.123Z"}"#,
        ] {
            assert!(serde_json::from_str::<Response>(duplicate).is_err());
        }
        let duplicate =
            FIXTURE.replacen("\"responses\": [", "\"responses\": [], \"responses\": [", 1);
        assert!(serde_json::from_str::<Responses>(&duplicate).is_err());
    }

    #[test]
    fn unknown_fields_are_ignored_not_preserved() {
        let expected = body();
        let mut value = expected.clone();
        value["vendor"] = json!({"enabled": true});
        value["responses"][0]["vendor"] = json!(42);
        let decoded: Responses = serde_json::from_value(value).unwrap();
        assert_eq!(serde_json::to_value(decoded).unwrap(), expected);
    }

    #[test]
    fn header_id_enforces_uint32_width() {
        for valid in [0, u32::MAX] {
            let mut value = body();
            value["headerId"] = json!(valid);
            assert_eq!(round_trip(value).header.header_id, valid);
        }
        for invalid in [
            json!(-1),
            json!(u64::from(u32::MAX) + 1),
            json!(0.5),
            json!(1.0),
        ] {
            let mut value = body();
            value["headerId"] = invalid;
            assert!(serde_json::from_value::<Responses>(value).is_err());
        }
    }

    #[test]
    fn timestamp_checks_apply_to_header_and_optional_expiry() {
        // §7.5 requires the same exact spelling for both fields. Unlike the
        // header, the schema does not even assert date-time on leaseExpiry.
        for pointer in ["/timestamp", "/responses/0/leaseExpiry"] {
            for valid in ["2017-04-15T11:40:03.123Z", "2000-02-29T00:00:00.000Z"] {
                let mut value = body();
                *value.pointer_mut(pointer).unwrap() = json!(valid);
                round_trip(value);
            }
            for invalid in [
                "2017-04-15T11:40:03Z",
                "2017-04-15T11:40:03.123+00:00",
                "2017-04-15T11:40:03.1234Z",
                "2017-02-30T11:40:03.123Z",
                "not a date",
            ] {
                let mut value = body();
                *value.pointer_mut(pointer).unwrap() = json!(invalid);
                if pointer.ends_with("leaseExpiry") {
                    assert!(support::schema("responses").is_valid(&value));
                }
                assert!(serde_json::from_value::<Responses>(value).is_err());
            }
        }
        for year in [0, 9999] {
            let mut message: Responses = serde_json::from_str(FIXTURE).unwrap();
            let expiry = Utc.with_ymd_and_hms(year, 1, 1, 0, 0, 0).unwrap();
            message.responses[0].lease_expiry = Some(expiry);
            let text = serde_json::to_string(&message).unwrap();
            assert_eq!(serde_json::from_str::<Responses>(&text).unwrap(), message);
        }
        for expiry in [
            Utc.with_ymd_and_hms(2017, 1, 1, 0, 0, 0)
                .unwrap()
                .with_nanosecond(1)
                .unwrap(),
            Utc.with_ymd_and_hms(-1, 1, 1, 0, 0, 0).unwrap(),
            Utc.with_ymd_and_hms(10000, 1, 1, 0, 0, 0).unwrap(),
        ] {
            let mut message: Responses = serde_json::from_str(FIXTURE).unwrap();
            message.responses[0].lease_expiry = Some(expiry);
            assert!(serde_json::to_string(&message).is_err());
        }
    }

    #[test]
    fn cross_field_and_request_semantics_remain_unvalidated() {
        // §7.5 says expiry shall only be sent with GRANTED. Keep this explicit
        // acceptance boundary until semantic validation has an agreed API.
        for decision in ["GRANTED", "QUEUED", "REVOKED", "REJECTED"] {
            let mut value = body();
            value["responses"][0]["grantType"] = json!(decision);
            value["responses"][1]["requestId"] = value["responses"][0]["requestId"].clone();
            round_trip(value);
        }
        let mut value = body();
        value["responses"][0]["leaseExpiry"] = json!("1991-03-11T11:40:03.123Z");
        let decoded = round_trip(value);
        assert!(decoded.responses[0].lease_expiry.unwrap() < decoded.header.timestamp);
        assert_eq!(
            decoded.responses[0].lease_expiry,
            Some("1991-03-11T11:40:03.123Z".parse::<DateTime<Utc>>().unwrap())
        );
    }
}

#[cfg(feature = "json")]
#[test]
fn json_helpers_round_trip_and_return_original_errors() {
    use chrono::Timelike;
    let message = Responses::from_json(FIXTURE).unwrap();
    assert_eq!(
        Responses::from_json_slice(FIXTURE.as_bytes()).unwrap(),
        message
    );
    assert_eq!(
        Responses::from_json(&message.to_json().unwrap()).unwrap(),
        message
    );
    assert_eq!(
        Responses::from_json_slice(&message.to_json_vec().unwrap()).unwrap(),
        message
    );
    assert_eq!(
        Responses::from_json("{").unwrap_err().to_string(),
        serde_json::from_str::<Responses>("{")
            .unwrap_err()
            .to_string()
    );
    assert!(Responses::from_json_slice(&[0xff]).is_err());
    let mut invalid = message;
    invalid.responses[0].lease_expiry = Some(invalid.header.timestamp.with_nanosecond(1).unwrap());
    assert!(invalid.to_json().is_err());
    assert!(invalid.to_json_vec().is_err());
}
