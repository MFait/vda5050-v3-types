//! Consistency checks relate pinned specification sections 4.2-4.3 and 7.2.
//! Exact identity equality and header version 3.0.0 are agreed helper policies,
//! separate from decoding and protocol validation.

#![cfg(feature = "topic")]

use vda5050_v3_types::{
    Connection,
    common::Header,
    connection::ConnectionState,
    topic::{Message, PairingError, Topic, TopicKind, TopicMessage},
};

fn body() -> Connection {
    Connection {
        header: Header {
            header_id: 0,
            timestamp: chrono::DateTime::UNIX_EPOCH,
            version: "3.0.0".into(),
            manufacturer: "KIT".into(),
            serial_number: "0001".into(),
        },
        connection_state: ConnectionState::Online,
    }
}

#[test]
fn pairing_access_clone_and_consuming_extraction_work_without_serde() {
    let body = body();
    let topic = body.topic("custom").unwrap();
    let pair = TopicMessage::new(topic.clone(), body.clone()).unwrap();
    assert_eq!(pair.topic(), &topic);
    assert_eq!(pair.message().header(), &body.header);
    assert_eq!(pair.message().topic_kind(), TopicKind::Connection);
    assert_eq!(pair.clone(), pair);
    assert_eq!(
        TopicMessage::new(topic.clone(), Box::new(body.clone())).unwrap(),
        pair
    );
    let (extracted_topic, mut message) = pair.into_parts();
    assert_eq!(extracted_topic, topic);
    assert_eq!(message, Message::from(body));
    let Message::Connection(inner) = &mut message else {
        panic!("incorrect variant");
    };
    inner.header.serial_number = "different".into();
    assert_eq!(
        TopicMessage::new(extracted_topic, message),
        Err(PairingError::SerialNumberMismatch)
    );
}

#[test]
fn each_consistency_rule_rejects_a_mismatch_without_repair() {
    let original = body();
    let topic = original.topic("custom").unwrap();
    let wrong_kind = Topic::new("custom", "KIT", "0001", TopicKind::Order).unwrap();
    assert_eq!(
        TopicMessage::new(wrong_kind, original.clone()),
        Err(PairingError::KindMismatch {
            topic_kind: TopicKind::Order,
            body_kind: TopicKind::Connection,
        })
    );
    for version in ["", "3.0.1", "3.1.0", "2.0.0", "v3", "3.0.0 ", "3.0.0+build"] {
        let mut invalid = original.clone();
        invalid.header.version = version.into();
        assert_eq!(
            TopicMessage::new(topic.clone(), invalid),
            Err(PairingError::UnsupportedVersion)
        );
    }
    for manufacturer in ["kit", " KIT", "KIT ", "", "other", "K\u{0130}T"] {
        let mut invalid = original.clone();
        invalid.header.manufacturer = manufacturer.into();
        assert_eq!(
            TopicMessage::new(topic.clone(), invalid),
            Err(PairingError::ManufacturerMismatch)
        );
    }
    for serial in ["0002", "0001 ", " 0001", "", "robot"] {
        let mut invalid = original.clone();
        invalid.header.serial_number = serial.into();
        assert_eq!(
            TopicMessage::new(topic.clone(), invalid),
            Err(PairingError::SerialNumberMismatch)
        );
    }
    let mut unicode = original.clone();
    unicode.header.manufacturer = "M\u{00fc}ller".into();
    let normalized_differently =
        Topic::new("custom", "Mu\u{0308}ller", "0001", TopicKind::Connection).unwrap();
    assert_eq!(
        TopicMessage::new(normalized_differently, unicode.clone()),
        Err(PairingError::ManufacturerMismatch)
    );
    assert!(TopicMessage::new(unicode.topic("custom").unwrap(), unicode).is_ok());
}

#[test]
fn errors_report_deterministic_first_failure_and_do_not_retain_inputs() {
    use std::error::Error;
    let mut invalid = body();
    invalid.header.version = "2.0.0".into();
    invalid.header.manufacturer = "other".into();
    invalid.header.serial_number = "different".into();
    let topic = body().topic("custom").unwrap();
    assert_eq!(
        TopicMessage::new(topic.clone(), invalid.clone()),
        Err(PairingError::UnsupportedVersion)
    );
    invalid.header.version = "3.0.0".into();
    assert_eq!(
        TopicMessage::new(topic.clone(), invalid.clone()),
        Err(PairingError::ManufacturerMismatch)
    );
    invalid.header.manufacturer = "KIT".into();
    assert_eq!(
        TopicMessage::new(topic, invalid),
        Err(PairingError::SerialNumberMismatch)
    );
    for error in [
        PairingError::KindMismatch {
            topic_kind: TopicKind::State,
            body_kind: TopicKind::Order,
        },
        PairingError::UnsupportedVersion,
        PairingError::ManufacturerMismatch,
        PairingError::SerialNumberMismatch,
    ] {
        assert!(!error.to_string().is_empty());
        assert!(error.source().is_none());
    }
}

