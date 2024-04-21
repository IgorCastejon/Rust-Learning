use std::collections::HashSet;

mod tests;

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct LightsCoordinates {
    x: u32,
    y: u32,
}

impl LightsCoordinates {
    pub fn create(x: u32, y: u32) -> LightsCoordinates {
        LightsCoordinates { x, y }
    }
}

fn is_light_in_grid(
    grid_start: &LightsCoordinates,
    grid_end: &LightsCoordinates,
    pos: &LightsCoordinates,
) -> bool {
    grid_start.x <= pos.x && grid_end.x >= pos.x && grid_start.y <= pos.y && grid_end.y >= pos.y
}

static NUM_OF_LIGHTS: u32 = 1000 * 1000;

pub struct Lights {
    turned_on_lights: HashSet<LightsCoordinates>,
}

impl Lights {
    pub fn create() -> Lights {
        Lights {
            turned_on_lights: HashSet::new(),
        }
    }

    pub fn get_number_of_lights(&self) -> u32 {
        NUM_OF_LIGHTS
    }

    pub fn get_number_of_turned_off_lights(&self) -> u32 {
        NUM_OF_LIGHTS - self.get_number_of_turned_on_lights()
    }

    pub fn get_number_of_turned_on_lights(&self) -> u32 {
        self.turned_on_lights.len() as u32
    }

    pub fn turn_on(
        &self,
        first_coord: &LightsCoordinates,
        second_coord: &LightsCoordinates,
    ) -> Lights {
        let newly_turned_on_lights: HashSet<LightsCoordinates> =
            self.get_grid_set(first_coord, second_coord);
        let union_turned_on_lights: HashSet<LightsCoordinates> = self
            .turned_on_lights
            .union(&newly_turned_on_lights)
            .cloned()
            .collect();

        Lights {
            turned_on_lights: union_turned_on_lights,
        }
    }

    pub fn turn_off(
        &self,
        first_coord: &LightsCoordinates,
        second_coord: &LightsCoordinates,
    ) -> Lights {
        let newly_turned_off_lights: HashSet<LightsCoordinates> =
            self.get_grid_set(first_coord, second_coord);
        let difference_turned_on_lights: HashSet<LightsCoordinates> = self
            .turned_on_lights
            .difference(&newly_turned_off_lights)
            .cloned()
            .collect();

        Lights {
            turned_on_lights: difference_turned_on_lights,
        }
    }

    pub fn toggle(
        &self,
        first_coord: &LightsCoordinates,
        second_coord: &LightsCoordinates,
    ) -> Lights {
        let newly_toggled_lights: HashSet<LightsCoordinates> =
            self.get_grid_set(first_coord, second_coord);
        let symmetric_difference_toggled_lights: HashSet<LightsCoordinates> = self
            .turned_on_lights
            .symmetric_difference(&newly_toggled_lights)
            .cloned()
            .collect();

        Lights {
            turned_on_lights: symmetric_difference_toggled_lights,
        }
    }

    fn get_grid_set(
        &self,
        first_coord: &LightsCoordinates,
        second_coord: &LightsCoordinates,
    ) -> HashSet<LightsCoordinates> {
        let mut newly_turned_off_lights: HashSet<LightsCoordinates> = HashSet::new();

        for x in first_coord.x..=second_coord.x {
            for y in first_coord.y..=second_coord.y {
                newly_turned_off_lights.insert(LightsCoordinates::create(x, y));
            }
        }
        newly_turned_off_lights
    }
}
