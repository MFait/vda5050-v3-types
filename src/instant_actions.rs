//! Instant actions body (VDA 5050 v3.0.0, sections 6.2.1 and 7.4).

use crate::common::{Action, Header};

object_only_deserialize! {
/// Actions to perform immediately outside the regular order.
///
/// Action support, identifier uniqueness, parameter meaning and conflicts with
/// the current order are not validated. Encoding/decoding enforces `NONE`
/// blocking for every action; direct Rust construction is not validated.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct InstantActions {
    /// Shared header, flattened into the wire object.
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub header: Header,
    /// Required action list; no minimum length is asserted by the schema.
    #[cfg_attr(feature = "serde", serde(with = "wire_actions"))]
    pub actions: Vec<Action>,
}
}

#[cfg(feature = "json")]
impl_json_helpers!(InstantActions);

#[cfg(feature = "serde")]
mod wire_actions {
    use super::*;
    use crate::common::BlockingType;
    use serde::{
        Deserialize, Deserializer, Serialize, Serializer, de::Error as _, ser::Error as _,
    };

    fn check(actions: &[Action]) -> Result<(), String> {
        for (index, action) in actions.iter().enumerate() {
            if action.blocking_type != BlockingType::None {
                return Err(format!(
                    "actions[{index}].blockingType must be NONE for instant actions (section 6.2.1)"
                ));
            }
        }
        Ok(())
    }

    pub(super) fn serialize<S: Serializer>(
        actions: &[Action],
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        check(actions).map_err(S::Error::custom)?;
        actions.serialize(serializer)
    }

    pub(super) fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Vec<Action>, D::Error> {
        let actions = Vec::<Action>::deserialize(deserializer)?;
        check(&actions).map_err(D::Error::custom)?;
        Ok(actions)
    }
}
