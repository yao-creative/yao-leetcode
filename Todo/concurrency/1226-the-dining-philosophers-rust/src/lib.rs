use std::sync::{Arc, Mutex};
use std::thread;

pub struct DiningPhilosophers;

impl DiningPhilosophers {
    pub fn new() -> Self {
        Self
    }

    pub fn wants_to_eat<PL, PR, E, UL, UR>(
        &self,
        philosopher: i32,
        pick_left_fork: PL,
        pick_right_fork: PR,
        eat: E,
        put_left_fork: UL,
        put_right_fork: UR,
    ) where
        PL: Fn() + Send + 'static,
        PR: Fn() + Send + 'static,
        E: Fn() + Send + 'static,
        UL: Fn() + Send + 'static,
        UR: Fn() + Send + 'static,
    {
        let _ = philosopher;
        let _ = &pick_left_fork;
        let _ = &pick_right_fork;
        let _ = &eat;
        let _ = &put_left_fork;
        let _ = &put_right_fork;
        todo!("implement deadlock-free dining protocol");
    }
}

fn run_case(order: &[i32]) -> Vec<String> {
    let dining = Arc::new(DiningPhilosophers::new());
    let log = Arc::new(Mutex::new(Vec::new()));
    let mut handles = Vec::with_capacity(order.len());

    for &philosopher in order {
        let dining = Arc::clone(&dining);
        let log = Arc::clone(&log);
        handles.push(thread::spawn(move || {
            let base = philosopher;
            dining.wants_to_eat(
                base,
                {
                    let log = Arc::clone(&log);
                    move || log.lock().unwrap().push(format!("{base}:pick_left"))
                },
                {
                    let log = Arc::clone(&log);
                    move || log.lock().unwrap().push(format!("{base}:pick_right"))
                },
                {
                    let log = Arc::clone(&log);
                    move || log.lock().unwrap().push(format!("{base}:eat"))
                },
                {
                    let log = Arc::clone(&log);
                    move || log.lock().unwrap().push(format!("{base}:put_left"))
                },
                move || log.lock().unwrap().push(format!("{base}:put_right")),
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
        let dining = DiningPhilosophers::new();
        let _ = dining;
    }

    #[test]
    #[ignore = "remove ignore after implementing DiningPhilosophers::wants_to_eat"]
    fn case_single_philosopher() {
        assert_eq!(
            run_case(&[0]),
            vec![
                "0:pick_left",
                "0:pick_right",
                "0:eat",
                "0:put_left",
                "0:put_right",
            ]
        );
    }

    #[test]
    #[ignore = "remove ignore after implementing DiningPhilosophers::wants_to_eat"]
    fn case_all_philosophers_eat_once() {
        let log = run_case(&[0, 1, 2, 3, 4]);
        for philosopher in 0..5 {
            assert!(log.contains(&format!("{philosopher}:eat")));
        }
        assert_eq!(log.len(), 25);
    }
}
