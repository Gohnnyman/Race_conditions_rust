use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

const N: usize = 10;

struct Philosopher {
    index: usize,
}

impl Philosopher {
    fn new(index: usize) -> Philosopher {
        Philosopher { index }
    }

    fn left(&self) -> usize {
        return (self.index + N - 1) % N;
    }

    fn right(&self) -> usize {
        return (self.index + 1) % N;
    }

    fn eat(&self, table: &Table) {
        println!("{} is hungry.", self.index);

        let _left = table.forks[self.left()].lock().unwrap();
        let _right = table.forks[self.right()].lock().unwrap();

        println!("{} is eating.", self.index);

        let rand_ms = rand::random::<u64>() % 5000;
        thread::sleep(Duration::from_millis(rand_ms));
    }

    fn think(&self) {
        println!("{} is thinking.", self.index);

        let rand_ms = rand::random::<u64>() % 5000;
        thread::sleep(Duration::from_millis(rand_ms));
    }
}

#[derive(Default)]
struct Table {
    forks: Vec<Mutex<()>>,
}

impl Table {
    fn new(size: usize) -> Table {
        Table {
            forks: (0..size).into_iter().map(|_| Mutex::new(())).collect(),
        }
    }
}

fn main() {
    let philosophers = (0..N)
        .into_iter()
        .map(|i| Philosopher::new(i))
        .collect::<Vec<_>>();

    let table = Arc::new(Table::new(N));

    let handles: Vec<_> = philosophers
        .into_iter()
        .map(|p| {
            let table = table.clone();

            thread::spawn(move || loop {
                p.think();
                p.eat(&table);
            })
        })
        .collect();

    for h in handles {
        h.join().unwrap();
    }
}
