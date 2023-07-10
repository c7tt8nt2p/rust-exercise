use std::os::unix::raw::gid_t;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Fork {
    id: usize,
}

impl Fork {
    fn new(id: usize) -> Self {
        Self { id }
    }
}

struct Philosopher {
    name: String,
    left_fork: Arc<Mutex<Fork>>,
    right_fork: Arc<Mutex<Fork>>,
}

impl Philosopher {
    fn new(name: &str, left_fork: Arc<Mutex<Fork>>, right_fork: Arc<Mutex<Fork>>) -> Self {
        Self {
            name: name.to_owned(),
            left_fork,
            right_fork,
        }
    }

    // fn think(&self) {
    //     self.thoughts
    //         .send(format!("Eureka! {} has a new idea!", &self.name))
    //         .unwrap();
    // }

    fn eat(&self) {
        let left_fork_lock = self.left_fork.lock().unwrap();
        let right_fork_lock = self.right_fork.lock().unwrap();
        println!("{} is eating...", &self.name);
        thread::sleep(Duration::from_millis(100));
        println!("......{} is done", &self.name);
    }
}

static PHILOSOPHERS: &[&str] = &[
    "[p0]Socrates",
    "[p1]Plato",
    "[p2]Aristotle",
    "[p3]Thales",
    "[p4]Pythagoras",
];

fn main() {
    // Create forks
    // let forks: Vec<Arc<Mutex<Fork>>> = (0..5).map(|i| Arc::new(Mutex::new(Fork::new(i)))).collect();

    // Create philosophers
    let philosophers: Vec<Arc<Mutex<Philosopher>>> = (0..5)
        .map(|i| {
            Arc::new(Mutex::new(Philosopher::new(
                PHILOSOPHERS[i],
                Arc::new(Mutex::new(Fork::new(i))),
                Arc::new(Mutex::new(Fork::new((i + 1) % 5))),
            )))
        })
        .collect();
    // Make each of them think and eat 100 times

    let mut handler = Vec::new();
    for p in philosophers {
        for _ in 0..10 {
            let p_clone = p.clone();
            let handle = thread::spawn(move || {
                p_clone.lock().unwrap().eat();
            });
            handler.push(handle);
        }
    }
    handler.into_iter().for_each(|e| e.join().unwrap());
    // Output their thoughts
}
