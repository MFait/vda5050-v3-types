use vda5050_v3_types::Connection;

fn main() -> Result<(), serde_json::Error> {
    let body = r#"{
        "headerId": 0,
        "timestamp": "2017-04-15T11:40:03.123Z",
        "version": "3.0.0",
        "manufacturer": "Example",
        "serialNumber": "robot-1",
        "connectionState": "ONLINE"
    }"#;
    let message = Connection::from_json(body)?;
    let payload = message.to_json_vec()?;
    assert_eq!(Connection::from_json_slice(&payload)?, message);
    Ok(())
}
