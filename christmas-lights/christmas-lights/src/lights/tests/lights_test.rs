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
fn given_initial_lights_when_get_number_of_turned_off_lights_then_should_return_all() {
    let lights: Lights = Lights::create();
    let num_of_turned_of_lights = lights.get_number_of_turned_off_lights();
    assert_eq!(lights.get_number_of_lights(), num_of_turned_of_lights);
}

#[test]
fn given_initial_lights_when_get_number_of_turned_on_lights_then_should_return_0() {
    let lights: Lights = Lights::create();
    let num_of_turned_on_lights = lights.get_number_of_turned_on_lights();
    assert_eq!(0, num_of_turned_on_lights);
}

#[test]
fn given_lights_when_turn_on_lights_0_0_to_2_2_then_should_light_9_lights() {
    let lights: Lights = Lights::create();
    let first_lights_coordinate = LightsCoordinates::create(0, 0);
    let second_lights_coordinate = LightsCoordinates::create(2, 2);
    let new_lights: Lights = lights.turn_on(&first_lights_coordinate, &second_lights_coordinate);

    assert_eq!(9, new_lights.get_number_of_turned_on_lights());
    assert_eq!(
        new_lights.get_number_of_lights() - new_lights.get_number_of_turned_on_lights(),
        new_lights.get_number_of_turned_off_lights()
    );
}

#[test]
fn given_lights_when_toggle_lights_0_0_to_2_2_then_should_light_9_lights() {
    let lights: Lights = Lights::create();
    let new_lights: Lights = lights.toggle(
        &LightsCoordinates::create(0, 0),
        &LightsCoordinates::create(2, 2),
    );

    assert_eq!(9, new_lights.get_number_of_turned_on_lights());
    assert_eq!(
        new_lights.get_number_of_lights() - new_lights.get_number_of_turned_on_lights(),
        new_lights.get_number_of_turned_off_lights()
    );
}

#[test]
fn given_lights_when_turn_on_lights_0_0_to_999_999_then_should_light_all_lights() {
    let lights: Lights = Lights::create();
    let first_lights_coordinate = LightsCoordinates::create(0, 0);
    let second_lights_coordinate = LightsCoordinates::create(999, 999);
    let new_lights: Lights = lights.turn_on(&first_lights_coordinate, &second_lights_coordinate);

    assert_eq!(
        new_lights.get_number_of_lights(),
        new_lights.get_number_of_turned_on_lights()
    );
    assert_eq!(0, new_lights.get_number_of_turned_off_lights());
}

#[test]
fn given_lights_when_turn_on_lights_0_0_to_2_2_and_turn_off_lights_0_0_to_2_2_then_should_have_0_turned_on_lights(
) {
    let lights: Lights = Lights::create();
    let first_lights_coordinate = LightsCoordinates::create(0, 0);
    let second_lights_coordinate = LightsCoordinates::create(2, 2);
    let new_lights_turned_on: Lights =
        lights.turn_on(&first_lights_coordinate, &second_lights_coordinate);

    let new_lights_turned_off: Lights =
        new_lights_turned_on.turn_off(&first_lights_coordinate, &second_lights_coordinate);

    assert_eq!(
        new_lights_turned_off.get_number_of_lights(),
        new_lights_turned_off.get_number_of_turned_off_lights()
    );
    assert_eq!(0, new_lights_turned_off.get_number_of_turned_on_lights());
}

#[test]
fn given_lights_when_toggle_lights_0_0_to_2_2_and_toggle_lights_0_0_to_2_2_then_should_have_0_turned_on_lights(
) {
    let lights: Lights = Lights::create();

    let new_lights_turned_on: Lights = lights.toggle(
        &LightsCoordinates::create(0, 0),
        &LightsCoordinates::create(2, 2),
    );

    let new_lights_turned_off: Lights = new_lights_turned_on.toggle(
        &LightsCoordinates::create(0, 0),
        &LightsCoordinates::create(2, 2),
    );

    assert_eq!(
        new_lights_turned_off.get_number_of_lights(),
        new_lights_turned_off.get_number_of_turned_off_lights()
    );
    assert_eq!(0, new_lights_turned_off.get_number_of_turned_on_lights());
}

#[test]
fn given_lights_when_turn_on_lights_0_0_to_2_2_and_turn_on_lights_3_3_to_4_4_then_should_have_13_turned_on_lights(
) {
    let lights: Lights = Lights::create();
    let new_lights_turned_on: Lights = lights.turn_on(
        &LightsCoordinates::create(0, 0),
        &LightsCoordinates::create(2, 2),
    );

    let new_lights_turned_on_again: Lights = new_lights_turned_on.turn_on(
        &LightsCoordinates::create(3, 3),
        &LightsCoordinates::create(4, 4),
    );

    assert_eq!(
        new_lights_turned_on_again.get_number_of_lights()
            - new_lights_turned_on_again.get_number_of_turned_on_lights(),
        new_lights_turned_on_again.get_number_of_turned_off_lights()
    );
    assert_eq!(
        13,
        new_lights_turned_on_again.get_number_of_turned_on_lights()
    );
}

