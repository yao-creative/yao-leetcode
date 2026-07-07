use std::sync::{Arc, Mutex};
use std::thread;

pub struct ZeroEvenOdd {
    n: i32,
}

impl ZeroEvenOdd {
    pub fn new(n: i32) -> Self {
        Self { n }
    }

    pub fn zero<F>(&self, print_number: F)
    where
        F: Fn(i32) + Send + 'static,
    {
        let _ = &print_number;
        let _ = self.n;
        todo!("implement coordination for zero");
    }

    pub fn even<F>(&self, print_number: F)
    where
        F: Fn(i32) + Send + 'static,
    {
        let _ = &print_number;
        let _ = self.n;
        todo!("implement coordination for even");
    }

    pub fn odd<F>(&self, print_number: F)
    where
        F: Fn(i32) + Send + 'static,
    {
        let _ = &print_number;
        let _ = self.n;
        todo!("implement coordination for odd");
    }
}

fn run_case(n: i32) -> Vec<i32> {
    let zero_even_odd = Arc::new(ZeroEvenOdd::new(n));
    let out = Arc::new(Mutex::new(Vec::with_capacity((n * 2) as usize)));

    let zero_runner = {
        let zero_even_odd = Arc::clone(&zero_even_odd);
        let out = Arc::clone(&out);
        thread::spawn(move || {
            zero_even_odd.zero(move |value| {
                out.lock().unwrap().push(value);
            });
        })
    };

    let even_runner = {
        let zero_even_odd = Arc::clone(&zero_even_odd);
        let out = Arc::clone(&out);
        thread::spawn(move || {
            zero_even_odd.even(move |value| {
                out.lock().unwrap().push(value);
            });
        })
    };

    let odd_runner = {
        let zero_even_odd = Arc::clone(&zero_even_odd);
        let out = Arc::clone(&out);
        thread::spawn(move || {
            zero_even_odd.odd(move |value| {
                out.lock().unwrap().push(value);
            });
        })
    };

    zero_runner.join().unwrap();
    even_runner.join().unwrap();
    odd_runner.join().unwrap();

    Arc::try_unwrap(out).unwrap().into_inner().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scaffold_builds() {
        let zero_even_odd = ZeroEvenOdd::new(3);
        let _ = zero_even_odd;
    }

    #[test]
    #[ignore = "remove ignore after implementing ZeroEvenOdd"]
    fn case_n2() {
        assert_eq!(run_case(2), vec![0, 1, 0, 2]);
    }

    #[test]
    #[ignore = "remove ignore after implementing ZeroEvenOdd"]
    fn case_n5() {
        assert_eq!(run_case(5), vec![0, 1, 0, 2, 0, 3, 0, 4, 0, 5]);
    }
}
