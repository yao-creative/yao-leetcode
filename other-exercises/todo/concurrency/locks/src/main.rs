use std::sync::atomic::{AtomicBool, Ordering, AtomicUsize};


pub struct LockOne {
    flags: (AtomicBool, AtomicBool),
}

pub struct LockTwo{
    victim: AtomicUsize,
}

pub struct PetersonLock{
    flags: (AtomicBool, AtomicBool),
    victim: AtomicUsize,
}

pub trait CustomLockTrait {

    fn lock(&self, i: usize);

    fn unlock(&self, i: usize);
        
}

impl LockOne {
    pub fn new() -> Self {
        flags: (AtomicBool::new(false), AtomicBool::new(false)),
    }
}

impl LockTwo {
    pub fn new() -> Self {
        victim: AtomicUsize::new(0)
   
    }
}

impl PetersonLock {
    pub fn new() -> Self {
        flags: (AtomicBool::new(false), AtomicBool::new(false)),
        victim: AtomicUsize::new(0),
    }
}


// Mutex <=> | Threads crit section (k) | <= 1 for all k time stamp >= 0.
// mutex by structural induction on transition prossibilities.
// intent possibilities. 

// failure case if no one wants to unlock first, but atleast mutex (is safe)
impl CustomLockTrait for LockOne{ 
    fn lock(&self, i: usize){
        let other = 1 - i; // in the mod 2 space 1 gives xor

        // giving intent
        if i == 0 {
            self.flags.0.store(true, Ordering::SeqCst);
        } else {
            self.flags.1.store(true, Ordering::SeqCst);
        }
        
        // Spin while the other flag is true (i.e. the other thread intends to enter)
        while if other == 0 { self.flags.0.load(Ordering::SeqCst) } else { self.flags.1.load(Ordering::SeqCst) } {
            std::hint::spin_loop();
        }
    }

    fn unlock(&self, i: usize){
        // set flags to false.
        if i == 0 {
            self.flags.0.store(false, Ordering::SeqCst);
        } else {
            self.flags.1.store(false, Ordering::SeqCst);
        }
    }

}

// Failure mode when one person goes to lock and has to wait for someone else to lock to be able to start.
impl CustomLockTrait for LockTwo{
    fn lock(&self, i: usize) {
        let other = 1 - i;
        // Set victim to self
        self.victim.store(i, Ordering::SeqCst);

        // Spin while victim is self
        while self.victim.load(Ordering::SeqCst) == i {
            std::hint::spin_loop();
        }
    }

    fn unlock(&self, i: usize) {
        // Nothing to do for LockTwo, as lock is fully controlled by victim variable
        // Unlock is a no-op
    }

}

impl CustomLockTrait for PetersonLock{
    fn lock(&self, i: usize) {
        let other = 1 - i;
        // Set victim to self
        self.victim.store(i, Ordering::SeqCst);
        // giving intent
        if i == 0 {
            self.flags.0.store(true, Ordering::SeqCst);
        } else {
            self.flags.1.store(true, Ordering::SeqCst);
        }

        // Interest predicate and priority predicate.
        while (self.victim.load(Ordering::SeqCst) == i) && (if other == 0 { self.flags.0.load(Ordering::SeqCst) } else { self.flags.1.load(Ordering::SeqCst) }) {
            std::hint::spin_loop();
        }
    }

    fn unlock(&self, i: usize) {
        // Nothing to do for LockTwo, as lock is fully controlled by victim variable
        // Unlock is a no-op
    }
}