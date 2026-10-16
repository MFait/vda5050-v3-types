use vda5050_v3_types::{
    Factsheet, InstantActions, Order, Responses, State, Visualization, ZoneSet, zone_set::ZoneKind,
};

fn main() {
    // Independent package: library dev-dependencies must not enable JSON
    // features that mask consumer precision regressions.
    check_zone_set();
    check_factsheet();
    check_visualization();
    check_instant_actions();
    check_responses();
    check_order();
    check_state();
}

fn check_zone_set() {
    let mut zones = ZoneSet::from_json(include_str!("../../fixtures/zone-set.json")).unwrap();
    assert_eq!(
        ZoneSet::from_json_slice(&zones.to_json_vec().unwrap()).unwrap(),
        zones
    );
    for value in [
        8.971135692166839e-150,
        f64::MIN,
        f64::MAX,
        f64::MIN_POSITIVE,
        f64::from_bits(1),
        -0.0,
        0.1,
    ] {
        zones.zone_set.zones[4].kind = ZoneKind::SpeedLimit {
            maximum_speed: value,
        };
        zones.zone_set.zones[0].vertices[0].x = value;
        for decoded in [
            ZoneSet::from_json(&zones.to_json().unwrap()).unwrap(),
            ZoneSet::from_json_slice(&zones.to_json_vec().unwrap()).unwrap(),
        ] {
            let ZoneKind::SpeedLimit { maximum_speed } = decoded.zone_set.zones[4].kind else {
                panic!("zone kind changed during round trip");
            };
            assert_eq!(maximum_speed.to_bits(), value.to_bits());
            assert_eq!(
                decoded.zone_set.zones[0].vertices[0].x.to_bits(),
                value.to_bits()
            );
        }
    }
}

fn check_factsheet() {
    let mut factsheet =
        Factsheet::from_json(include_str!("../../fixtures/factsheet.json")).unwrap();
    assert_eq!(
        Factsheet::from_json(&factsheet.to_json().unwrap()).unwrap(),
        factsheet
    );
    assert_eq!(
        Factsheet::from_json_slice(&factsheet.to_json_vec().unwrap()).unwrap(),
        factsheet
    );
    for value in [0.1, f32::MAX, f32::MIN_POSITIVE, f32::from_bits(1), -0.0] {
        factsheet.protocol_limits.timing.minimum_order_interval = value;
        for decoded in [
            Factsheet::from_json(&factsheet.to_json().unwrap()).unwrap(),
            Factsheet::from_json_slice(&factsheet.to_json_vec().unwrap()).unwrap(),
        ] {
            assert_eq!(
                decoded
                    .protocol_limits
                    .timing
                    .minimum_order_interval
                    .to_bits(),
                value.to_bits()
            );
        }
    }
    for value in [
        8.971135692166839e-150,
        f64::MIN,
        f64::MAX,
        f64::MIN_POSITIVE,
        f64::from_bits(1),
        -f64::from_bits(1),
        -0.0,
        0.1,
        std::f64::consts::PI,
    ] {
        factsheet.physical_parameters.length = value;
        for decoded in [
            Factsheet::from_json(&factsheet.to_json().unwrap()).unwrap(),
            Factsheet::from_json_slice(&factsheet.to_json_vec().unwrap()).unwrap(),
        ] {
            assert_eq!(
                decoded.physical_parameters.length.to_bits(),
                value.to_bits()
            );
        }
    }
}

fn check_visualization() {
    let mut body = Visualization::from_json(include_str!("../../fixtures/visualization.json"))
        .expect("valid fixture");
    for value in [
        8.971135692166839e-150,
        f64::MIN,
        f64::MAX,
        f64::MIN_POSITIVE,
        f64::from_bits(1),
        -f64::from_bits(1),
        -0.0,
        0.1,
        std::f64::consts::PI,
    ] {
        body.velocity.as_mut().unwrap().vx = Some(value);
        for decoded in [
            Visualization::from_json(&body.to_json().unwrap()).unwrap(),
            Visualization::from_json_slice(&body.to_json_vec().unwrap()).unwrap(),
        ] {
            assert_eq!(
                decoded.velocity.unwrap().vx.unwrap().to_bits(),
                value.to_bits(),
                "float64 precision lost for {value:?}"
            );
        }
    }
}

fn check_instant_actions() {
    let actions =
        InstantActions::from_json(include_str!("../../fixtures/instant-actions.json")).unwrap();
    assert_eq!(
        InstantActions::from_json_slice(&actions.to_json_vec().unwrap()).unwrap(),
        actions
    );
}

fn check_responses() {
    let responses = Responses::from_json(include_str!("../../fixtures/responses.json")).unwrap();
    assert_eq!(
        Responses::from_json(&responses.to_json().unwrap()).unwrap(),
        responses
    );
    assert_eq!(
        Responses::from_json_slice(&responses.to_json_vec().unwrap()).unwrap(),
        responses
    );
}

fn check_order() {
    let mut order = Order::from_json(include_str!("../../fixtures/order.json")).unwrap();
    for value in [
        8.971135692166839e-150,
        f64::MIN,
        f64::MAX,
        f64::MIN_POSITIVE,
        f64::from_bits(1),
        -f64::from_bits(1),
        -0.0,
        0.1,
    ] {
        order.edges[0].maximum_rotation_speed = Some(value);
        for decoded in [
            Order::from_json(&order.to_json().unwrap()).unwrap(),
            Order::from_json_slice(&order.to_json_vec().unwrap()).unwrap(),
        ] {
            assert_eq!(
                decoded.edges[0].maximum_rotation_speed.unwrap().to_bits(),
                value.to_bits()
            );
        }
    }
}

fn check_state() {
    let mut state = State::from_json(include_str!("../../fixtures/state.json")).unwrap();
    for value in [
        8.971135692166839e-150,
        f64::MIN,
        f64::MAX,
        f64::MIN_POSITIVE,
        f64::from_bits(1),
        -f64::from_bits(1),
        -0.0,
        0.1,
    ] {
        state.power_supply.battery_current = Some(value);
        for decoded in [
            State::from_json(&state.to_json().unwrap()).unwrap(),
            State::from_json_slice(&state.to_json_vec().unwrap()).unwrap(),
        ] {
            assert_eq!(
                decoded.power_supply.battery_current.unwrap().to_bits(),
                value.to_bits()
            );
        }
    }
}
