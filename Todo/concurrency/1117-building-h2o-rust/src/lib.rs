use std::sync::{Arc, Mutex};
use std::thread;

pub struct H2O;

impl H2O {
    pub fn new() -> Self {
        Self
    }

    pub fn hydrogen<F>(&self, release_hydrogen: F)
    where
        F: Fn() + Send + 'static,
    {
        let _ = &release_hydrogen;
        todo!("implement batch coordination for hydrogen");
    }

    pub fn oxygen<F>(&self, release_oxygen: F)
    where
        F: Fn() + Send + 'static,
    {
        let _ = &release_oxygen;
        todo!("implement batch coordination for oxygen");
    }
}

fn run_case(sequence: &str) -> Vec<char> {
    let h2o = Arc::new(H2O::new());
    let out = Arc::new(Mutex::new(Vec::with_capacity(sequence.len())));
    let mut handles = Vec::with_capacity(sequence.len());

    for atom in sequence.chars() {
        let h2o = Arc::clone(&h2o);
        let out = Arc::clone(&out);
        let handle = thread::spawn(move || match atom {
            'H' => h2o.hydrogen(move || out.lock().unwrap().push('H')),
            'O' => h2o.oxygen(move || out.lock().unwrap().push('O')),
            _ => panic!("unexpected atom"),
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    Arc::try_unwrap(out).unwrap().into_inner().unwrap()
}

fn assert_is_valid_h2o_output(out: &[char]) {
    assert_eq!(out.len() % 3, 0);
    for chunk in out.chunks(3) {
        let mut atoms = chunk.to_vec();
        atoms.sort_unstable();
        assert_eq!(atoms, vec!['H', 'H', 'O']);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scaffold_builds() {
        let h2o = H2O::new();
        let _ = h2o;
    }

    #[test]
    #[ignore = "remove ignore after implementing H2O::hydrogen and H2O::oxygen"]
    fn case_single_molecule() {
        let out = run_case("HHO");
        assert_is_valid_h2o_output(&out);
    }

    #[test]
    #[ignore = "remove ignore after implementing H2O::hydrogen and H2O::oxygen"]
    fn case_multiple_molecules() {
        let out = run_case("OOHHHH");
        assert_is_valid_h2o_output(&out);
    }
}
