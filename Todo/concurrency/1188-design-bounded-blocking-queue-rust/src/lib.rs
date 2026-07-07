use std::sync::{Arc, Mutex};
use std::thread;

pub struct BoundedBlockingQueue {
    capacity: i32,
}

impl BoundedBlockingQueue {
    pub fn new(capacity: i32) -> Self {
        Self { capacity }
    }

    pub fn enqueue(&self, element: i32) {
        let _ = element;
        let _ = self.capacity;
        todo!("implement blocking enqueue");
    }

    pub fn dequeue(&self) -> i32 {
        let _ = self.capacity;
        todo!("implement blocking dequeue");
    }

    pub fn size(&self) -> i32 {
        let _ = self.capacity;
        todo!("implement queue size accounting");
    }
}

fn run_fifo_case() -> Vec<i32> {
    let queue = BoundedBlockingQueue::new(2);
    queue.enqueue(1);
    queue.enqueue(2);
    let first = queue.dequeue();
    queue.enqueue(3);
    let second = queue.dequeue();
    let third = queue.dequeue();
    vec![first, second, third, queue.size()]
}

fn run_blocking_case() -> Vec<i32> {
    let queue = Arc::new(BoundedBlockingQueue::new(1));
    let consumed = Arc::new(Mutex::new(Vec::new()));

    let producer = {
        let queue = Arc::clone(&queue);
        thread::spawn(move || {
            queue.enqueue(10);
            queue.enqueue(20);
        })
    };

    let consumer = {
        let queue = Arc::clone(&queue);
        let consumed = Arc::clone(&consumed);
        thread::spawn(move || {
            consumed.lock().unwrap().push(queue.dequeue());
            consumed.lock().unwrap().push(queue.dequeue());
        })
    };

    producer.join().unwrap();
    consumer.join().unwrap();

    let mut result = Arc::try_unwrap(consumed).unwrap().into_inner().unwrap();
    result.push(queue.size());
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scaffold_builds() {
        let queue = BoundedBlockingQueue::new(2);
        let _ = queue;
    }

    #[test]
    #[ignore = "remove ignore after implementing BoundedBlockingQueue"]
    fn case_fifo() {
        assert_eq!(run_fifo_case(), vec![1, 2, 3, 0]);
    }

    #[test]
    #[ignore = "remove ignore after implementing BoundedBlockingQueue"]
    fn case_blocking() {
        assert_eq!(run_blocking_case(), vec![10, 20, 0]);
    }
}