#[test]
fn given_lights_when_toggle_lights_0_0_to_2_2_and_toggle_lights_3_3_to_4_4_then_should_have_13_turned_on_lights(
) {
    let lights: Lights = Lights::create();
    let new_lights_turned_on: Lights = lights.toggle(
        &LightsCoordinates::create(0, 0),
        &LightsCoordinates::create(2, 2),
    );

    let new_lights_turned_on_again: Lights = new_lights_turned_on.toggle(
        &LightsCoordinates::create(3, 3),
        &LightsCoordinates::create(4, 4),
    );

    assert_eq!(
        new_lights_turned_on_again.get_number_of_lights()
            - new_lights_turned_on_again.get_number_of_turned_on_lights(),
        new_lights_turned_on_again.get_number_of_turned_off_lights()
    );
    assert_eq!(
        13,
        new_lights_turned_on_again.get_number_of_turned_on_lights()
    );
}

#[test]
fn given_lights_when_turn_on_lights_0_0_to_2_2_and_turn_on_lights_2_2_to_3_3_then_should_have_12_turned_on_lights(
) {
    let lights: Lights = Lights::create();
    let new_lights_turned_on: Lights = lights.turn_on(
        &LightsCoordinates::create(0, 0),
        &LightsCoordinates::create(2, 2),
    );

    let new_lights_turned_on_again: Lights = new_lights_turned_on.turn_on(
        &LightsCoordinates::create(2, 2),
        &LightsCoordinates::create(3, 3),
    );

    assert_eq!(
        new_lights_turned_on_again.get_number_of_lights()
            - new_lights_turned_on_again.get_number_of_turned_on_lights(),
        new_lights_turned_on_again.get_number_of_turned_off_lights()
    );
    assert_eq!(
        12,
        new_lights_turned_on_again.get_number_of_turned_on_lights()
    );
}

#[test]
fn given_lights_when_toggle_lights_0_0_to_2_2_and_toggle_lights_2_2_to_3_3_then_should_have_11_turned_on_lights(
) {
    let lights: Lights = Lights::create();
    let new_lights_toggle: Lights = lights.toggle(
        &LightsCoordinates::create(0, 0),
        &LightsCoordinates::create(2, 2),
    );

    let new_lights_toggle_again: Lights = new_lights_toggle.toggle(
        &LightsCoordinates::create(2, 2),
        &LightsCoordinates::create(3, 3),
    );

    assert_eq!(
        new_lights_toggle_again.get_number_of_lights()
            - new_lights_toggle_again.get_number_of_turned_on_lights(),
        new_lights_toggle_again.get_number_of_turned_off_lights()
    );
    assert_eq!(11, new_lights_toggle_again.get_number_of_turned_on_lights());
}

#[test]
fn given_lights_when_turn_on_lights_0_0_to_2_2_and_turn_off_lights_1_1_to_1_2_then_should_have_7_turned_on_lights(
) {
    let lights: Lights = Lights::create();
    let first_lights_coordinate = LightsCoordinates::create(0, 0);
    let second_lights_coordinate = LightsCoordinates::create(2, 2);
    let new_lights_turned_on: Lights =
        lights.turn_on(&first_lights_coordinate, &second_lights_coordinate);

    let first_lights_coordinate_other = LightsCoordinates::create(1, 1);
    let second_lights_coordinate_other = LightsCoordinates::create(1, 2);
    let new_lights_turned_off: Lights = new_lights_turned_on.turn_off(
        &first_lights_coordinate_other,
        &second_lights_coordinate_other,
    );

    assert_eq!(
        new_lights_turned_off.get_number_of_lights()
            - new_lights_turned_off.get_number_of_turned_on_lights(),
        new_lights_turned_off.get_number_of_turned_off_lights()
    );
    assert_eq!(7, new_lights_turned_off.get_number_of_turned_on_lights());
}

#[test]
fn given_lights_when_turn_on_lights_0_0_to_2_2_and_turn_off_lights_1_1_to_1_2_and_turn_off_lights_0_1_to_1_2_and_turn_on_lights_0_0_to_0_0_then_should_have_5_turned_on_lights(
) {
    let lights: Lights = Lights::create();

    let new_lights_turned_on: Lights = lights.turn_on(
        &LightsCoordinates::create(0, 0),
        &LightsCoordinates::create(2, 2),
    );

    let new_lights_turned_off: Lights = new_lights_turned_on.turn_off(
        &LightsCoordinates::create(1, 1),
        &LightsCoordinates::create(1, 2),
    );

    let new_lights_turned_off_again: Lights = new_lights_turned_off.turn_off(
        &LightsCoordinates::create(0, 1),
        &LightsCoordinates::create(1, 2),
    );

    let new_lights_turned_on_again: Lights = new_lights_turned_off_again.turn_on(
        &LightsCoordinates::create(0, 0),
        &LightsCoordinates::create(0, 0),
    );

    assert_eq!(
        new_lights_turned_on_again.get_number_of_lights()
            - new_lights_turned_on_again.get_number_of_turned_on_lights(),
        new_lights_turned_on_again.get_number_of_turned_off_lights()
    );
    assert_eq!(
        5,
        new_lights_turned_on_again.get_number_of_turned_on_lights()
    );
}
