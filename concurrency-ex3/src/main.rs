use std::sync::Arc;
use std::time::Duration;

use tokio::sync::Mutex;
use tokio::time::sleep;

struct Fork {
    id: usize,
}

impl Fork {
    fn new(id: usize) -> Self {
        Self { id }
    }
}

struct Philosopher {
    id: usize,
    name: String,
    left_fork: Arc<Mutex<Fork>>,
    right_fork: Arc<Mutex<Fork>>,
}

impl Philosopher {
    fn new(
        id: usize,
        name: &str,
        left_fork: Arc<Mutex<Fork>>,
        right_fork: Arc<Mutex<Fork>>,
    ) -> Self {
        Self {
            id,
            name: name.to_owned(),
            left_fork,
            right_fork,
        }
    }

    async fn eat(&self) {
        println!("{} is trying to eat...", self.name);
        let left_fork_lock = {
            if self.id == 0 {
                self.left_fork.lock().await
            } else {
                self.right_fork.lock().await
            }
        };
        // println!(
        //     "{} got left fork !!!!!!!!! id = {}",
        //     self.name, left_fork_lock.id
        // );
        let right_fork_lock = {
            if self.id == 0 {
                self.right_fork.lock().await
            } else {
                self.left_fork.lock().await
            }
        };
        println!(
            "{} got 2 forks !!!!!!!!! id = {} and {}",
            self.name, left_fork_lock.id, right_fork_lock.id
        );
        sleep(Duration::from_millis(10)).await;
        println!("......{} is done", &self.name);
        //drop(left_fork_lock);
        //drop(right_fork_lock);
    }
}

static PHILOSOPHERS: &[&str] = &[
    "[p0]Socrates",
    "[p1]Plato",
    "[p2]Aristotle",
    "[p3]Thales",
    "[p4]Pythagoras",
];

#[tokio::main]
async fn main() {
    let forks: Vec<Arc<Mutex<Fork>>> = (0..5).map(|i| Arc::new(Mutex::new(Fork::new(i)))).collect();

    let mut handler = Vec::new();
    for i in 0..5 {
        let left_i = i as usize;
        let right_i = (i - 1_i8).rem_euclid(5_i8) as usize;
        let phil = Philosopher::new(
            i as usize,
            PHILOSOPHERS[i as usize],
            Arc::clone(&forks[left_i]),
            Arc::clone(&forks[right_i]),
        );

        handler.push(tokio::spawn(async move {
            for ii in 0..100 {
                println!("i - ii: {} - {}", i, ii);
                phil.eat().await
            }
        }));
    }
    for handle in handler {
        handle.await.unwrap();
    }
    // Output their thoughts
}
