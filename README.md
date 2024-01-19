# Implementation of Stack and Queue Using Linked List

Yash Raj Maiya

## Introduction

Stacks and queues are fundamental data structures in computer science that organize data in a specific order. Stacks follow a Last-In-First-Out (LIFO) principle, meaning the last element added is the first one to be removed. Queues, on the other hand, follow a First-In-First-Out (FIFO) principle, meaning the first element added is the first one to be removed.

Linked lists play a crucial role in implementing these structures due to their dynamic nature, allowing for efficient insertions and deletions.

The objective of this assignment is to understand these data structures, their implementation using linked lists, and their applications in various computing scenarios.

## Theoretical Background

### Stack

A stack is a linear data structure that follows a particular order in which operations are performed. The order is LIFO as there is addition and removal of elements at one end, called the "top" of the stack. Stacks are used in many computing contexts, including but not limited to, memory management, runtime memory stack, backtracking, syntax parsing, and tree traversal.

### Queue

A queue is another linear data structure that follows a particular order in which operations are performed. The order is FIFO, meaning that the first element inserted will be the first one to be removed. Queues are used in various computing scenarios such as CPU scheduling, Disk Scheduling, and data streaming.

### Linked List

A linked list is a linear data structure where each element is a separate object. Each element (node) of a list consists of two items - the data and a reference to the next node. The last node has a reference to null, indicating the end of the chain.

Linked lists are advantageous for implementing stacks and queues as they allow for constant-time insertions and deletions. They are dynamic, so the size of the stack or queue can change according to the needs at runtime. They also do not require a chunk of contiguous memory, unlike arrays, making them more flexible for data storage.

## Implementation

### Stack

### Push Operation

The push operation adds a new element to the top of the stack.

Algorithm for `push` method:

1. Create a new stack node with the given value.
2. Set the `next` pointer of the new node to point to the current stack.
3. Clone the current stack and box it, assigning it to the `next` pointer.
4. Replace the current stack with the new node.

```rust
pub fn push(&mut self, value: i32) {
        let new_node = Stack {
            value,
            next: Address::Val(Box::new(self.clone())),
        };
        *self = new_node;
    }
```

### Pop Operation

The pop operation removes the top element from the stack and returns it.

Algorithm for `pop` method:

1. Check the `next` pointer of the current stack.
2. If the `next` pointer is not empty (`Address::Val`), proceed to pop.
3. Save the value of the current top element.
4. Set the current stack to the next node in the stack.
5. Return the saved value of the popped element.
6. If the `next` pointer is empty (`Address::Nil`), return `None` indicating the stack is empty.

```rust
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
```

### Queue Code Snippets

### Enqueue Operation

The enqueue operation adds a new element to the end of the queue.

Algorithm for `enqueue` method:

1. Start at the front of the queue.
2. Traverse the queue until you find the last node (where `next` is `Address::Nil`).
3. Create a new queue node with the given value.
4. Set the `next` pointer of the last node to the new node.

```rust
pub fn enqueue(&mut self, value: i32) {
        let mut current = self;
        while let Address::Val(ref mut next) = current.next {
            current = next;
        }
        current.next = Address::Val(Box::new(Queue::new(value)));
    }
```

### Dequeue Operation

The dequeue operation removes the front element from the queue and returns it.

Algorithm for `dequeue` method:

1. Check the `next` pointer of the current queue.
2. If the `next` pointer is not empty (`Address::Val`), proceed to dequeue.
3. Save the value of the current front element.
4. Set the current queue to the next node in the queue.
5. Return the saved value of the dequeued element.
6. If the `next` pointer is empty (`Address::Nil`), return `None` indicating the queue is empty.

```rust
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
```

### Common Code Snippets

### Display Operation

Algorithm for  `display` method:

1. Start with the current node as the top of the stack or the front of the queue.
2. Loop through each node in the stack or queue:
    - While the `next` pointer is not empty (`Address::Val`), print the current node's value.
    - Update the current node to the next node in the stack or queue.
3. After the loop, print the value of the last node.

```rust
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
```

## Code Outputs

![Untitled](Implementation%20of%20Stack%20and%20Queue%20Using%20Linked%20Lis%20173c2d4369734161869ce1b777ffd74e/Untitled.png)

### Stack Outputs

![Untitled](Implementation%20of%20Stack%20and%20Queue%20Using%20Linked%20Lis%20173c2d4369734161869ce1b777ffd74e/Untitled%201.png)

![Untitled](Implementation%20of%20Stack%20and%20Queue%20Using%20Linked%20Lis%20173c2d4369734161869ce1b777ffd74e/Untitled%202.png)

### Queue Outputs

![Untitled](Implementation%20of%20Stack%20and%20Queue%20Using%20Linked%20Lis%20173c2d4369734161869ce1b777ffd74e/Untitled%203.png)

![Untitled](Implementation%20of%20Stack%20and%20Queue%20Using%20Linked%20Lis%20173c2d4369734161869ce1b777ffd74e/Untitled%204.png)

## Conclusion

In conclusion, this report has successfully demonstrated the implementation of Stack and Queue data structures using Linked Lists. The dynamic nature of Linked Lists has proven to be highly effective in managing these data structures, providing efficient and flexible solutions for data storage and manipulation.

The use of Linked Lists for Stacks and Queues allows for constant-time insertions and deletions, a significant advantage over other data structures that require shifting elements or have a fixed size. Furthermore, the fact that Linked Lists do not require a contiguous block of memory makes them a more flexible and practical choice for a variety of computing scenarios.

The detailed explanation and code snippets provided in this report offer a clear understanding of the underlying principles and practical applications of these data structures. The visual outputs further illustrate the functionality of each operation, reinforcing the theoretical concepts with practical examples.

Moving forward, these implementations can be further optimized or adapted to suit specific requirements in different computing scenarios. The principles and methods outlined in this report provide a solid foundation for further exploration and innovation in the field of data structures and algorithms.