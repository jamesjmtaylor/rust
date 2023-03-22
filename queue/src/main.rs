extern crate queues;
use queues::*; //Vec-based, dequeue takes O(n), 1_000_000 enqueues/dequeues = 90 seconds
use std::time::{Instant};

fn main() {
    standard_queue();
}

fn standard_queue() {
    let iterations = 100_000;
    let mut q: Queue<isize> = queue![];
    let start = Instant::now();
    for i in 0..iterations {
        _ = q.add(i);
    }
    for _ in 0..iterations {
        _ = q.remove()
    }
    let duration = start.elapsed();
    println!("Time: {:?}", duration);
}

struct LLQueue<T> {
    let head: Option<LinkedList<T>>
    let tail: Option<LinkedList<T>>
}
//Must use a Box Linked List solution because rust prevents you from creating
//data structures of infinite size through recursion.  
// See `rustc --explain E0072`
// https://dhghomon.github.io/easy_rust/Chapter_53.html
// https://dhghomon.github.io/easy_rust/Chapter_45.html
struct LinkedList<T> {
    let data: T
    let next: Option<Box<LinkedList<T>>>
    let prev: Option<Box<LinkedList<T>>>
    init(_ data: T){
        self.data = data
    }
}

impl LLQueue<T> {  
    fn enqueue(_ key: T) {
        let newNode = LinkedList<T>(key)
        if let t = tail {
            t.next = newNode
            newNode.prev = t
            tail = newNode
        } else if let h = head {
            h.next = newNode
            newNode.prev = h
            tail = newNode
        } else {
            head = newNode
        }
    }
    
    fn dequeue(&self) -> T? {
        if self.head?.data == nil { return nil  }
        let ret = head?.data
        if let nextItem = self.head?.next {
            self.head = nextItem
        } else {
            self.head = None
        }
        return ret
    }
} 
