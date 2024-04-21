use std::collections::{HashMap, HashSet};

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
    turned_on_lights: HashMap<LightsCoordinates, u32>,
}

impl Lights {
    pub fn create() -> Lights {
        Lights {
            turned_on_lights: HashMap::new(),
        }
    }

    pub fn get_number_of_lights(&self) -> u32 {
        NUM_OF_LIGHTS
    }

    pub fn get_total_brightness(&self) -> u32 {
        self.turned_on_lights.clone().into_values().sum()
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

        let mut copy_brightness = self.turned_on_lights.clone();
        newly_turned_on_lights.iter().for_each(|light| {
            copy_brightness.insert(
                light.clone(),
                self.turned_on_lights.get(light).unwrap_or(&0_u32) + 1,
            );
        });

        Lights {
            turned_on_lights: copy_brightness,
        }
    }

    pub fn turn_off(
        &self,
        first_coord: &LightsCoordinates,
        second_coord: &LightsCoordinates,
    ) -> Lights {
        let newly_turned_off_lights: HashSet<LightsCoordinates> =
            self.get_grid_set(first_coord, second_coord);
        let mut copy_brightness = self.turned_on_lights.clone();
        newly_turned_off_lights
            .iter()
            .for_each(|light| match copy_brightness.get(light) {
                Some(x) => match x {
                    1 => {
                        copy_brightness.remove(light);
                    }
                    _ => {
                        copy_brightness.insert(light.clone(), x - 1).unwrap();
                    }
                },
                None => (),
            });
        Lights {
            turned_on_lights: copy_brightness,
        }
    }

    pub fn toggle(
        &self,
        first_coord: &LightsCoordinates,
        second_coord: &LightsCoordinates,
    ) -> Lights {
        let newly_toggled_lights: HashSet<LightsCoordinates> =
            self.get_grid_set(first_coord, second_coord);
        let mut copy_brightness = self.turned_on_lights.clone();
        newly_toggled_lights.iter().for_each(|light| {
            copy_brightness.insert(
                light.clone(),
                self.turned_on_lights.get(light).unwrap_or(&0_u32) + 2,
            );
        });

        Lights {
            turned_on_lights: copy_brightness,
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
