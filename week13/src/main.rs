use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

// ===========================================
// Part 1: Iterators and Closures
// ===========================================

/// Analyzes text and returns word count, average word length, and longest word.
///
/// # Arguments
/// * `text` - The text to analyze
///
/// # Returns
/// Tuple of (word_count, average_length, longest_word)
fn analyze_text(text: &str) -> (usize, f64, String) {
    let words: Vec<&str> = text.split_whitespace().collect();

    if words.is_empty() {
        return (0, 0.0, String::new());
    }

    let word_count = words.len();

    // Calculate average word length using iterator methods
    let total_length: usize = words.iter().map(|word| word.len()).sum();

    let average_length = total_length as f64 / word_count as f64;

    // Find longest word
    let longest_word = words
        .iter()
        .max_by_key(|word| word.len())
        .unwrap_or(&"")
        .to_string();

    (word_count, average_length, longest_word)
}

/// Filters and transforms a list of numbers:
/// - Keep only even numbers
/// - Square each number
/// - Return sum of results
fn process_numbers(numbers: &[i32]) -> i32 {
    numbers
        .iter()
        .filter(|&&n| n % 2 == 0)
        .map(|&n| n * n)
        .sum()
}

/// Returns a closure that counts how many times it's been called.
fn make_counter() -> impl FnMut() -> i32 {
    let mut count = 0;
    move || {
        count += 1;
        count
    }
}

// ===========================================
// Part 2: Smart Pointers
// ===========================================

/// Binary tree data structure using Box<T>
#[derive(Debug, PartialEq)]
enum BinaryTree<T> {
    Empty,
    Node {
        value: T,
        left: Box<BinaryTree<T>>,
        right: Box<BinaryTree<T>>,
    },
}

impl<T> BinaryTree<T> {
    /// Creates a new empty tree
    #[allow(dead_code)]
    fn new() -> Self {
        BinaryTree::Empty
    }

    /// Creates a leaf node (node with no children)
    fn leaf(value: T) -> Self {
        BinaryTree::Node {
            value,
            left: Box::new(BinaryTree::Empty),
            right: Box::new(BinaryTree::Empty),
        }
    }

    /// Creates a node with left and right children
    fn node(value: T, left: BinaryTree<T>, right: BinaryTree<T>) -> Self {
        BinaryTree::Node {
            value,
            left: Box::new(left),
            right: Box::new(right),
        }
    }
}

/// Shared data structure for Rc demonstration
#[derive(Debug)]
struct SharedData {
    #[allow(dead_code)]
    value: i32,
}

/// Demonstrates shared ownership with Rc<T>
fn demonstrate_rc() {
    let data = Rc::new(SharedData { value: 42 });

    // Create additional owners
    let _owner1 = Rc::clone(&data);
    let _owner2 = Rc::clone(&data);

    // All owners share the same data
    println!("Reference count: {}", Rc::strong_count(&data));
}

/// Counter structure for RefCell demonstration
#[derive(Debug)]
struct Counter {
    value: i32,
}

impl Counter {
    fn new() -> Rc<RefCell<Counter>> {
        Rc::new(RefCell::new(Counter { value: 0 }))
    }

    fn increment(counter: &Rc<RefCell<Counter>>) {
        counter.borrow_mut().value += 1;
    }

    fn get_value(counter: &Rc<RefCell<Counter>>) -> i32 {
        counter.borrow().value
    }
}

/// Demonstrates multiple shared mutable references
fn demonstrate_refcell() {
    let counter = Counter::new();
    let counter_ref1 = Rc::clone(&counter);
    let counter_ref2 = Rc::clone(&counter);

    Counter::increment(&counter_ref1);
    Counter::increment(&counter_ref2);

    println!("Counter value: {}", Counter::get_value(&counter));
}

// ===========================================
// Part 3: Idiomatic Error Handling
// ===========================================

/// Divides two numbers, returning an error for division by zero.
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Division by zero"))
    } else {
        Ok(a / b)
    }
}

/// Custom error type for parsing
#[derive(Debug, Clone, PartialEq)]
enum ParseError {
    EmptyInput,
    InvalidNumber(String),
    OutOfRange(i32),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::EmptyInput => write!(f, "Input string is empty"),
            ParseError::InvalidNumber(s) => write!(f, "Invalid number: {}", s),
            ParseError::OutOfRange(n) => write!(f, "Number out of range: {}", n),
        }
    }
}

impl std::error::Error for ParseError {}

