//! Pinned VDA5050/VDA5050@0b2ae43154bcd1ab2be4e2445df8b406dcad77b2:
//! VDA5050_EN.md sections 6.2.1, 7.2, 7.3/7.3.1 and 7.4.
//! https://github.com/VDA5050/VDA5050/blob/0b2ae43154bcd1ab2be4e2445df8b406dcad77b2/VDA5050_EN.md
//! Schema: vendored instantActions.schema from the same revision.

use chrono::{TimeZone, Utc};
use serde_json::{Value, json};
use vda5050_v3_types::{
    InstantActions,
    common::{Action, ActionParameter, ActionParameterValue, BlockingType, Header},
};

mod support;

const FIXTURE: &str = include_str!("fixtures/instant-actions.json");

fn body() -> Value {
    serde_json::from_str(FIXTURE).unwrap()
}

#[test]
fn construct_actions_and_non_null_values_without_optional_features() {
    let value = ActionParameterValue::try_from(json!({"nested": [null, 42]})).unwrap();
    assert_eq!(value.as_json(), &json!({"nested": [null, 42]}));
    assert_eq!(value.clone().into_json(), json!({"nested": [null, 42]}));
    assert_eq!(
        ActionParameterValue::try_from(Value::Null)
            .unwrap_err()
            .to_string(),
        "action parameter value must not be null"
    );
    let message = InstantActions {
        header: Header {
            header_id: 0,
            timestamp: Utc.with_ymd_and_hms(2026, 10, 16, 12, 0, 0).unwrap(),
            version: "3.0.0".into(),
            manufacturer: "Example".into(),
            serial_number: "robot-1".into(),
        },
        actions: vec![Action {
            action_id: "custom-1".into(),
            action_type: "vendorAction".into(),
            action_descriptor: None,
            retriable: None,
            blocking_type: BlockingType::None,
            action_parameters: Some(vec![ActionParameter {
                key: "settings".into(),
                value,
            }]),
        }],
    };
    assert_eq!(message.clone(), message);
}

#[test]
fn fixture_is_schema_valid() {
    support::schema("instantActions").validate(&body()).unwrap();
}

#[cfg(feature = "serde")]
mod wire {
    use super::*;

    fn round_trip(value: Value) -> InstantActions {
        support::schema("instantActions").validate(&value).unwrap();
        let decoded: InstantActions = serde_json::from_str(&value.to_string()).unwrap();
        let encoded: Value =
            serde_json::from_slice(&serde_json::to_vec(&decoded).unwrap()).unwrap();
        assert_eq!(encoded, value);
        support::schema("instantActions")
            .validate(&encoded)
            .unwrap();
        decoded
    }

    #[test]
    fn fixture_and_empty_actions_round_trip() {
        let decoded = round_trip(body());
        assert_eq!(decoded.actions.len(), 2);
        assert_eq!(decoded.actions[0].action_type, "vendorSignal");
        assert!(decoded.actions[1].action_parameters.is_none());
        let mut minimal = body();
        minimal["actions"] = json!([]);
        assert!(round_trip(minimal).actions.is_empty());
    }

    #[test]
    fn retriable_follows_shared_spec_action_despite_schema_omission() {
        // §7.4 reuses §7.3, including optional retriable. The instant schema
        // omits the property and therefore does not constrain it at all.
        for flag in [false, true] {
            let mut value = body();
            value["actions"][0]["retriable"] = json!(flag);
            assert_eq!(round_trip(value).actions[0].retriable, Some(flag));
        }
        for invalid in [Value::Null, json!("true"), json!(0)] {
            let mut value = body();
            value["actions"][0]["retriable"] = invalid;
            assert!(support::schema("instantActions").is_valid(&value));
            assert!(serde_json::from_value::<InstantActions>(value).is_err());
        }
    }

    #[test]
    fn required_optional_and_null_rules_follow_schema() {
        support::assert_required::<InstantActions>(
            &body(),
            support::schema("instantActions"),
            &[
                "/headerId",
                "/timestamp",
                "/version",
                "/manufacturer",
                "/serialNumber",
                "/actions",
                "/actions/0/actionId",
                "/actions/0/actionType",
                "/actions/0/blockingType",
                "/actions/0/actionParameters/0/key",
            ],
        );
        // Value permits objects, so test omission/null separately from typed-field helpers.
        for invalid in [None, Some(Value::Null)] {
            let mut value = body();
            if let Some(invalid) = invalid {
                value["actions"][0]["actionParameters"][0]["value"] = invalid;
            } else {
                support::remove(&mut value, "/actions/0/actionParameters/0/value");
            }
            assert!(!support::schema("instantActions").is_valid(&value));
            assert!(serde_json::from_value::<InstantActions>(value).is_err());
        }
        support::assert_optional::<InstantActions>(
            &body(),
            support::schema("instantActions"),
            &["/actions/0/actionDescriptor", "/actions/0/actionParameters"],
        );
        let mut empty = body();
        empty["actions"][0]["actionParameters"] = json!([]);
        round_trip(empty);
    }

    #[test]
    fn instant_actions_require_none_in_both_wire_directions() {
        // Section 6.2.1: instant blockingType is always NONE; order actions differ.
        for (wire, blocking) in [
            ("NONE", BlockingType::None),
            ("SINGLE", BlockingType::Single),
            ("SOFT", BlockingType::Soft),
            ("HARD", BlockingType::Hard),
        ] {
            let mut value = body();
            value["actions"][0]["blockingType"] = json!(wire);
            let action: Action = serde_json::from_value(value["actions"][0].clone()).unwrap();
            assert_eq!(action.blocking_type, blocking);
            assert_eq!(serde_json::to_value(&action).unwrap()["blockingType"], wire);
            if blocking == BlockingType::None {
                round_trip(value);
            } else {
                assert!(!support::schema("instantActions").is_valid(&value));
                assert!(serde_json::from_value::<InstantActions>(value).is_err());
                let mut message: InstantActions = serde_json::from_str(FIXTURE).unwrap();
                message.actions[1].blocking_type = blocking;
                let error = serde_json::to_string(&message).unwrap_err();
                assert!(error.to_string().contains("actions[1].blockingType"));
            }
        }
        for invalid in ["none", "UNKNOWN", ""] {
            let mut value = body();
            value["actions"][0]["blockingType"] = json!(invalid);
            assert!(serde_json::from_value::<InstantActions>(value).is_err());
        }
    }

