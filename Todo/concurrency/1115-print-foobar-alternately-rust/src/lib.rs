use std::sync::{Arc, Mutex};
use std::thread;

pub struct FooBar {
    n: i32,
}

impl FooBar {
    pub fn new(n: i32) -> Self {
        Self { n }
    }

    pub fn foo<F>(&self, print_foo: F)
    where
        F: Fn() + Send + 'static,
    {
        let _ = &print_foo;
        let _ = self.n;
        todo!("implement alternating coordination for foo");
    }

    pub fn bar<F>(&self, print_bar: F)
    where
        F: Fn() + Send + 'static,
    {
        let _ = &print_bar;
        let _ = self.n;
        todo!("implement alternating coordination for bar");
    }
}

fn run_case(n: i32) -> Vec<String> {
    let foobar = Arc::new(FooBar::new(n));
    let out = Arc::new(Mutex::new(Vec::with_capacity((n * 2) as usize)));

    let foo_runner = {
        let foobar = Arc::clone(&foobar);
        let out = Arc::clone(&out);
        thread::spawn(move || {
            foobar.foo(move || {
                out.lock().unwrap().push("foo".to_string());
            });
        })
    };

    let bar_runner = {
        let foobar = Arc::clone(&foobar);
        let out = Arc::clone(&out);
        thread::spawn(move || {
            foobar.bar(move || {
                out.lock().unwrap().push("bar".to_string());
            });
        })
    };

    foo_runner.join().unwrap();
    bar_runner.join().unwrap();

    Arc::try_unwrap(out).unwrap().into_inner().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scaffold_builds() {
        let foobar = FooBar::new(1);
        let _ = foobar;
    }

    #[test]
    #[ignore = "remove ignore after implementing FooBar::foo and FooBar::bar"]
    fn case_n1() {
        assert_eq!(run_case(1), vec!["foo", "bar"]);
    }

    #[test]
    #[ignore = "remove ignore after implementing FooBar::foo and FooBar::bar"]
    fn case_n2() {
        assert_eq!(run_case(2), vec!["foo", "bar", "foo", "bar"]);
    }

    #[test]
    #[ignore = "remove ignore after implementing FooBar::foo and FooBar::bar"]
    fn case_n3() {
        assert_eq!(run_case(3), vec!["foo", "bar", "foo", "bar", "foo", "bar"]);
    }
}