/// Parses a string to a positive number (1-100).
fn parse_positive_number(input: &str) -> Result<i32, ParseError> {
    if input.is_empty() {
        return Err(ParseError::EmptyInput);
    }

    let num: i32 = input
        .trim()
        .parse()
        .map_err(|_| ParseError::InvalidNumber(input.to_string()))?;

    if !(1..=100).contains(&num) {
        return Err(ParseError::OutOfRange(num));
    }

    Ok(num)
}

// ===========================================
// Part 4: Integrative Exercise
// ===========================================

/// Configuration for file processing
#[derive(Debug, Clone)]
struct Config {
    min_length: usize,
    max_length: usize,
}

/// Error type for file processing
#[derive(Debug)]
enum ProcessError {
    LineTooShort(String),
    LineTooLong(String),
}

impl fmt::Display for ProcessError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ProcessError::LineTooShort(line) => {
                write!(f, "Line too short: {}", line)
            }
            ProcessError::LineTooLong(line) => {
                write!(f, "Line too long: {}", line)
            }
        }
    }
}

impl std::error::Error for ProcessError {}

/// Processes text lines with shared configuration.
fn process_lines(
    lines: &[String],
    config: Rc<RefCell<Config>>,
) -> Result<Vec<String>, ProcessError> {
    lines
        .iter()
        .map(|line| {
            let cfg = config.borrow();
            let len = line.len();

            if len < cfg.min_length {
                Err(ProcessError::LineTooShort(line.clone()))
            } else if len > cfg.max_length {
                Err(ProcessError::LineTooLong(line.clone()))
            } else {
                Ok(line.to_uppercase())
            }
        })
        .collect()
}

// ===========================================
// Main
// ===========================================

fn main() {
    println!("=== Lab 13: Idiomatic Rust ===\n");

    // Part 1: Iterators and Closures
    println!("Part 1: Iterators and Closures");
    let text = "The quick brown fox jumps";
    let (count, avg_len, longest) = analyze_text(text);
    println!("  Text: {}", text);
    println!("  Word count: {}", count);
    println!("  Average length: {:.2}", avg_len);
    println!("  Longest word: {}\n", longest);

    let nums = vec![1, 2, 3, 4, 5, 6];
    println!("  Numbers: {:?}", nums);
    println!("  Sum of squares of evens: {}\n", process_numbers(&nums));

    let mut counter = make_counter();
    println!("  Counter: {} {} {}\n", counter(), counter(), counter());

    // Part 2: Smart Pointers
    println!("Part 2: Smart Pointers");
    let tree = BinaryTree::node(5, BinaryTree::leaf(3), BinaryTree::leaf(7));
    println!("  Tree: {:?}\n", tree);
    demonstrate_rc();
    println!();
    demonstrate_refcell();
    println!();

    // Part 3: Error Handling
    println!("Part 3: Error Handling");
    match divide(10.0, 2.0) {
        Ok(result) => println!("  10.0 / 2.0 = {}", result),
        Err(e) => println!("  Error: {}", e),
    }

    match divide(10.0, 0.0) {
        Ok(result) => println!("  10.0 / 0.0 = {}", result),
        Err(e) => println!("  Error: {}", e),
    }

    match parse_positive_number("42") {
        Ok(n) => println!("  Parsed: {}", n),
        Err(e) => println!("  Error: {}", e),
    }
    println!();

    // Part 4: Integrative
    println!("Part 4: Integrative Exercise");
    let config = Rc::new(RefCell::new(Config {
        min_length: 3,
        max_length: 10,
    }));

    let lines = vec!["hello".to_string(), "world".to_string()];
    match process_lines(&lines, config) {
        Ok(processed) => println!("  Processed: {:?}", processed),
        Err(e) => println!("  Error: {}", e),
    }
}

// ===========================================
// Tests
// ===========================================

#[cfg(test)]
mod tests {
    use super::*;

    // Part 1: Iterators and Closures Tests
    #[test]
    fn test_analyze_text_basic() {
        let result = analyze_text("hello beautiful");
        assert_eq!(result.0, 2);
        assert!((result.1 - 7.0).abs() < 0.01);
        assert_eq!(result.2, "beautiful");
    }

    #[test]
    fn test_analyze_text_empty() {
        let result = analyze_text("");
        assert_eq!(result, (0, 0.0, String::new()));
    }

    #[test]
    fn test_analyze_text_single_word() {
        let result = analyze_text("hello");
        assert_eq!(result.0, 1);
        assert_eq!(result.1, 5.0);
        assert_eq!(result.2, "hello");
    }

