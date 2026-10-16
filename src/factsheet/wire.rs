use crate::serde_helpers::{finite, nonnegative, percentage};

field_adapter!(maximum_load_mass, nonnegative, "maximumLoadMass");
field_adapter!(minimum_speed, nonnegative, "minimumSpeed");
field_adapter!(maximum_speed, nonnegative, "maximumSpeed", optional_adapter);
field_adapter!(
    minimum_angular_speed,
    nonnegative,
    "minimumAngularSpeed",
    optional_adapter
);
field_adapter!(
    maximum_angular_speed,
    nonnegative,
    "maximumAngularSpeed",
    optional_adapter
);
field_adapter!(
    maximum_acceleration,
    nonnegative,
    "maximumAcceleration",
    optional_adapter
);
field_adapter!(
    maximum_deceleration,
    finite,
    "maximumDeceleration",
    optional_adapter
);
field_adapter!(minimum_height, finite, "minimumHeight");
field_adapter!(maximum_height, finite, "maximumHeight");
field_adapter!(width, finite, "width");
field_adapter!(length, finite, "length");
field_adapter!(diameter, finite, "diameter");
field_adapter!(
    center_displacement,
    finite,
    "centerDisplacement",
    optional_adapter
);
field_adapter!(
    maximum_weight,
    nonnegative,
    "maximumWeight",
    optional_adapter
);
field_adapter!(
    minimum_loadhandling_height,
    nonnegative,
    "minimumLoadhandlingHeight",
    optional_adapter
);
field_adapter!(
    maximum_loadhandling_height,
    nonnegative,
    "maximumLoadhandlingHeight",
    optional_adapter
);
field_adapter!(
    minimum_loadhandling_depth,
    finite,
    "minimumLoadhandlingDepth",
    optional_adapter
);
field_adapter!(
    maximum_loadhandling_depth,
    finite,
    "maximumLoadhandlingDepth",
    optional_adapter
);
field_adapter!(
    minimum_loadhandling_tilt,
    finite,
    "minimumLoadhandlingTilt",
    optional_adapter
);
field_adapter!(
    maximum_loadhandling_tilt,
    finite,
    "maximumLoadhandlingTilt",
    optional_adapter
);
field_adapter!(pick_time, nonnegative, "pickTime", optional_adapter);
field_adapter!(drop_time, nonnegative, "dropTime", optional_adapter);
field_adapter!(
    critical_low_charging_level,
    percentage,
    "criticalLowChargingLevel",
    optional_adapter
);
field_adapter!(
    minimum_desired_charging_level,
    percentage,
    "minimumDesiredChargingLevel",
    optional_adapter
);
field_adapter!(
    maximum_desired_charging_level,
    percentage,
    "maximumDesiredChargingLevel",
    optional_adapter
);

macro_rules! timing_adapter {
    ($name:ident, $field:literal $(, $optional:ident)?) => {
        pub(crate) mod $name {
            use serde::{Deserialize, Deserializer, Serializer, de::Error as _, ser::Error as _};

            pub(crate) fn serialize<S: Serializer>(value: &f32, serializer: S) -> Result<S::Ok, S::Error> {
                if !value.is_finite() || *value < 0.0 {
                    return Err(S::Error::custom(concat!($field, ": expected a finite nonnegative float32")));
                }
                serializer.serialize_f32(*value)
            }

            pub(crate) fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<f32, D::Error> {
                let value = f64::deserialize(deserializer)?;
                if !value.is_finite() || value < 0.0 {
                    return Err(D::Error::custom(concat!($field, ": expected a finite nonnegative float32")));
                }
                let narrowed = value as f32;
                if !narrowed.is_finite() {
                    return Err(D::Error::custom(concat!($field, ": expected a finite nonnegative float32")));
                }
                Ok(narrowed)
            }

            $($optional!(f32);)?
        }
    };
}

timing_adapter!(minimum_order_interval, "minimumOrderInterval");
timing_adapter!(minimum_state_interval, "minimumStateInterval");
timing_adapter!(
    default_state_interval,
    "defaultStateInterval",
    optional_adapter
);
timing_adapter!(
    visualization_interval,
    "visualizationInterval",
    optional_adapter
);
