//queue.rs

use std::io::{self, Write};

#[derive(Clone)]
enum Address {
    Val(Box<Queue>),
    Nil,
}

#[derive(Clone)]
pub struct Queue {
    value: i32,
    next: Address,
}

impl Queue {
    pub fn new(value: i32) -> Self {
        Queue {
            value,
            next: Address::Nil,
        }
    }

    pub fn enqueue(&mut self, value: i32) {
        let mut current = self;
        while let Address::Val(ref mut next) = current.next {
            current = next;
        }
        current.next = Address::Val(Box::new(Queue::new(value)));
    }

    pub fn dequeue(&mut self) -> Option<i32> {
        match self.next {
            Address::Val(ref mut next) => {
                let value = self.value;
                *self = (**next).clone();
                Some(value)
            }
            Address::Nil => None,
        }
    }

    pub fn display(&self) {
        let mut current = self;
        println!("+----------------+");
        while let Address::Val(ref next) = current.next {
            print!("{:} -> ", current.value);
            current = next;
        }
        println!("{:}", current.value);
        println!("+----------------+");
        let mut input = String::new();
        loop {
            print!("Press 1 to continue...");
            io::stdout().flush().unwrap(); // Make sure the prompt is printed immediately
            input.clear();
            io::stdin().read_line(&mut input).unwrap(); // Wait for the user to press Enter
            if input.trim() == "1" {
                break;
            }
        }
    }
}
