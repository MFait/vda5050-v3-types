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
    let message: Connection = serde_json::from_str(body)?;
    let value = serde_json::to_value(&message)?;
    assert_eq!(value["connectionState"], "ONLINE");
    assert!(value.get("header").is_none());
    Ok(())
}
