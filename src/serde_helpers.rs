use std::{fmt, marker::PhantomData};

use serde::{
    Deserialize, Deserializer,
    de::{MapAccess, Visitor, value::MapAccessDeserializer},
};

pub(crate) fn object<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    struct ObjectVisitor<T>(PhantomData<T>);

    impl<'de, T: Deserialize<'de>> Visitor<'de> for ObjectVisitor<T> {
        type Value = T;

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("an object")
        }

        fn visit_map<M: MapAccess<'de>>(self, map: M) -> Result<T, M::Error> {
            T::deserialize(MapAccessDeserializer::new(map))
        }
    }

    deserializer.deserialize_map(ObjectVisitor(PhantomData))
}

pub(crate) fn non_null_option<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    T::deserialize(deserializer).map(Some)
}

macro_rules! number_adapter {
    ($name:ident, $valid:expr, $error:literal) => {
        pub(crate) mod $name {
            use serde::{Deserialize, Deserializer, Serializer, de::Error as _, ser::Error as _};

            pub(crate) fn serialize<S>(value: &f64, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                if !value.is_finite() || !($valid)(*value) {
                    return Err(S::Error::custom($error));
                }
                serializer.serialize_f64(*value)
            }

            pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<f64, D::Error>
            where
                D: Deserializer<'de>,
            {
                let value = f64::deserialize(deserializer)?;
                if !value.is_finite() || !($valid)(value) {
                    return Err(D::Error::custom($error));
                }
                Ok(value)
            }
        }
    };
}

number_adapter!(finite, |_| true, "expected a finite float64");
number_adapter!(
    angle,
    |value| (-std::f64::consts::PI..=std::f64::consts::PI).contains(&value),
    "expected a finite angle in [-pi, pi]"
);
number_adapter!(
    unit_interval,
    |value| (0.0..=1.0).contains(&value),
    "expected a finite value in [0, 1]"
);
number_adapter!(
    nonnegative,
    |value| value >= 0.0,
    "expected a finite nonnegative value"
);
number_adapter!(
    positive,
    |value| value > 0.0,
    "expected a finite positive value"
);
number_adapter!(
    deviation_angle,
    |value| (0.0..=std::f64::consts::PI).contains(&value),
    "expected a finite angle in [0, pi]"
);
// Preserve the schema's explicit decimal bound, not mathematical pi/2.
#[allow(clippy::approx_constant)]
const ELLIPSE_ROTATION_LIMIT: f64 = 1.570796327;
number_adapter!(
    ellipse_angle,
    |value| (-super::ELLIPSE_ROTATION_LIMIT..=super::ELLIPSE_ROTATION_LIMIT).contains(&value),
    "expected a finite ellipse angle in [-1.570796327, 1.570796327]"
);

field_adapter!(x, finite, "x");
field_adapter!(y, finite, "y");
field_adapter!(theta, angle, "theta", optional_adapter);
field_adapter!(
    localization_score,
    unit_interval,
    "localizationScore",
    optional_adapter
);
field_adapter!(
    deviation_range,
    nonnegative,
    "deviationRange",
    optional_adapter
);
field_adapter!(vx, finite, "vx", optional_adapter);
field_adapter!(vy, finite, "vy", optional_adapter);
field_adapter!(omega, finite, "omega", optional_adapter);
field_adapter!(weight, positive, "weight", optional_adapter);
field_adapter!(ellipse_a, nonnegative, "a");
field_adapter!(ellipse_b, nonnegative, "b");
field_adapter!(ellipse_theta, ellipse_angle, "theta");
field_adapter!(
    allowed_deviation_theta,
    deviation_angle,
    "allowedDeviationTheta",
    optional_adapter
);
field_adapter!(maximum_speed, finite, "maximumSpeed", optional_adapter);
field_adapter!(
    maximum_mobile_robot_height,
    finite,
    "maximumMobileRobotHeight",
    optional_adapter
);
field_adapter!(
    minimum_load_handling_device_height,
    finite,
    "minimumLoadHandlingDeviceHeight",
    optional_adapter
);
field_adapter!(orientation, angle, "orientation", optional_adapter);
field_adapter!(
    maximum_rotation_speed,
    finite,
    "maximumRotationSpeed",
    optional_adapter
);
field_adapter!(length, finite, "length", optional_adapter);
field_adapter!(left_width, nonnegative, "leftWidth");
field_adapter!(right_width, nonnegative, "rightWidth");
field_adapter!(
    distance_since_last_node,
    finite,
    "distanceSinceLastNode",
    optional_adapter
);
field_adapter!(z, finite, "z");
field_adapter!(bounding_box_theta, finite, "theta", optional_adapter);
field_adapter!(load_weight, nonnegative, "weight", optional_adapter);
field_adapter!(load_length, nonnegative, "length");
field_adapter!(load_width, nonnegative, "width");
field_adapter!(load_height, nonnegative, "height", optional_adapter);
field_adapter!(battery_voltage, finite, "batteryVoltage", optional_adapter);
field_adapter!(battery_current, finite, "batteryCurrent", optional_adapter);

