#[cfg(test)]
use crate::lights::Lights;
use crate::lights::LightsCoordinates;

#[test]
fn given_lights_when_get_number_of_lights_then_should_return_100000() {
    let lights: Lights = Lights::create();
    let num_of_lights = lights.get_number_of_lights();
    assert_eq!(1000000, num_of_lights);
}

#[test]
fn given_initial_lights_when_get_total_brightness_then_should_return_0() {
    let lights: Lights = Lights::create();
    let total_brightness = lights.get_total_brightness();
    assert_eq!(0, total_brightness);
}

#[test]
fn given_lights_when_turn_on_lights_0_0_to_2_2_then_should_have_9_brightness() {
    let lights: Lights = Lights::create();

    let first_lights_coordinate = LightsCoordinates::create(0, 0);
    let second_lights_coordinate = LightsCoordinates::create(2, 2);
    let new_lights: Lights = lights.turn_on(&first_lights_coordinate, &second_lights_coordinate);

    assert_eq!(9, new_lights.get_total_brightness());
}

#[test]
fn given_lights_when_toggle_lights_0_0_to_2_2_then_should_have_18_brightness() {
    let lights: Lights = Lights::create();

    let new_lights: Lights = lights.toggle(
        &LightsCoordinates::create(0, 0),
        &LightsCoordinates::create(2, 2),
    );

    assert_eq!(18, new_lights.get_total_brightness());
}

#[test]
fn given_lights_when_turn_on_lights_0_0_to_999_999_then_should_have_1000000_brightness() {
    let lights: Lights = Lights::create();

    let new_lights: Lights = lights.turn_on(
        &LightsCoordinates::create(0, 0),
        &LightsCoordinates::create(999, 999),
    );

    assert_eq!(
        new_lights.get_number_of_lights(),
        new_lights.get_total_brightness()
    );
}

#[test]
fn given_lights_when_turn_on_lights_0_0_to_2_2_and_turn_off_lights_0_0_to_2_2_then_should_have_0_brightness(
) {
    let lights: Lights = Lights::create();

    let new_lights: Lights = lights
        .turn_on(
            &LightsCoordinates::create(0, 0),
            &LightsCoordinates::create(2, 2),
        )
        .turn_off(
            &LightsCoordinates::create(0, 0),
            &LightsCoordinates::create(2, 2),
        );

    assert_eq!(0, new_lights.get_total_brightness());
}

#[test]
fn given_lights_when_toggle_lights_0_0_to_2_2_and_toggle_lights_0_0_to_2_2_then_should_have_36_brightness(
) {
    let lights: Lights = Lights::create();

    let new_lights: Lights = lights
        .toggle(
            &LightsCoordinates::create(0, 0),
            &LightsCoordinates::create(2, 2),
        )
        .toggle(
            &LightsCoordinates::create(0, 0),
            &LightsCoordinates::create(2, 2),
        );

    assert_eq!(36, new_lights.get_total_brightness());
}

#[test]
fn given_lights_when_turn_on_lights_0_0_to_2_2_and_turn_on_lights_3_3_to_4_4_then_should_have_13_brightness(
) {
    let lights: Lights = Lights::create();

    let new_lights: Lights = lights
        .turn_on(
            &LightsCoordinates::create(0, 0),
            &LightsCoordinates::create(2, 2),
        )
        .turn_on(
            &LightsCoordinates::create(3, 3),
            &LightsCoordinates::create(4, 4),
        );

    assert_eq!(13, new_lights.get_total_brightness());
}

#[test]
fn given_lights_when_toggle_lights_0_0_to_2_2_and_toggle_lights_3_3_to_4_4_then_should_have_26_brightness(
) {
    let lights: Lights = Lights::create();

    let new_lights: Lights = lights
        .toggle(
            &LightsCoordinates::create(0, 0),
            &LightsCoordinates::create(2, 2),
        )
        .toggle(
            &LightsCoordinates::create(3, 3),
            &LightsCoordinates::create(4, 4),
        );

    assert_eq!(26, new_lights.get_total_brightness());
}

#[test]
fn given_lights_when_turn_on_lights_0_0_to_2_2_and_turn_on_lights_2_2_to_3_3_then_should_have_13_brightness(
) {
    let lights: Lights = Lights::create();

    let new_lights: Lights = lights
        .turn_on(
            &LightsCoordinates::create(0, 0),
            &LightsCoordinates::create(2, 2),
        )
        .turn_on(
            &LightsCoordinates::create(2, 2),
            &LightsCoordinates::create(3, 3),
        );

    assert_eq!(13, new_lights.get_total_brightness());
}

#[test]
fn given_lights_when_toggle_lights_0_0_to_2_2_and_toggle_lights_2_2_to_3_3_then_should_have_26_brightness(
) {
    let lights: Lights = Lights::create();
    let new_lights: Lights = lights
        .toggle(
            &LightsCoordinates::create(0, 0),
            &LightsCoordinates::create(2, 2),
        )
        .toggle(
            &LightsCoordinates::create(2, 2),
            &LightsCoordinates::create(3, 3),
        );

    assert_eq!(26, new_lights.get_total_brightness());
}

#[test]
fn given_lights_when_turn_on_lights_0_0_to_2_2_and_turn_off_lights_1_1_to_1_2_then_should_have_7_brightness(
) {
    let lights: Lights = Lights::create();

    let new_lights: Lights = lights
        .turn_on(
            &LightsCoordinates::create(0, 0),
            &LightsCoordinates::create(2, 2),
        )
        .turn_off(
            &LightsCoordinates::create(1, 1),
            &LightsCoordinates::create(1, 2),
        );

    assert_eq!(7, new_lights.get_total_brightness());
}

#[test]
fn given_lights_when_turn_on_lights_0_0_to_2_2_and_turn_off_lights_1_1_to_1_2_and_turn_off_lights_0_1_to_1_2_and_turn_on_lights_0_0_to_0_0_then_should_have_6_brightness(
) {
    let lights: Lights = Lights::create();

    let new_lights: Lights = lights
        .turn_on(
            &LightsCoordinates::create(0, 0),
            &LightsCoordinates::create(2, 2),
        )
        .turn_off(
            &LightsCoordinates::create(1, 1),
            &LightsCoordinates::create(1, 2),
        )
        .turn_off(
            &LightsCoordinates::create(0, 1),
            &LightsCoordinates::create(1, 2),
        )
        .turn_on(
            &LightsCoordinates::create(0, 0),
            &LightsCoordinates::create(0, 0),
        );

    assert_eq!(6, new_lights.get_total_brightness());
}
