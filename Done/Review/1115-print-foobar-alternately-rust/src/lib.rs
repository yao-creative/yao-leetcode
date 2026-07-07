use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use std::thread;

pub struct FooBar {
    n: i32,
    flag: AtomicBool,
    counter: AtomicI32,
}

pub struct FooBar2 {
    n: i32,
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
    fn new(n: i32) -> Self {
        Self {
            // should print foo, else should print 
            n,
            flag: AtomicBool::new(true),
            counter: AtomicI32::new(0),

        }
    }
}

impl FooBar2 {
    pub fn new(n: i32) -> Self {
        Self { n }
    }
}

impl FooBarTrait for FooBar {

    fn foo<F>(&self, print_foo: F)
    where
        F: Fn() + Send + 'static,
    {
        // if true 
        // If the current value of the atomic was `true`, atomically:
        // - set the new value to `false`
        // - return `true`, indicating that the swap was successful
        // This is a way to atomically "try to acquire a lock" using an atomic
        // flag. If the flag was `true`, meaning the lock was available,
        // the lock is acquired and the function proceeds. If the flag was
        // `false`, meaning the lock was not available, the lock is not
        // acquired and the function does nothing.

        // Hint: if this CAS succeeds but the counter check fails, who flips `flag` back?
        // Question: can one thread "consume a turn" without printing anything?
        if self.flag.compare_exchange(true, false, Ordering::AcqRel, Ordering::Acquire).is_ok() && 
        self.counter.load(Ordering::Relaxed) < self.n { 
            print_foo();
        }
        // todo!("scaffold only: implement FooBar2::foo");
    }

   

    fn bar<F>(&self, print_bar: F)
    where
        F: Fn() + Send + 'static,
    {
        // Hint: with the current `counter` initialization, does this branch run even once?
        // Question: should `counter` represent completed "foobar" pairs, pending turns, or something else?
        if self.flag.compare_exchange(false, true, Ordering::AcqRel,Ordering::Acquire).is_ok() &&
           self.counter.load(Ordering::Relaxed) < self.n {
            print_bar();
            self.counter.fetch_add(1, Ordering::SeqCst);
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
        todo!("scaffold only: implement FooBar::foo");
    }

    fn bar<F>(&self, print_bar: F)
    where
        F: Fn() + Send + 'static,
    {
        let _ = &print_bar;
        let _ = self.n;
        todo!("scaffold only: implement FooBar::bar");
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
    #[ignore = "remove ignore after implementing synchronization logic"]
    fn behavior_contract_foobar() {
        let foobar = FooBar::new(1);
        foobar.foo(|| {});
    }

    #[test]
    #[ignore = "remove ignore after implementing synchronization logic"]
    fn behavior_contract_foobar2() {
        let foobar2 = FooBar2::new(1);
        foobar2.foo(|| {});
    }
}