    #[test]
    fn test_process_numbers_basic() {
        assert_eq!(process_numbers(&[1, 2, 3, 4, 5, 6]), 56);
    }

    #[test]
    fn test_process_numbers_no_evens() {
        assert_eq!(process_numbers(&[1, 3, 5]), 0);
    }

    #[test]
    fn test_process_numbers_empty() {
        assert_eq!(process_numbers(&[]), 0);
    }

    #[test]
    fn test_counter_closure() {
        let mut counter = make_counter();
        assert_eq!(counter(), 1);
        assert_eq!(counter(), 2);
        assert_eq!(counter(), 3);
    }

    // Part 2: Smart Pointers Tests
    #[test]
    fn test_binary_tree_creation() {
        let tree = BinaryTree::node(5, BinaryTree::leaf(3), BinaryTree::leaf(7));

        match tree {
            BinaryTree::Node { value, .. } => assert_eq!(value, 5),
            _ => panic!("Expected Node"),
        }
    }

    #[test]
    fn test_binary_tree_empty() {
        let tree: BinaryTree<i32> = BinaryTree::new();
        assert_eq!(tree, BinaryTree::Empty);
    }

    #[test]
    fn test_binary_tree_leaf() {
        let tree = BinaryTree::leaf(42);
        match tree {
            BinaryTree::Node { value, .. } => assert_eq!(value, 42),
            _ => panic!("Expected Node"),
        }
    }

    #[test]
    fn test_rc_reference_counting() {
        let data = Rc::new(SharedData { value: 42 });
        assert_eq!(Rc::strong_count(&data), 1);

        let _owner1 = Rc::clone(&data);
        assert_eq!(Rc::strong_count(&data), 2);

        let _owner2 = Rc::clone(&data);
        assert_eq!(Rc::strong_count(&data), 3);
    }

    #[test]
    fn test_refcell_mutation() {
        let counter = Counter::new();
        assert_eq!(Counter::get_value(&counter), 0);

        Counter::increment(&counter);
        assert_eq!(Counter::get_value(&counter), 1);

        Counter::increment(&counter);
        assert_eq!(Counter::get_value(&counter), 2);
    }

    #[test]
    fn test_refcell_shared_mutation() {
        let counter = Counter::new();
        let counter_ref1 = Rc::clone(&counter);
        let counter_ref2 = Rc::clone(&counter);

        Counter::increment(&counter_ref1);
        Counter::increment(&counter_ref2);

        assert_eq!(Counter::get_value(&counter), 2);
    }

    // Part 3: Error Handling Tests
    #[test]
    fn test_divide_success() {
        assert_eq!(divide(10.0, 2.0), Ok(5.0));
        assert_eq!(divide(9.0, 3.0), Ok(3.0));
    }

    #[test]
    fn test_divide_by_zero() {
        assert!(divide(10.0, 0.0).is_err());
    }

    #[test]
    fn test_parse_positive_number_valid() {
        assert_eq!(parse_positive_number("50"), Ok(50));
        assert_eq!(parse_positive_number("1"), Ok(1));
        assert_eq!(parse_positive_number("100"), Ok(100));
    }

    #[test]
    fn test_parse_positive_number_empty() {
        assert!(matches!(
            parse_positive_number(""),
            Err(ParseError::EmptyInput)
        ));
    }

    #[test]
    fn test_parse_positive_number_invalid() {
        assert!(matches!(
            parse_positive_number("abc"),
            Err(ParseError::InvalidNumber(_))
        ));
    }

    #[test]
    fn test_parse_positive_number_out_of_range() {
        assert!(matches!(
            parse_positive_number("150"),
            Err(ParseError::OutOfRange(150))
        ));
    }

    // Part 4: Integrative Tests
    #[test]
    fn test_process_lines_success() {
        let config = Rc::new(RefCell::new(Config {
            min_length: 3,
            max_length: 10,
        }));

        let lines = vec!["hello".to_string(), "world".to_string()];

        let result = process_lines(&lines, config);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec!["HELLO", "WORLD"]);
    }

    #[test]
    fn test_process_lines_too_short() {
        let config = Rc::new(RefCell::new(Config {
            min_length: 5,
            max_length: 10,
        }));

        let lines = vec!["hi".to_string()];
        let result = process_lines(&lines, config);
        assert!(result.is_err());
    }

    #[test]
    fn test_process_lines_too_long() {
        let config = Rc::new(RefCell::new(Config {
            min_length: 3,
            max_length: 5,
        }));

        let lines = vec!["toolongword".to_string()];
        let result = process_lines(&lines, config);
        assert!(result.is_err());
    }
}
