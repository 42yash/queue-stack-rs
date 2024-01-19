// main.rs

use termion::{clear, cursor};

mod queue;
mod stack;

use queue::Queue;
use stack::Stack;
use std::io::{self, Write};

fn main() {
    let initial_value = read_number();
    let mut queue = Queue::new(initial_value);
    let mut stack = Stack::new(initial_value);

    loop {
        match choose_data_structure() {
            1 => handle_queue(&mut queue),
            2 => handle_stack(&mut stack),
            3 => break,
            _ => println!("Invalid choice"),
        }
    }
}

fn choose_data_structure() -> i32 {
    print!("{}{}", clear::All, cursor::Goto(1, 1));
    println!("Choose a data structure:");
    println!("1. Queue");
    println!("2. Stack");
    println!("3. Exit");

    let choice = read_input();
    choice
}

fn handle_queue(queue: &mut Queue) {
    loop {
        print!("{}{}", clear::All, cursor::Goto(1, 1));
        println!("You chose Queue. What operation would you like to perform?");
        println!("1. Enqueue");
        println!("2. Dequeue");
        println!("3. Display");
        println!("4. Back");

        match read_input() {
            1 => {
                let num = read_number();
                queue.enqueue(num);
            }
            2 => match queue.dequeue() {
                Some(num) => println!("Dequeued: {}", num),
                None => println!("Queue is empty"),
            },
            3 => {
                queue.display();
            }
            4 => break,
            _ => println!("Invalid operation"),
        }
    }
}

fn handle_stack(stack: &mut Stack) {
    loop {
        print!("{}{}", clear::All, cursor::Goto(1, 1));
        println!("You chose Stack. What operation would you like to perform?");
        println!("1. Push");
        println!("2. Pop");
        println!("3. Display");
        println!("4. Back");

        match read_input() {
            1 => {
                let num = read_number();
                stack.push(num);
            }
            2 => match stack.pop() {
                Some(num) => println!("Popped: {}", num),
                None => println!("Stack is empty"),
            },
            3 => {
                stack.display();
            }
            4 => break,
            _ => println!("Invalid operation"),
        }
    }
}

fn read_input() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input");
            0
        }
    };
    input
}

fn read_number() -> i32 {
    println!("\n----------");
    print!("Enter a number: ");
    io::stdout().flush().unwrap();
    let num = read_input();
    num as i32
}
