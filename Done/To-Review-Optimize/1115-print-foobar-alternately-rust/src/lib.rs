use std::sync::atomic::{AtomicBool, Ordering};

pub struct FooBar {
    n: usize,
    flag: AtomicBool,
}

pub struct FooBar2 {
    n: usize,
}

pub trait FooBarTrait {
    fn foo<F>(&self, print_foo: F)
    where
        F: Fn() + Send + 'static;

    fn bar<F>(&self, print_bar: F)
    where
        F: Fn() + Send + 'static;
}


impl FooBar {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            // true = foo turn
            // false = bar turn
            flag: AtomicBool::new(true),
        }
    }
}


impl FooBar2 {
    pub fn new(n: usize) -> Self {
        Self { n }
    }
}


impl FooBarTrait for FooBar {

    fn foo<F>(&self, print_foo: F)
    where
        F: Fn() + Send + 'static,
    {
        for _ in 0..self.n {
            loop {
                if self.flag
                    .compare_exchange(
                        true,
                        false,
                        Ordering::AcqRel,
                        Ordering::Acquire,
                    )
                    .is_ok()
                {
                    print_foo();
                    break;
                }
                std::thread::yield_now();

            }
        }
    }


    fn bar<F>(&self, print_bar: F)
    where
        F: Fn() + Send + 'static,
    {
        for _ in 0..self.n {
            loop {
                if self.flag
                    .compare_exchange(
                        false,
                        true,
                        Ordering::AcqRel,
                        Ordering::Acquire,
                    )
                    .is_ok()
                {
                    print_bar();
                    break;
                }
                std::thread::yield_now();

            }
        }
    }
}


impl FooBarTrait for FooBar2 {

    fn foo<F>(&self, print_foo: F)
    where
        F: Fn() + Send + 'static,
    {
        let _ = &print_foo;
        let _ = self.n;

        todo!("implement FooBar2::foo");
    }


    fn bar<F>(&self, print_bar: F)
    where
        F: Fn() + Send + 'static,
    {
        let _ = &print_bar;
        let _ = self.n;

        todo!("implement FooBar2::bar");
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn assert_trait_compat<T: FooBarTrait>(_item: &T) {}


    #[test]
    fn scaffold_builds_for_both_types() {
        let foobar = FooBar::new(1);
        let foobar2 = FooBar2::new(1);

        assert_trait_compat(&foobar);
        assert_trait_compat(&foobar2);
    }


    #[test]
    fn behavior_contract_foobar() {
        let foobar = FooBar::new(1);

        foobar.foo(|| {});
        foobar.bar(|| {});
    }

    #[test]
    fn behavior_contract_foobar_n3() {
        use std::sync::Arc;
        use std::thread;

        let foobar = Arc::new(FooBar::new(3));

        let foo_obj = Arc::clone(&foobar);
        let bar_obj = Arc::clone(&foobar);

        let foo_thread = thread::spawn(move || {
            foo_obj.foo(|| {
                print!("foo");
            });
        });

        let bar_thread = thread::spawn(move || {
            bar_obj.bar(|| {
                print!("bar");
            });
        });

        foo_thread.join().unwrap();
        bar_thread.join().unwrap();
    }


    #[test]
    #[ignore = "remove ignore after implementing synchronization logic"]
    fn behavior_contract_foobar2() {
        let foobar2 = FooBar2::new(1);

        foobar2.foo(|| {});
        foobar2.bar(|| {});
    }

    #[test]
    #[ignore = "remove ignore after implementing synchronization logic"]
    fn behavior_contract_foobar2_n3() {
        let foobar2 = FooBar2::new(3);

        foobar2.foo(|| {});
        foobar2.bar(|| {});
    }

}