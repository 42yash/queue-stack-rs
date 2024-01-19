//stack.rs

use std::io::{self, Write};

#[derive(Clone)]
enum Address {
    Val(Box<Stack>),
    Nil,
}

#[derive(Clone)]
pub struct Stack {
    value: i32,
    next: Address,
}

impl Stack {
    pub fn new(value: i32) -> Self {
        Stack {
            value,
            next: Address::Nil,
        }
    }

    pub fn push(&mut self, value: i32) {
        let new_node = Stack {
            value,
            next: Address::Val(Box::new(self.clone())),
        };
        *self = new_node;
    }

    pub fn pop(&mut self) -> Option<i32> {
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
