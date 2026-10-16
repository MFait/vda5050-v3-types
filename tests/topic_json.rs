//! Dispatch follows pinned VDA5050/VDA5050@0b2ae43154bcd1ab2be4e2445df8b406dcad77b2
//! VDA5050_EN.md section 4.3's topic/schema correspondence.
//! Decoding is distinct from topic/header consistency and protocol validation.

#![cfg(all(feature = "topic", feature = "json"))]

use vda5050_v3_types::{
    Connection, Factsheet, InstantActions, Order, Responses, State, Visualization, ZoneSet,
    topic::{Message, Topic, TopicKind},
};

const FIXTURES: [(TopicKind, &str); 8] = [
    (TopicKind::Order, include_str!("fixtures/order.json")),
    (
        TopicKind::InstantActions,
        include_str!("fixtures/instant-actions.json"),
    ),
    (TopicKind::State, include_str!("fixtures/state.json")),
    (
        TopicKind::Visualization,
        include_str!("fixtures/visualization.json"),
    ),
    (
        TopicKind::Connection,
        include_str!("fixtures/connection.json"),
    ),
    (
        TopicKind::Factsheet,
        include_str!("fixtures/factsheet.json"),
    ),
    (TopicKind::ZoneSet, include_str!("fixtures/zone-set.json")),
    (
        TopicKind::Responses,
        include_str!("fixtures/responses.json"),
    ),
];

fn expected(kind: TopicKind, payload: &str) -> Message {
    match kind {
        TopicKind::Order => Order::from_json(payload).unwrap().into(),
        TopicKind::InstantActions => InstantActions::from_json(payload).unwrap().into(),
        TopicKind::State => State::from_json(payload).unwrap().into(),
        TopicKind::Visualization => Visualization::from_json(payload).unwrap().into(),
        TopicKind::Connection => Connection::from_json(payload).unwrap().into(),
        TopicKind::Factsheet => Factsheet::from_json(payload).unwrap().into(),
        TopicKind::ZoneSet => ZoneSet::from_json(payload).unwrap().into(),
        TopicKind::Responses => Responses::from_json(payload).unwrap().into(),
    }
}

fn concrete_error(kind: TopicKind, payload: &[u8]) -> serde_json::Error {
    match kind {
        TopicKind::Order => Order::from_json_slice(payload).unwrap_err(),
        TopicKind::InstantActions => InstantActions::from_json_slice(payload).unwrap_err(),
        TopicKind::State => State::from_json_slice(payload).unwrap_err(),
        TopicKind::Visualization => Visualization::from_json_slice(payload).unwrap_err(),
        TopicKind::Connection => Connection::from_json_slice(payload).unwrap_err(),
        TopicKind::Factsheet => Factsheet::from_json_slice(payload).unwrap_err(),
        TopicKind::ZoneSet => ZoneSet::from_json_slice(payload).unwrap_err(),
        TopicKind::Responses => Responses::from_json_slice(payload).unwrap_err(),
    }
}

fn same_error(actual: serde_json::Error, expected: &serde_json::Error) {
    assert_eq!(actual.classify(), expected.classify());
    assert_eq!(actual.line(), expected.line());
    assert_eq!(actual.column(), expected.column());
    assert_eq!(actual.to_string(), expected.to_string());
}

#[test]
fn all_eight_kinds_and_topics_decode_text_and_bytes_to_exact_bodies() {
    for (kind, fixture) in FIXTURES {
        let expected = expected(kind, fixture);
        let topic = Topic::new("custom", "different-maker", "different-serial", kind).unwrap();
        for decoded in [
            kind.decode_json(fixture).unwrap(),
            kind.decode_json_slice(fixture.as_bytes()).unwrap(),
            topic.decode_json(fixture).unwrap(),
            topic.decode_json_slice(fixture.as_bytes()).unwrap(),
        ] {
            assert_eq!(decoded, expected);
            assert_eq!(decoded.topic_kind(), kind);
        }
    }
}

