use std::collections::HashSet;

mod gossips_tests;

#[derive(Copy, Clone)]
struct Stop {
    pub id: u32,
}

impl Stop {
    fn from(id: u32) -> Stop {
        Stop { id }
    }
}

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
                .map(|route| FullDayRoute::from(*route))
                .collect(),
        }
    }

    fn num_of_routes(&self) -> u32 {
        self.list_of_routes.len() as u32
    }

    fn get_all_stops_at_time_i(&self, time: u32) -> Vec<Stop> {
        self.list_of_routes
            .iter()
            .map(|route| route.get_stop_at_time(time).to_owned())
            .collect()
    }

    fn get_all_drivers_that_meet_at_time(&self, t: u32) {
        let all_stops_at_time_t = self.get_all_stops_at_time_i()
    }
}

struct KnownSecretsByDriver {
    secrets: HashSet<u32>,
}

impl KnownSecretsByDriver {
    fn init(id: u32) -> KnownSecretsByDriver {
        KnownSecretsByDriver {
            secrets: HashSet::from([id]),
        }
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

    check_all_drivers_for_all_possible_times(full_day_routes)
}

fn check_all_drivers_for_all_possible_times(
    full_day_routes: FullDayRoutes,
) -> Result<u32, GossipError> {
    let mut known_secrets_per_driver: Vec<KnownSecretsByDriver> = Vec::new();
    for i in 0..full_day_routes.num_of_routes() {
        known_secrets_per_driver.push(KnownSecretsByDriver::init(i));
    }

    for i in 0..MAX_STOPS {
        if do_all_drivers_at_time_i_know_everything(
            &full_day_routes,
            i,
            &mut known_secrets_per_driver,
        ) {
            return Ok(i as u32 + 1);
        }
    }

    Err(GossipError::GossipImpossible)
}

fn do_all_drivers_at_time_i_know_everything(
    full_day_routes: &FullDayRoutes,
    i: usize,
    known_secrets_per_driver: &mut Vec<HashSet<u32>>,
) -> bool {
    let all_stops_at_time_i = full_day_routes.get_all_stops_at_time_i(i as u32);
    let all_drivers_that_meet_at_time_i = get_all_drivers_that_meet_at_time_i(all_stops_at_time_i);
    all_drivers_that_meet_at_time_i_share_secrets_with_one_another(
        all_drivers_that_meet_at_time_i,
        known_secrets_per_driver,
    );

    does_all_drivers_know_all_secrets_at_time_i(&*known_secrets_per_driver, full_routes)
}

fn does_all_drivers_know_all_secrets_at_time_i(
    known_secrets_per_driver: &Vec<HashSet<u32>>,
    full_routes: &Vec<Vec<u32>>,
) -> bool {
    let does_all_drivers_know_all_secrets: bool = known_secrets_per_driver
        .iter()
        .all(|secret_per_driver| secret_per_driver.len() == full_routes.len());
    does_all_drivers_know_all_secrets
}

fn all_drivers_that_meet_at_time_i_share_secrets_with_one_another(
    all_drivers_that_meet_at_time_i: Vec<(u32, u32)>,
    known_secrets_per_driver: &mut Vec<HashSet<u32>>,
) {
    for (driver_1, driver_2) in all_drivers_that_meet_at_time_i {
        drivers_share_secrets(known_secrets_per_driver, driver_1, driver_2);
    }
}

fn get_all_stops_at_time_i(full_routes: &Vec<Vec<u32>>, i: usize) -> Vec<u32> {
    let all_stops_at_time_i = full_routes
        .iter()
        .map(|route| route.get(i).unwrap().to_owned())
        .collect();
    all_stops_at_time_i
}

fn drivers_share_secrets(
    known_secrets_per_driver: &mut Vec<HashSet<u32>>,
    driver_1: u32,
    driver_2: u32,
) {
    known_secrets_per_driver[driver_1 as usize] = known_secrets_per_driver
        .get(driver_1 as usize)
        .map(|secrets_driver_1| {
            let get = known_secrets_per_driver.get(driver_2 as usize).unwrap();
            secrets_driver_1.union(get).cloned().collect()
        })
        .unwrap();

    known_secrets_per_driver[driver_2 as usize] = known_secrets_per_driver
        .get(driver_2 as usize)
        .map(|secrets_driver_2| {
            let get = known_secrets_per_driver.get(driver_1 as usize).unwrap();
            secrets_driver_2.union(get).cloned().collect()
        })
        .unwrap();
}

fn get_all_drivers_that_meet_at_time_i(vec: Vec<u32>) -> Vec<(u32, u32)> {
    let mut equal_pairs: Vec<(u32, u32)> = Vec::new();
    for i in 0..vec.len() {
        for j in i + 1..vec.len() {
            if vec.get(i).unwrap() == vec.get(j).unwrap() {
                equal_pairs.push((i as u32, j as u32));
            }
        }
    }
    equal_pairs
}
