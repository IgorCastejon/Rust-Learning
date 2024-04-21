#[cfg(test)]
use crate::bus::get_time_when_all_drivers_know_all_secrets;
use crate::bus::{Route, Routes, Stop};
#[test]
fn given_one_driver_when_check_gossips_then_should_return_1() {
    let routes = [[1, 2, 3]];
    let result = get_time_when_all_drivers_know_all_secrets(routes.map(|f| f.to_vec()).to_vec());
    assert_eq!(1, result.unwrap());
}

#[test]
fn given_two_drivers_and_same_route_start_when_check_gossips_then_should_return_1() {
    let routes = [[1], [1]];
    let result = get_time_when_all_drivers_know_all_secrets(routes.map(|f| f.to_vec()).to_vec());
    assert_eq!(1, result.unwrap());
}

#[test]
fn given_two_drivers_and_different_routes_when_check_gossips_then_should_return_never() {
    let routes = [[1], [4]];
    let result = get_time_when_all_drivers_know_all_secrets(routes.map(|f| f.to_vec()).to_vec());
    assert!(result.is_err());
}

#[test]
fn given_two_drivers_and_same_route_middle_when_check_gossips_then_should_return_2() {
    let routes = [[1, 2], [4, 2]];
    let result = get_time_when_all_drivers_know_all_secrets(routes.map(|f| f.to_vec()).to_vec());
    assert_eq!(2, result.unwrap());
}

#[test]
fn given_two_drivers_and_route_that_intersect_in_middle_after_first_repeats_when_check_gossips_then_should_return_2(
) {
    let routes: Vec<Vec<u32>> = vec![vec![1], vec![4, 1]];
    let result = get_time_when_all_drivers_know_all_secrets(routes);
    assert_eq!(2, result.unwrap());
}

#[test]
fn given_two_drivers_and_route_that_intersect_in_middle_after_second_repeats_when_check_gossips_then_should_return_2(
) {
    let routes: Vec<Vec<u32>> = vec![vec![4, 1], vec![1]];
    let result = get_time_when_all_drivers_know_all_secrets(routes);
    assert_eq!(2, result.unwrap());
}

#[test]
fn given_three_drivers_and_route_that_dont_intersect_when_check_gossips_then_should_return_never() {
    let routes: Vec<Vec<u32>> = vec![vec![1], vec![2], vec![3]];
    let result = get_time_when_all_drivers_know_all_secrets(routes);
    assert!(result.is_err());
}

#[test]
fn given_three_drivers_and_route_that_first_and_two_intersect_but_not_three_when_check_gossips_then_should_return_never(
) {
    let routes: Vec<Vec<u32>> = vec![vec![1, 2], vec![2], vec![3]];
    let result = get_time_when_all_drivers_know_all_secrets(routes);
    assert!(result.is_err());
}

#[test]
fn given_three_drivers_and_route_that_first_and_third_intersect_but_not_two_when_check_gossips_then_should_return_never(
) {
    let routes: Vec<Vec<u32>> = vec![vec![3, 1], vec![2], vec![3]];
    let result = get_time_when_all_drivers_know_all_secrets(routes);
    assert!(result.is_err());
}

#[test]
fn given_three_drivers_and_route_that_first_and_third_intersect_at_stop_1_but_intersect_at_two_at_stop_2_when_check_gossips_then_should_return_2(
) {
    let routes: Vec<Vec<u32>> = vec![vec![3, 2], vec![2], vec![3, 2]];
    let result = get_time_when_all_drivers_know_all_secrets(routes);
    assert_eq!(2, result.unwrap());
}

#[test]
fn given_three_drivers_example_1_that_cross_when_check_gossips_then_should_return_5() {
    let first_route = vec![3, 1, 2, 3];
    let second_route = vec![3, 2, 3, 1];
    let third_route = vec![4, 2, 3, 4, 5];
    let routes: Vec<Vec<u32>> = vec![first_route, second_route, third_route];
    let result = get_time_when_all_drivers_know_all_secrets(routes);
    assert_eq!(5, result.unwrap());
}

#[test]
fn given_two_drivers_example_2_that_dont_cross_when_check_gossips_then_should_return_never() {
    let first_route = vec![2, 1, 2];
    let second_route = vec![5, 2, 8];
    let routes: Vec<Vec<u32>> = vec![first_route, second_route];
    let result = get_time_when_all_drivers_know_all_secrets(routes);
    assert!(result.is_err());
}
