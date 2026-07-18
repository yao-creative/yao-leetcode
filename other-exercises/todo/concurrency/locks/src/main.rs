use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

// SeqCst enforces comparability within the scope within the thread.

pub struct LockOne {
    flags: [AtomicBool; 2],
}

pub struct LockTwo{
    turn: AtomicUsize,
}

pub struct PetersonLock{
    flags: [AtomicBool; 2],
    victim: AtomicUsize,
}

pub struct LamportBakeryLock{
    choosing: Vec<AtomicBool>,
    number: Vec<AtomicUsize>,
    n: usize, // vector of the number 
}
pub trait CustomLockTrait {

    fn lock(&self, i: usize);

    fn unlock(&self, i: usize);
        
}

impl LockOne {
    pub fn new() -> Self {
        Self {
            flags: [AtomicBool::new(false), AtomicBool::new(false)],
        }
    }
}

impl LockTwo {
    pub fn new() -> Self {
        Self {
            turn: AtomicUsize::new(0),
        }
    }
}

impl PetersonLock {
    pub fn new() -> Self {
        Self {
            flags: [AtomicBool::new(false), AtomicBool::new(false)],
            victim: AtomicUsize::new(0),
        }
    }
}

impl LamportBakeryLock {
    pub fn new(n: usize) -> Self {
        Self {
            choosing: (0..n).map(|_| AtomicBool::new(false)).collect(),
            number: (0..n).map(|_| AtomicUsize::new(0)).collect(),
            n,
    
        }
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
        self.flags[i].store(true, Ordering::SeqCst);
        
        // Spin while the other flag is true (i.e. the other thread intends to enter)
        while self.flags[other].load(Ordering::SeqCst) {
            std::hint::spin_loop();
        }
    }

    fn unlock(&self, i: usize){
        // set flags to false.
        self.flags[i].store(false, Ordering::SeqCst);

    }

}

// Failure mode when one person goes to lock and has to wait for someone else to lock to be able to start.
impl CustomLockTrait for LockTwo{
    fn lock(&self, i: usize) {
        let other = 1 - i;
        while self.turn.load(Ordering::SeqCst) == other {
            std::hint::spin_loop();
        }
    }

    fn unlock(&self, i: usize) {
        let other = 1 - i;
        self.turn.store(other, Ordering::SeqCst);
    }

}

impl CustomLockTrait for PetersonLock{
    fn lock(&self, i: usize) {
        let other = 1 - i;
        // Set victim to self
        self.victim.store(i, Ordering::SeqCst);
        // giving intent
        self.flags[i].store(true, Ordering::SeqCst);


        // Interest predicate and priority predicate.
        while self.victim.load(Ordering::SeqCst) == i && self.flags[other].load(Ordering::SeqCst) {
            std::hint::spin_loop();
        }
    }

    fn unlock(&self, i: usize) {
        // Nothing to do for LockTwo, as lock is fully controlled by victim variable
        self.flags[i].store(false, Ordering::SeqCst);

    }
}


impl CustomLockTrait for LamportBakeryLock{
    fn lock(&self, i: usize) {

        // Doorway
        // choosing is like an atomic lock for checking number
        self.choosing[i].store(true, Ordering::SeqCst);

        let max = self
            .number
            .iter()
            .map(|x| x.load(Ordering::SeqCst))
            .max()
            .unwrap_or(0);
        self.number[i].store(max + 1, Ordering::SeqCst);
        self.choosing[i].store(false, Ordering::SeqCst);

        //waiting room
        for j in 0..self.n {
            if i == j {
                continue;
            }
            // Wait until thread j has picked its number (i.e., is not in doorway) 
            // this makes for no ambigious priority
            while self.choosing[j].load(Ordering::SeqCst) {
                std::hint::spin_loop();
            }

            // this resolves the arbtiration of who should go first
            let mi = self.number[i].load(Ordering::SeqCst);
            // Wait while j has a ticket and (number[j], j) < (number[i], i) // priority
            loop {
                let mj = self.number[j].load(Ordering::SeqCst);
            
                if mj == 0 {
                    break;
                }
            
                if (mj, j) >= (mi, i) {
                    break;
                }
            
                std::hint::spin_loop();
            }
        }
   
        
    }

