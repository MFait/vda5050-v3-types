use vda5050_v3_types::topic::Topic;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let topic: Topic = serde_json::from_str(r#""vda5050/v3/KIT/0001/state""#)?;
    assert_eq!(
        serde_json::to_string(&topic)?,
        r#""vda5050/v3/KIT/0001/state""#
    );
    assert!(serde_json::from_str::<Topic>(r#""vda5050/v2/KIT/0001/state""#).is_err());
    Ok(())
}
