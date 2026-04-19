use std::fmt;

/// Generic stack data structure that works with any type
/// Implements LIFO (Last In, First Out) semantics
#[derive(Clone)]
struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    /// Creates a new empty stack
    fn new() -> Stack<T> {
        Stack { items: Vec::new() }
    }

    /// Returns true if the stack has no elements
    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Adds an item to the top of the stack
    fn push(&mut self, item: T) {
        self.items.push(item);
    }

    /// Returns the number of items in the stack
    fn len(&self) -> usize {
        self.items.len()
    }

    /// Removes and returns the top item, or None if empty
    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    /// Returns a reference to the top item without removing it
    fn peek(&self) -> Option<&T> {
        self.items.last()
    }
}

/// Display trait implementation for stacks with displayable elements
impl<T: fmt::Display> fmt::Display for Stack<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        for (i, item) in self.items.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", item)?;
        }
        write!(f, "]")
    }
}

/// Iterator trait implementation for stacks
/// Iterates in LIFO (Last In, First Out) order
impl<T> Iterator for Stack<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
    }
}

fn main() {
    println!("=== Lab 12: Generic Stack Implementation ===\n");

    // Demonstrate with integers
    println!("=== Integer Stack ===");
    let mut int_stack = Stack::new();
    int_stack.push(10);
    int_stack.push(20);
    int_stack.push(30);
    println!("Stack: {}", int_stack);
    println!("Popped: {:?}", int_stack.pop());
    println!("Peek: {:?}", int_stack.peek());
    println!();

    // Demonstrate with strings
    println!("=== String Stack ===");
    let mut string_stack = Stack::new();
    string_stack.push(String::from("Rust"));
    string_stack.push(String::from("is"));
    string_stack.push(String::from("awesome"));
    println!("Stack: {}", string_stack);

    // Demonstrate iterator
    println!("\nIterating over string stack (LIFO order):");
    let mut string_stack_items = Vec::new();
    for item in string_stack {
        string_stack_items.push(item);
    }
    for item in string_stack_items {
        println!("  {}", item);
    }

    // Demonstrate with floats
    println!("\n=== Float Stack ===");
    let mut float_stack = Stack::new();
    float_stack.push(15.5);
    float_stack.push(42.25);
    float_stack.push(8.75);
    println!("Stack: {}", float_stack);
    println!("Stack length: {}", float_stack.len());
    println!("Is empty? {}", float_stack.is_empty());
}

#[cfg(test)]
mod tests {
    use super::*;

    // Basic functionality tests
    #[test]
    fn test_new_stack_is_empty() {
        let stack: Stack<i32> = Stack::new();
        assert!(stack.is_empty());
        assert_eq!(stack.len(), 0);
    }

    #[test]
    fn test_push_increases_length() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        assert_eq!(stack.len(), 3);
        assert!(!stack.is_empty());
    }

    #[test]
    fn test_pop_returns_last_pushed() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_pop_empty_stack() {
        let mut stack: Stack<i32> = Stack::new();
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_peek_without_removing() {
        let mut stack = Stack::new();
        stack.push(42);
        assert_eq!(stack.peek(), Some(&42));
        assert_eq!(stack.len(), 1);
    }

    #[test]
    fn test_peek_empty_stack() {
        let stack: Stack<i32> = Stack::new();
        assert_eq!(stack.peek(), None);
    }

    #[test]
    fn test_with_strings() {
        let mut stack = Stack::new();
        stack.push(String::from("hello"));
        stack.push(String::from("world"));
        assert_eq!(stack.pop(), Some(String::from("world")));
    }

    #[test]
    fn test_push_pop_sequence() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        assert_eq!(stack.pop(), Some(2));
        stack.push(3);
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(1));
    }

    // Display trait tests
    #[test]
    fn test_display_format() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        assert_eq!(format!("{}", stack), "[1, 2, 3]");
    }

    #[test]
    fn test_display_empty() {
        let stack: Stack<i32> = Stack::new();
        assert_eq!(format!("{}", stack), "[]");
    }

    #[test]
    fn test_display_strings() {
        let mut stack = Stack::new();
        stack.push("hello");
        stack.push("world");
        assert_eq!(format!("{}", stack), "[hello, world]");
    }

    // Iterator trait tests
    #[test]
    fn test_iterator() {
        let mut stack: Stack<i32> = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);

        let mut iter = stack.into_iter();
        assert_eq!(iter.next(), Some(3)); // LIFO order!
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_for_loop() {
        let mut stack: Stack<i32> = Stack::new();
        stack.push(1);
        stack.push(2);

        let mut results: Vec<i32> = Vec::new();
        for item in stack {
            results.push(item);
        }
        assert_eq!(results, vec![2, 1]); // LIFO!
    }

    #[test]
    fn test_iterator_with_strings() {
        let mut stack: Stack<String> = Stack::new();
        stack.push(String::from("first"));
        stack.push(String::from("second"));

        let collected: Vec<String> = stack.into_iter().collect();
        assert_eq!(
            collected,
            vec![String::from("second"), String::from("first")]
        );
    }

    // Polymorphism test
    #[test]
    fn test_polymorphism() {
        // Same Stack code works with different types!
        let mut int_stack = Stack::new();
        int_stack.push(1);
        int_stack.push(2);
        assert_eq!(int_stack.pop(), Some(2));

        let mut string_stack = Stack::new();
        string_stack.push(String::from("hello"));
        string_stack.push(String::from("world"));
        assert_eq!(string_stack.pop(), Some(String::from("world")));

        let mut float_stack = Stack::new();
        float_stack.push(15.5);
        float_stack.push(42.25);
        assert_eq!(float_stack.pop(), Some(42.25));
    }

    // Additional edge case test
    #[test]
    fn test_single_element_stack() {
        let mut stack = Stack::new();
        stack.push(100);
        assert!(!stack.is_empty());
        assert_eq!(stack.len(), 1);
        assert_eq!(stack.peek(), Some(&100));
        assert_eq!(stack.pop(), Some(100));
        assert!(stack.is_empty());
    }
}