    fn unlock(&self, i: usize) {
        self.number[i].store(0, Ordering::SeqCst);
    }
}

fn main() {}


#[cfg(test)]
mod tests{
    use super::*;
    use std::sync::atomic::AtomicUsize;
    use std::sync::{Arc, Barrier};
    use std::time::Duration;

    fn mutex_test_2<L: CustomLockTrait + Sync + Send + 'static>(lock: Arc<L>, iters: usize) {
        let barrier = Arc::new(Barrier::new(2));
        let in_cs = Arc::new(AtomicUsize::new(0));
        let total = Arc::new(AtomicUsize::new(0));

        let mut handles = Vec::new();
        for tid in 0..2 {
            let lock = Arc::clone(&lock);
            let barrier = Arc::clone(&barrier);
            let in_cs = Arc::clone(&in_cs);
            let total = Arc::clone(&total);
            handles.push(std::thread::spawn(move || {
                barrier.wait();
                for _ in 0..iters {
                    lock.lock(tid);
                    let prev = in_cs.fetch_add(1, Ordering::SeqCst);
                    assert_eq!(prev, 0, "more than one thread in critical section");
                    total.fetch_add(1, Ordering::SeqCst);
                    in_cs.fetch_sub(1, Ordering::SeqCst);
                    lock.unlock(tid);
                }
            }));
        }

        for h in handles {
            h.join().unwrap();
        }
        assert_eq!(total.load(Ordering::SeqCst), 2 * iters);
    }

    fn mutex_test_n<L: CustomLockTrait + Sync + Send + 'static>(
        lock: Arc<L>,
        n: usize,
        iters: usize,
    ) {
        let barrier = Arc::new(Barrier::new(n));
        let in_cs = Arc::new(AtomicUsize::new(0));
        let total = Arc::new(AtomicUsize::new(0));

        let mut handles = Vec::new();
        for tid in 0..n {
            let lock = Arc::clone(&lock);
            let barrier = Arc::clone(&barrier);
            let in_cs = Arc::clone(&in_cs);
            let total = Arc::clone(&total);
            handles.push(std::thread::spawn(move || {
                barrier.wait();
                for _ in 0..iters {
                    lock.lock(tid);
                    let prev = in_cs.fetch_add(1, Ordering::SeqCst);
                    assert_eq!(prev, 0, "more than one thread in critical section");
                    total.fetch_add(1, Ordering::SeqCst);
                    in_cs.fetch_sub(1, Ordering::SeqCst);
                    lock.unlock(tid);
                }
            }));
        }

        for h in handles {
            h.join().unwrap();
        }
        assert_eq!(total.load(Ordering::SeqCst), n * iters);
    }

    #[test]
    fn peterson_mutex() {
        let lock = Arc::new(PetersonLock::new());
        mutex_test_2(lock, 10_000);
    }

    #[test]
    fn bakery_mutex() {
        let n = 6;
        let lock = Arc::new(LamportBakeryLock::new(n));
        mutex_test_n(lock, n, 5_000);
    }

    #[test]
    fn lock_one_smoke_sequential() {
        let lock = LockOne::new();
        lock.lock(0);
        lock.unlock(0);
        lock.lock(1);
        lock.unlock(1);
    }

    #[test]
    fn lock_two_strict_alternation_blocks_reentry_until_other_runs() {
        let lock = Arc::new(LockTwo::new());
        lock.lock(0);
        lock.unlock(0); // hands turn to thread 1

        let (tx, rx) = std::sync::mpsc::channel::<()>();
        let lock_t0 = Arc::clone(&lock);
        let h0 = std::thread::spawn(move || {
            tx.send(()).unwrap(); // ready
            lock_t0.lock(0); // will block until thread 1 takes a turn
            lock_t0.unlock(0);
        });

        rx.recv().unwrap();
        std::thread::sleep(Duration::from_millis(20));

        // Unblock by letting thread 1 take its turn once.
        lock.lock(1);
        lock.unlock(1);

        h0.join().unwrap();
    }


}
