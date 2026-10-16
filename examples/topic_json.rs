use vda5050_v3_types::topic::{Message, Topic, TopicMessage};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let topic: Topic = "vda5050/v3/KIT/0001/responses".parse()?;
    let payload = br#"{
        "headerId": 0,
        "timestamp": "2017-04-15T11:40:03.123Z",
        "version": "3.0.0",
        "manufacturer": "KIT",
        "serialNumber": "0001",
        "responses": []
    }"#;
    let message = topic.decode_json_slice(payload)?;
    assert!(matches!(&message, Message::Responses(_)));

    // Dispatch selects the body type; pairing separately checks header consistency.
    let pair = TopicMessage::new(topic, message)?;
    let (topic_text, payload_bytes) = pair.to_json_vec()?;
    assert_eq!(topic_text, "vda5050/v3/KIT/0001/responses");
    assert_eq!(
        pair.topic().decode_json_slice(&payload_bytes)?,
        *pair.message()
    );
    Ok(())
}
