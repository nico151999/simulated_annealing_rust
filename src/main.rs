mod route;
mod city;

use std::f64::consts::E;
use rand::{Rng, thread_rng};
use crate::route::{Route, RouteActions};

fn main() {
    // to use the greedy or random approach just use the respective generator here
    let mut best_route: Route = Route::generate_route();
    print!("Initial route with cost {:.2}: ", best_route.calc_cost());
    best_route.print();
    {
        const MIN_TEMPERATURE: f64 = 0.0005;
        const ALPHA: f64 = 0.995;
        let mut temp: f64 = 10.0;
        let mut cur_route = best_route;
        let route_len = cur_route.len();
        let mut rng = thread_rng();
        while temp > MIN_TEMPERATURE {
            let mut new_route = cur_route;
            new_route.swap(
                rng.gen_range(0..route_len),
                rng.gen_range(0..route_len)
            );
            let cur_cost = cur_route.calc_cost();
            let new_cost = new_route.calc_cost();
            if E.powf(-(cur_cost - new_cost).abs() / temp) > rng.gen_range(0.0..=1.0) {
                cur_route = new_route;
            }
            if cur_cost < best_route.calc_cost() {
                best_route = cur_route;
                print!("New best route with cost {:.2}: ", best_route.calc_cost());
                best_route.print();
            }
            temp *= ALPHA;
        }
    }
    print!("Final route with cost {:.2}: ", best_route.calc_cost());
    best_route.print();
}