#[test]
fn malformed_input_returns_original_errors_without_fallback() {
    for (kind, _) in FIXTURES {
        let topic = Topic::new("vda5050", "KIT", "0001", kind).unwrap();
        for invalid in [
            "",
            "{",
            "{}",
            "[]",
            "null",
            "true",
            "\"text\"",
            "{\n\"headerId\":",
            "{} {}",
        ] {
            let expected = concrete_error(kind, invalid.as_bytes());
            same_error(kind.decode_json(invalid).unwrap_err(), &expected);
            same_error(
                kind.decode_json_slice(invalid.as_bytes()).unwrap_err(),
                &expected,
            );
            same_error(topic.decode_json(invalid).unwrap_err(), &expected);
            same_error(
                topic.decode_json_slice(invalid.as_bytes()).unwrap_err(),
                &expected,
            );
        }
        let invalid = b"\xff";
        let expected = concrete_error(kind, invalid);
        same_error(kind.decode_json_slice(invalid).unwrap_err(), &expected);
        same_error(topic.decode_json_slice(invalid).unwrap_err(), &expected);
    }
    let order = include_str!("fixtures/order.json");
    assert!(TopicKind::Connection.decode_json(order).is_err());
}

#[test]
fn concrete_wire_constraints_are_not_bypassed_by_dispatch() {
    for (kind, fixture) in FIXTURES {
        let mut value: serde_json::Value = serde_json::from_str(fixture).unwrap();
        for invalid in [serde_json::json!(-1), serde_json::json!(4294967296_u64)] {
            value["headerId"] = invalid;
            let payload = value.to_string();
            same_error(
                kind.decode_json(&payload).unwrap_err(),
                &concrete_error(kind, payload.as_bytes()),
            );
        }
        value["headerId"] = serde_json::json!(0);
        value["timestamp"] = serde_json::json!("2017-04-15T11:40:03Z");
        assert!(kind.decode_json(&value.to_string()).is_err());
    }
    let mut actions: serde_json::Value =
        serde_json::from_str(include_str!("fixtures/instant-actions.json")).unwrap();
    actions["actions"][0]["blockingType"] = serde_json::json!("HARD");
    assert!(
        TopicKind::InstantActions
            .decode_json(&actions.to_string())
            .is_err()
    );
    let mut zones: serde_json::Value =
        serde_json::from_str(include_str!("fixtures/zone-set.json")).unwrap();
    zones["zoneSet"]["zones"][0]["vertices"] = serde_json::json!([]);
    assert!(TopicKind::ZoneSet.decode_json(&zones.to_string()).is_err());
}

#[test]
fn decoding_neither_checks_header_consistency_nor_accepts_an_envelope() {
    for (kind, fixture) in FIXTURES {
        let topic = Topic::new("vda5050", "unrelated", "unrelated", kind).unwrap();
        let mut value: serde_json::Value = serde_json::from_str(fixture).unwrap();
        value["version"] = serde_json::json!("2.0.0");
        value["manufacturer"] = serde_json::json!("not/a/topic/level");
        value["serialNumber"] = serde_json::json!("");
        let decoded = topic.decode_json(&value.to_string()).unwrap();
        assert_eq!(decoded.header().version, "2.0.0");
        assert_eq!(decoded.header().manufacturer, "not/a/topic/level");
        assert_eq!(decoded.header().serial_number, "");
        assert!(decoded.topic("vda5050").is_err());
        let envelope = serde_json::json!({"kind": kind.as_str(), "body": value});
        assert!(kind.decode_json(&envelope.to_string()).is_err());
    }
}

#[test]
fn kind_selection_is_not_payload_kind_detection() {
    let mut combined: serde_json::Value =
        serde_json::from_str(include_str!("fixtures/connection.json")).unwrap();
    combined["responses"] = serde_json::json!([]);
    let payload = combined.to_string();
    assert!(matches!(
        TopicKind::Connection.decode_json(&payload).unwrap(),
        Message::Connection(_)
    ));
    assert!(matches!(
        TopicKind::Responses.decode_json(&payload).unwrap(),
        Message::Responses(_)
    ));
}

#[test]
fn dispatch_preserves_float_precision_through_text_and_bytes() {
    let mut body = Visualization::from_json(include_str!("fixtures/visualization.json")).unwrap();
    let topic = body.topic("vda5050").unwrap();
    for value in [
        8.971135692166839e-150,
        f64::MIN,
        f64::MAX,
        f64::from_bits(1),
        -0.0,
        0.1,
    ] {
        body.velocity.as_mut().unwrap().vx = Some(value);
        for decoded in [
            topic.decode_json(&body.to_json().unwrap()).unwrap(),
            topic
                .decode_json_slice(&body.to_json_vec().unwrap())
                .unwrap(),
        ] {
            let Message::Visualization(decoded) = decoded else {
                panic!("incorrect message variant");
            };
            assert_eq!(
                decoded.velocity.unwrap().vx.unwrap().to_bits(),
                value.to_bits()
            );
        }
    }
}
