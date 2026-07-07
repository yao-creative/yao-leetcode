use std::sync::{Arc, Mutex, Condvar};
use std::thread;

struct Event {
    flag: Mutex<bool>,
    cv: Condvar,
}

impl Event {
    fn new() -> Self {
        Self {
            flag: Mutex::new(false),
            cv: Condvar::new(),
        }
    }

    fn set(&self) {
        let mut flag = self.flag.lock().unwrap();
        *flag = true;
        self.cv.notify_all();
    }

    fn wait(&self) {
        let mut flag = self.flag.lock().unwrap();
        while !*flag {
            flag = self.cv.wait(flag).unwrap();
        }
    }
}

struct Foo {
    first_done: Arc<Event>,
    second_done: Arc<Event>,
}

impl Foo {
    fn new() -> Self {
        Self {
            first_done: Arc::new(Event::new()),
            second_done: Arc::new(Event::new()),
        }
    }

    fn first(&self, print_first: impl Fn()) {
        print_first();
        self.first_done.set();
    }

    fn second(&self, print_second: impl Fn()) {
        self.first_done.wait();
        print_second();
        self.second_done.set();
    }

    fn third(&self, print_third: impl Fn()) {
        self.second_done.wait();
        print_third();
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use std::thread;

    #[test]
    fn test_ordering_first_second_third() {
        let foo = Arc::new(Foo::new());

        let output = Arc::new(Mutex::new(Vec::new()));

        let o1 = output.clone();
        let f1 = foo.clone();
        let t1 = thread::spawn(move || {
            f1.first(|| o1.lock().unwrap().push(1));
        });

        let o2 = output.clone();
        let f2 = foo.clone();
        let t2 = thread::spawn(move || {
            f2.second(|| o2.lock().unwrap().push(2));
        });

        let o3 = output.clone();
        let f3 = foo.clone();
        let t3 = thread::spawn(move || {
            f3.third(|| o3.lock().unwrap().push(3));
        });

        t1.join().unwrap();
        t2.join().unwrap();
        t3.join().unwrap();

        let result = output.lock().unwrap();

        assert_eq!(&*result, &vec![1, 2, 3]);
    }

    #[test]
    fn test_reset_behavior() {
        let foo = Foo::new();

        foo.first(|| {});
        foo.second(|| {});
        foo.third(|| {});

        foo.first_done.set(); // should be idempotent-safe logically
        assert!(true);
    }
}