use std::collections::HashSet;

mod gossips_tests;

#[derive(Copy, Clone, PartialEq, Eq)]
struct Stop {
    pub id: u32,
}

impl Stop {
    fn from(id: u32) -> Stop {
        Stop { id }
    }
}

#[derive(Clone)]
struct Route {
    pub list_of_stops: Vec<Stop>,
}

impl Route {
    fn from(list_of_stops: Vec<Stop>) -> Route {
        Route { list_of_stops }
    }

    fn num_of_stops(&self) -> u32 {
        self.list_of_stops.len() as u32
    }
}

struct Routes {
    pub list_of_routes: Vec<Route>,
}

impl Routes {
    fn from(list_of_routes: Vec<Route>) -> Routes {
        Routes { list_of_routes }
    }

    fn from_vec(list_of_routes: Vec<Vec<u32>>) -> Routes {
        Routes::from(
            list_of_routes
                .iter()
                .map(|route| {
                    let stops: Vec<Stop> =
                        route.iter().map(|&stop_id| Stop::from(stop_id)).collect();
                    Route::from(stops)
                })
                .collect(),
        )
    }

    fn num_of_routes(&self) -> u32 {
        self.list_of_routes.len() as u32
    }
}

struct FullDayRoute {
    pub list_of_stops: Vec<Stop>,
}

impl FullDayRoute {
    fn from(route: Route) -> FullDayRoute {
        let num_of_repeats = (MAX_STOPS + route.num_of_stops() - 1) / route.num_of_stops();
        FullDayRoute {
            list_of_stops: route.list_of_stops.repeat(num_of_repeats as usize)
                [0..MAX_STOPS as usize]
                .to_vec(),
        }
    }

    fn get_stop_at_time(&self, t: u32) -> Stop {
        self.list_of_stops.get(t as usize).unwrap().to_owned()
    }
}

struct FullDayRoutes {
    pub list_of_routes: Vec<FullDayRoute>,
}

impl FullDayRoutes {
    fn from(routes: Routes) -> FullDayRoutes {
        FullDayRoutes {
            list_of_routes: routes
                .list_of_routes
                .iter()
                .map(|route| FullDayRoute::from(route.clone()))
                .collect(),
        }
    }

    fn num_of_routes(&self) -> u32 {
        self.list_of_routes.len() as u32
    }

    fn get_all_stops_at(&self, time_t: u32) -> Vec<Stop> {
        self.list_of_routes
            .iter()
            .map(|route| route.get_stop_at_time(time_t).to_owned())
            .collect()
    }

    fn get_all_drivers_pairs_that_meet_at(&self, time_t: u32) -> Vec<(u32, u32)> {
        let all_stops_at_time_t: Vec<Stop> = self.get_all_stops_at(time_t);

        let mut equal_pairs: Vec<(u32, u32)> = Vec::new();
        for i in 0..all_stops_at_time_t.len() {
            for j in i + 1..all_stops_at_time_t.len() {
                if all_stops_at_time_t.get(i).unwrap() == all_stops_at_time_t.get(j).unwrap() {
                    equal_pairs.push((i as u32, j as u32));
                }
            }
        }

        equal_pairs
    }
}
struct Driver {
    driver_id: u32,
    known_secrets: HashSet<u32>,
}

impl Driver {
    fn init(id: u32) -> Driver {
        Driver {
            driver_id: id,
            known_secrets: HashSet::from([id]),
        }
    }

    fn know_secret_from_driver(&self, other_driver: &Driver) -> Driver {
        let new_known_secrets = self.known_secrets.union(&other_driver.known_secrets);

        Driver {
            driver_id: self.driver_id,
            known_secrets: new_known_secrets.cloned().collect(),
        }
    }
}

struct Drivers {
    drivers: Vec<Driver>,
}

impl Drivers {
    fn for_each_route(routes: &FullDayRoutes) -> Drivers {
        let mut drivers: Vec<Driver> = Vec::new();
        for i in 0..routes.num_of_routes() {
            drivers.push(Driver::init(i));
        }
        Drivers { drivers }
    }

    fn share_secrets_between(&mut self, driver_id_1: u32, driver_id_2: u32) {
        self.drivers[driver_id_1 as usize] = self.drivers[driver_id_1 as usize]
            .know_secret_from_driver(&self.drivers[driver_id_2 as usize]);
        self.drivers[driver_id_2 as usize] = self.drivers[driver_id_2 as usize]
            .know_secret_from_driver(&self.drivers[driver_id_1 as usize]);
    }

    fn do_all_drivers_know_all_secrets(&self) -> bool {
        self.drivers
            .iter()
            .all(|driver| driver.known_secrets.len() == self.drivers.len())
    }
}

#[derive(Debug, PartialEq)]
enum GossipError {
    GossipImpossible,
}
const MAX_STOPS: u32 = 480;

fn get_time_when_all_drivers_know_all_secrets(routes: Routes) -> Result<u32, GossipError> {
    if routes.num_of_routes() == 1 {
        return Ok(1);
    }

    let full_day_routes: FullDayRoutes = FullDayRoutes::from(routes);
    let mut drivers: Drivers = Drivers::for_each_route(&full_day_routes);

    check_all_drivers_for_all_possible_times(full_day_routes, &mut drivers)
}

fn check_all_drivers_for_all_possible_times(
    full_day_routes: FullDayRoutes,
    drivers: &mut Drivers,
) -> Result<u32, GossipError> {
    for time_t in 0..MAX_STOPS {
        if all_drivers_know_everything_at_time_t(&full_day_routes, time_t, drivers) {
            return Ok(time_t + 1);
        }
    }

    Err(GossipError::GossipImpossible)
}

fn all_drivers_know_everything_at_time_t(
    full_day_routes: &FullDayRoutes,
    time_t: u32,
    drivers: &mut Drivers,
) -> bool {
    let all_drivers_that_meet_at_time_i =
        full_day_routes.get_all_drivers_pairs_that_meet_at(time_t);

    all_drivers_that_meet_at_time_i_share_secrets_with_one_another(
        all_drivers_that_meet_at_time_i,
        drivers,
    );

    drivers.do_all_drivers_know_all_secrets()
}
fn all_drivers_that_meet_at_time_i_share_secrets_with_one_another(
    all_drivers_that_meet_at_time_i: Vec<(u32, u32)>,
    drivers: &mut Drivers,
) {
    for (driver_id_1, driver_id_2) in all_drivers_that_meet_at_time_i {
        drivers.share_secrets_between(driver_id_1, driver_id_2);
    }
}