#[cfg(feature = "json")]
mod json {
    use super::*;

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

    #[test]
    fn every_body_encodes_separate_topic_and_payload_without_an_envelope() {
        for (kind, fixture) in FIXTURES {
            let message = kind.decode_json(fixture).unwrap();
            let topic = message.topic("custom").unwrap();
            let pair = TopicMessage::new(topic.clone(), message.clone()).unwrap();
            let (text_topic, text) = pair.to_json().unwrap();
            let (bytes_topic, bytes) = pair.to_json_vec().unwrap();
            assert_eq!(text_topic, topic.to_string());
            assert_eq!(bytes_topic, text_topic);
            assert_eq!(text.as_bytes(), bytes);
            assert_eq!(topic.decode_json(&text).unwrap(), message);
            assert_eq!(topic.decode_json_slice(&bytes).unwrap(), message);
            let value: serde_json::Value = serde_json::from_str(&text).unwrap();
            let expected: serde_json::Value = serde_json::from_str(fixture).unwrap();
            assert_eq!(value, expected);
            assert!(value.get("body").is_none());
            assert!(value.get("topic").is_none());
            assert_eq!(
                TopicMessage::new(topic.clone(), topic.decode_json(&text).unwrap()).unwrap(),
                pair
            );
            for (other_kind, _) in FIXTURES {
                if other_kind != kind {
                    let other = Topic::new(
                        "custom",
                        topic.manufacturer(),
                        topic.serial_number(),
                        other_kind,
                    )
                    .unwrap();
                    assert_eq!(
                        TopicMessage::new(other, message.clone()),
                        Err(PairingError::KindMismatch {
                            topic_kind: other_kind,
                            body_kind: kind,
                        })
                    );
                }
            }
        }
    }

    #[test]
    fn ordinary_dispatch_stays_unchecked_until_explicit_pairing() {
        for (kind, fixture) in FIXTURES {
            let topic = Topic::new("custom", "different", "different", kind).unwrap();
            let decoded = topic.decode_json(fixture).unwrap();
            assert_eq!(
                TopicMessage::new(topic, decoded),
                Err(PairingError::ManufacturerMismatch)
            );
        }
    }

    #[test]
    fn pairing_does_not_claim_body_validity_and_encoding_returns_original_errors() {
        let mut body =
            vda5050_v3_types::Visualization::from_json(include_str!("fixtures/visualization.json"))
                .unwrap();
        for invalid in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            body.velocity.as_mut().unwrap().vx = Some(invalid);
            let expected = body.to_json().unwrap_err();
            let pair = TopicMessage::new(body.topic("custom").unwrap(), body.clone()).unwrap();
            for actual in [pair.to_json().unwrap_err(), pair.to_json_vec().unwrap_err()] {
                assert_eq!(actual.to_string(), expected.to_string());
                assert_eq!(actual.classify(), expected.classify());
                assert_eq!(actual.line(), expected.line());
                assert_eq!(actual.column(), expected.column());
            }
        }
        let mut zones =
            vda5050_v3_types::ZoneSet::from_json(include_str!("fixtures/zone-set.json")).unwrap();
        zones.zone_set.zones[0].vertices.clear();
        let pair = TopicMessage::new(zones.topic("custom").unwrap(), zones).unwrap();
        assert!(pair.to_json().is_err());
        assert!(pair.to_json_vec().is_err());
    }

    #[test]
    fn paired_payload_text_and_bytes_preserve_float_bits() {
        let mut body =
            vda5050_v3_types::Visualization::from_json(include_str!("fixtures/visualization.json"))
                .unwrap();
        for value in [
            8.971135692166839e-150,
            f64::MIN,
            f64::MAX,
            f64::from_bits(1),
            -0.0,
            0.1,
        ] {
            body.velocity.as_mut().unwrap().vx = Some(value);
            let pair = TopicMessage::new(body.topic("custom").unwrap(), body.clone()).unwrap();
            let (_, text) = pair.to_json().unwrap();
            let (_, bytes) = pair.to_json_vec().unwrap();
            for decoded in [
                pair.topic().decode_json(&text).unwrap(),
                pair.topic().decode_json_slice(&bytes).unwrap(),
            ] {
                let Message::Visualization(inner) = decoded else {
                    panic!("incorrect variant");
                };
                assert_eq!(
                    inner.velocity.unwrap().vx.unwrap().to_bits(),
                    value.to_bits()
                );
            }
        }
    }
}
