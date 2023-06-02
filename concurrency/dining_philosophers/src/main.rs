use std::sync::{mpsc, Arc, Mutex};
use std::thread::{self, Thread};
use std::time::Duration;
use crate::mpsc::Sender;

struct Fork;

struct Philosopher {
    name: String,
    left_fork: Arc<Mutex<Fork>>,
    right_fork: Arc<Mutex<Fork>>,
    thoughts: Sender<String>,
}

impl Philosopher {
    fn think(&self) {
        self.thoughts
            .send(format!("Eureka! {} has a new idea!", &self.name))
            .unwrap();
    }

    fn eat(&self) {
        // Pick up forks...
        println!("{} is eating...", &self.name);
        thread::sleep(Duration::from_millis(10));
    }
}

static PHILOSOPHERS: &[&str] =
    &["Socrates", "Plato", "Aristotle", "Thales", "Pythagoras"];

fn main() {
    let (tx, rx) = mpsc::channel();

    // Create forks
    let mut forks = Vec::new();
    for _ in PHILOSOPHERS {
        forks.push(Arc::new(Mutex::new(Fork)));
    }

    // Create philosophers
    let mut phils = Vec::new();
    for (i, p) in PHILOSOPHERS.iter().enumerate() {
        phils.push(
            Philosopher { 
                name: p.to_string(),
                left_fork: forks[i].clone(),
                right_fork: forks[(i+1) % PHILOSOPHERS.len()].clone(),
                thoughts: tx.clone(),
            }
        );
    }

    // Make them think and eat
    for p in phils {
        thread::spawn(move || {
            loop {
                let ll = p.left_fork.try_lock();
                if ll.is_ok() {
                    let rl = p.right_fork.try_lock();
                    if rl.is_ok() {
                        p.eat();
                    }
                    else {
                        drop(ll);
                        drop(rl);
                        p.think();
                    }
                }
                else {
                    drop(ll);
                    p.think();
                }
            }
        });
    }
    drop(tx);
    for msg in rx.iter() {
        println!("{msg}");
    }

    // Output their thoughts
}   