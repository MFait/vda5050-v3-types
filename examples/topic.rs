use vda5050_v3_types::topic::{Topic, TopicFilter, TopicKind};

fn main() -> Result<(), vda5050_v3_types::topic::TopicError> {
    assert_eq!(TopicKind::InstantActions.as_str(), "instantActions");
    let topic = Topic::new("vda5050", "KIT", "0001", TopicKind::Order)?;
    assert_eq!(topic.to_string(), "vda5050/v3/KIT/0001/order");
    assert_eq!(topic.to_string().parse::<Topic>()?, topic);

    let states = TopicFilter::new("vda5050", None, None, Some(TopicKind::State))?;
    assert_eq!(states.to_string(), "vda5050/v3/+/+/state");
    Ok(())
}
