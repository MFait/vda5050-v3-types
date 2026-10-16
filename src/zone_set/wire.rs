use crate::serde_helpers::{finite, unit_interval};

field_adapter!(maximum_speed, finite, "maximumSpeed");
field_adapter!(priority_factor, unit_interval, "priorityFactor");
field_adapter!(penalty_factor, unit_interval, "penaltyFactor");
field_adapter!(direction, finite, "direction");

pub(crate) mod vertices {
    use crate::common::Vertex;
    use serde::{
        Deserialize, Deserializer, Serialize, Serializer, de::Error as _, ser::Error as _,
    };

    pub(crate) fn serialize<S: Serializer>(
        vertices: &[Vertex],
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        if vertices.len() < 3 {
            return Err(S::Error::custom(
                "vertices: expected at least three vertices",
            ));
        }
        vertices.serialize(serializer)
    }

    pub(crate) fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Vec<Vertex>, D::Error> {
        let vertices = Vec::<Vertex>::deserialize(deserializer)?;
        if vertices.len() < 3 {
            return Err(D::Error::custom(
                "vertices: expected at least three vertices",
            ));
        }
        Ok(vertices)
    }
}