number_adapter!(
    percentage,
    |value| (0.0..=100.0).contains(&value),
    "expected a finite percentage in [0, 100]"
);
field_adapter!(state_of_charge, percentage, "stateOfCharge");

pub(crate) mod battery_health {
    use serde::{Deserialize, Deserializer, Serializer, de::Error as _, ser::Error as _};

    pub(crate) fn serialize<S: Serializer>(value: &i8, serializer: S) -> Result<S::Ok, S::Error> {
        if !(0..=100).contains(value) {
            return Err(S::Error::custom("batteryHealth must be in [0, 100]"));
        }
        serializer.serialize_i8(*value)
    }

    pub(crate) fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<i8, D::Error> {
        let value = i8::deserialize(deserializer)?;
        if !(0..=100).contains(&value) {
            return Err(D::Error::custom("batteryHealth must be in [0, 100]"));
        }
        Ok(value)
    }

    optional_adapter!(i8);
}

pub(crate) mod degree {
    use serde::{Deserialize, Deserializer, Serializer, de::Error as _, ser::Error as _};

    pub(crate) fn serialize<S>(value: &u32, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if *value == 0 {
            return Err(S::Error::custom("NURBS degree must be at least 1"));
        }
        serializer.serialize_u32(*value)
    }

    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<u32, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        if value == 0 {
            return Err(D::Error::custom("NURBS degree must be at least 1"));
        }
        Ok(value)
    }

    optional_adapter!(u32);
}

pub(crate) mod knots {
    use serde::{
        Deserialize, Deserializer, Serialize, Serializer, de::Error as _, ser::Error as _,
    };

    fn valid(values: &[f64]) -> bool {
        values
            .iter()
            .all(|value| value.is_finite() && (0.0..=1.0).contains(value))
    }

    pub(crate) fn serialize<S>(values: &[f64], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if !valid(values) {
            return Err(S::Error::custom("NURBS knots must be finite and in [0, 1]"));
        }
        values.serialize(serializer)
    }

    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Vec<f64>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let values = Vec::<f64>::deserialize(deserializer)?;
        if !valid(&values) {
            return Err(D::Error::custom("NURBS knots must be finite and in [0, 1]"));
        }
        Ok(values)
    }

    optional_adapter!(Vec<f64>);
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::de::value::{Error, F64Deserializer};

    #[test]
    fn nonfinite_values_are_rejected_even_by_non_json_deserializers() {
        // Pinned VDA 5050 v3.0.0 section 7.1.4: IEEE NaN/infinity unsupported.
        // See docs/provenance.md for the upstream revision.
        for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            assert!(finite::deserialize(F64Deserializer::<Error>::new(value)).is_err());
            assert!(vx::optional::deserialize(F64Deserializer::<Error>::new(value)).is_err());
            assert!(angle::deserialize(F64Deserializer::<Error>::new(value)).is_err());
            assert!(unit_interval::deserialize(F64Deserializer::<Error>::new(value)).is_err());
            assert!(positive::deserialize(F64Deserializer::<Error>::new(value)).is_err());
            assert!(nonnegative::deserialize(F64Deserializer::<Error>::new(value)).is_err());
            assert!(deviation_angle::deserialize(F64Deserializer::<Error>::new(value)).is_err());
            assert!(ellipse_angle::deserialize(F64Deserializer::<Error>::new(value)).is_err());
            assert!(percentage::deserialize(F64Deserializer::<Error>::new(value)).is_err());
        }
    }
}