    #[test]
    fn parameter_values_preserve_supported_shapes_and_numeric_boundaries() {
        // Section 7.3.1: arbitrary non-null JSON; nested null is not prohibited.
        for value in [
            json!(true),
            json!(false),
            json!(""),
            json!(i64::MIN),
            json!(u64::MAX),
            json!(f64::MAX),
            json!(8.971135692166839e-150),
            json!([]),
            json!({}),
            json!([null, {"nested": [true, 1]}]),
            json!({"nested": null}),
        ] {
            let wrapped = ActionParameterValue::try_from(value.clone()).unwrap();
            assert_eq!(serde_json::to_value(&wrapped).unwrap(), value);
            assert_eq!(
                serde_json::from_str::<ActionParameterValue>(
                    &serde_json::to_string(&wrapped).unwrap()
                )
                .unwrap(),
                wrapped
            );
            let mut message = body();
            message["actions"][0]["actionParameters"][0]["value"] = value;
            round_trip(message);
        }
        assert!(serde_json::from_str::<ActionParameterValue>("null").is_err());
    }

    #[test]
    fn public_models_reject_arrays_duplicates_and_wrong_types() {
        for value in [json!([]), json!([1]), Value::Null] {
            assert!(serde_json::from_value::<Action>(value.clone()).is_err());
            assert!(serde_json::from_value::<ActionParameter>(value.clone()).is_err());
            assert!(serde_json::from_value::<InstantActions>(value).is_err());
        }
        for (pointer, invalid) in [
            ("/actions", json!({})),
            ("/actions", json!([null])),
            ("/actions/0", json!([])),
            ("/actions/0/actionId", json!(42)),
            ("/actions/0/actionParameters", json!([[]])),
            ("/actions/0/actionParameters/0/key", json!(false)),
        ] {
            let mut value = body();
            *value.pointer_mut(pointer).unwrap() = invalid;
            assert!(!support::schema("instantActions").is_valid(&value));
            assert!(serde_json::from_value::<InstantActions>(value).is_err());
        }
        for duplicate in [
            r#"{"actionId":"a","actionId":"b","actionType":"x","blockingType":"NONE"}"#,
            r#"{"actionId":"a","actionType":"x","blockingType":"NONE","blockingType":"NONE"}"#,
        ] {
            assert!(serde_json::from_str::<Action>(duplicate).is_err());
        }
        assert!(
            serde_json::from_str::<ActionParameter>(r#"{"key":"a","value":1,"value":2}"#).is_err()
        );
    }

    #[test]
    fn header_checks_are_reused() {
        // Sections 7.2 and 7.4: uint32 and millisecond UTC.
        for id in [0, u32::MAX] {
            let mut value = body();
            value["headerId"] = json!(id);
            assert_eq!(round_trip(value).header.header_id, id);
        }
        for invalid in [json!(-1), json!(u64::from(u32::MAX) + 1), json!(1.5)] {
            let mut value = body();
            value["headerId"] = invalid;
            assert!(serde_json::from_value::<InstantActions>(value).is_err());
        }
        let mut value = body();
        value["timestamp"] = json!("2017-04-15T11:40:03Z");
        assert!(serde_json::from_value::<InstantActions>(value).is_err());
    }

    #[test]
    fn extensions_and_semantic_acceptance_boundary_are_explicit() {
        // Sections 7.3/7.4: custom names allowed; UUID is only a suggestion.
        // Section 6.2.1 order conflicts, support and ID uniqueness are not checked.
        let mut value = body();
        value["actions"][1]["actionId"] = value["actions"][0]["actionId"].clone();
        value["actions"][1]["actionType"] = json!("vendor.custom");
        round_trip(value.clone());
        let expected = value.clone();
        value["vendorField"] = json!(true);
        value["actions"][0]["vendorField"] = json!(true);
        value["actions"][0]["actionParameters"][0]["vendorField"] = json!(true);
        let decoded: InstantActions = serde_json::from_value(value).unwrap();
        assert_eq!(serde_json::to_value(decoded).unwrap(), expected);
    }
}

#[cfg(feature = "json")]
#[test]
fn json_helpers_preserve_bodies_and_errors() {
    let message = InstantActions::from_json(FIXTURE).unwrap();
    assert_eq!(
        InstantActions::from_json_slice(FIXTURE.as_bytes()).unwrap(),
        message
    );
    assert_eq!(
        InstantActions::from_json(&message.to_json().unwrap()).unwrap(),
        message
    );
    assert_eq!(
        InstantActions::from_json_slice(&message.to_json_vec().unwrap()).unwrap(),
        message
    );
    assert_eq!(
        InstantActions::from_json("{").unwrap_err().to_string(),
        serde_json::from_str::<InstantActions>("{")
            .unwrap_err()
            .to_string()
    );
    assert!(InstantActions::from_json_slice(&[0xff]).is_err());
    let mut invalid = message;
    invalid.actions[0].blocking_type = BlockingType::Hard;
    assert!(invalid.to_json().is_err());
    assert!(invalid.to_json_vec().is_err());
}
