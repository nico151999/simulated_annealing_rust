use rand::seq::SliceRandom;
use rand::thread_rng;
use crate::city::City;

// must be 1 or greater
const ROUTE_LENGTH: usize = 24;

pub type Route = [City; ROUTE_LENGTH];

const CITY_COORDINATES: [(f64, f64); ROUTE_LENGTH] = [
    (2.7933,3.694),
    (2.6067,4.4254),
    (2.86,5.0373),
    (2.54,6.2463),
    (3.1267,6.4701),
    (3.7267,6.8881),
    (4.4867,7.4403),
    (5.5533,7.4254),
    (6.3,7.3955),
    (7.6333,6.9179),
    (7.22,6.3955),
    (6.6333,5.8284),
    (7.0867,5.1269),
    (7.4733,4.4701),
    (7.18,3.709),
    (6.6867,2.8284),
    (6.2067,2.0522),
    (5.54,1.8731),
    (5.1533,2.3358),
    (4.9667,3.0075),
    (4.8867,3.5448),
    (4.2733,3.2313),
    (3.6333,2.7537),
    (2.9933,2.8433)
];

pub trait RouteActions {
    fn print(&self);
    fn calc_cost(&self) -> f64;
    fn is_shorter_than(&self, other: &Self) -> bool;
    fn greedily_generate_route() -> Route {
        let coordinate = CITY_COORDINATES[0];
        let mut route: Route = [City::new(coordinate.0, coordinate.1); ROUTE_LENGTH];
        for i in 0..ROUTE_LENGTH-1 {
            let mut lowest_cost: Option<f64> = None;
            let mut element: Option<City> = None;
            for coordinate in CITY_COORDINATES[1..ROUTE_LENGTH].iter() {
                let city = City::new(coordinate.0, coordinate.1);
                if route.contains(&city) {
                    continue;
                }
                let cost = route[i].distance_to(&city);
                if let Some(temp) = lowest_cost {
                    if cost < temp {
                        lowest_cost = Some(cost);
                        element = Some(city);
                    }
                } else {
                    lowest_cost = Some(cost);
                    element = Some(city);
                }
            }
            route[i+1] = element.expect(
                "failed finding the next element initialising route greedily"
            );
        }
        route
    }
    fn generate_route() -> Route {
        let coordinate = CITY_COORDINATES[0];
        let mut route: Route = [City::new(coordinate.0, coordinate.1); ROUTE_LENGTH];
        for (i, coordinate) in CITY_COORDINATES[1..ROUTE_LENGTH].iter().enumerate() {
            route[i] = City::new(coordinate.0, coordinate.1);
        }
        route
    }
    fn generate_random_route() -> Route {
        let mut route = Self::generate_route();
        route.shuffle(&mut thread_rng());
        route
    }
}

impl RouteActions for Route {
    fn print(&self) {
        match self.first() {
            None => {},
            Some(first) => {
                for route_item in self {
                    print!("{} -> ", route_item);
                }
                println!("{}", first);
            },
        }
    }
    fn calc_cost(&self) -> f64 {
        let mut cost: f64 = 0.0;
        let n = self.len();
        for i in 1..n {
            cost += self[i-1].distance_to(&self[i])
        }
        cost + self[n-1].distance_to(&self[0])
    }
    fn is_shorter_than(&self, other: &Route) -> bool {
        self.calc_cost() < other.calc_cost()
    }
}
