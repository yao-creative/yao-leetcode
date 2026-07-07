use std::sync::{Arc, Mutex};
use std::thread;

pub struct TrafficLight;

impl TrafficLight {
    pub fn new() -> Self {
        Self
    }

    pub fn car_arrived<FG, CC>(
        &self,
        car_id: i32,
        road_id: i32,
        direction: i32,
        turn_green: FG,
        cross_car: CC,
    ) where
        FG: Fn() + Send + 'static,
        CC: Fn() + Send + 'static,
    {
        let _ = car_id;
        let _ = road_id;
        let _ = direction;
        let _ = &turn_green;
        let _ = &cross_car;
        todo!("implement serialized traffic-light control");
    }
}

fn run_case(cars: &[(i32, i32, i32)]) -> Vec<String> {
    let traffic_light = Arc::new(TrafficLight::new());
    let log = Arc::new(Mutex::new(Vec::new()));
    let mut handles = Vec::with_capacity(cars.len());

    for &(car_id, road_id, direction) in cars {
        let traffic_light = Arc::clone(&traffic_light);
        let log = Arc::clone(&log);
        handles.push(thread::spawn(move || {
            traffic_light.car_arrived(
                car_id,
                road_id,
                direction,
                {
                    let log = Arc::clone(&log);
                    move || log.lock().unwrap().push(format!("{car_id}:green:{road_id}"))
                },
                move || log.lock().unwrap().push(format!("{car_id}:cross")),
            );
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    Arc::try_unwrap(log).unwrap().into_inner().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scaffold_builds() {
        let traffic_light = TrafficLight::new();
        let _ = traffic_light;
    }

    #[test]
    #[ignore = "remove ignore after implementing TrafficLight::car_arrived"]
    fn case_single_car_same_road() {
        assert_eq!(run_case(&[(1, 1, 2)]), vec!["1:cross"]);
    }

    #[test]
    #[ignore = "remove ignore after implementing TrafficLight::car_arrived"]
    fn case_switch_then_cross() {
        let log = run_case(&[(1, 1, 2), (2, 2, 1)]);
        assert_eq!(log.len(), 3);
        assert!(log.contains(&"1:cross".to_string()));
        assert!(log.contains(&"2:cross".to_string()));
        assert!(log.iter().any(|entry| entry == "2:green:2"));
    }
}
