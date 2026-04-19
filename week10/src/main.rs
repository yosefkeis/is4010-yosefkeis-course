// Lab 10: The Borrow Checker Game
// Fix each problem one at a time by uncommenting the function call in main()

fn main() {
    println!("Lab 10: Mastering Ownership and Borrowing");
    println!("Working through ownership and borrowing exercises!\n");

    // All problems solved:
    problem_1();
    problem_2();
    problem_3();
    problem_4();
    problem_5();
    problem_6();
    problem_7();

    println!("\n=== Implementation Exercises ===\n");

    // Test implementation exercises
    let result = to_uppercase_owned(String::from("hello"));
    println!("to_uppercase_owned: {}", result);

    let s = String::from("Rust");
    println!("string_length: {} chars", string_length(&s));

    let mut greeting = String::from("Hello");
    append_suffix(&mut greeting, ", World!");
    println!("append_suffix: {}", greeting);

    let result = concat_strings("Hello", " Rust");
    println!("concat_strings: {}", result);

    let sentence = "The quick brown fox jumps";
    println!("first_word: {}", first_word(sentence));
}

// ============================================================================
// PROBLEM 1: Value used after move
// ============================================================================
// Fixed: Changed calculate_length to borrow instead of taking ownership.
// Learning goal: Understand move semantics and when to use references
// ============================================================================
fn problem_1() {
    println!("Problem 1: Value used after move");
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("  The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &str) -> usize {
    s.len()
}

// ============================================================================
// PROBLEM 2: Immutable and mutable borrow conflict
// ============================================================================
// Fixed: Separated the immutable and mutable borrows using scopes.
// Learning goal: Understand the "one mutable OR many immutable" rule
// ============================================================================
fn problem_2() {
    println!("Problem 2: Mutable and immutable borrow conflict");
    let mut s = String::from("hello");
    let r1 = &s;
    println!("  Immutable borrow: {}", r1);
    // r1 scope ends here (Non-Lexical Lifetimes)

    let r2 = &mut s;
    r2.push_str(", world");
    println!("  After mutable borrow: {}", r2);
}

// ============================================================================
// PROBLEM 3: Mutating through immutable reference
// ============================================================================
// Fixed: Changed to use &mut for both the variable and parameter.
// Learning goal: Know when to use &T vs &mut T
// ============================================================================
fn problem_3() {
    println!("Problem 3: Mutating through immutable reference");
    let mut s = String::from("hello");
    add_to_string(&mut s);
    println!("  Result: {}", s);
}

fn add_to_string(s: &mut String) {
    s.push_str(", world");
}

// ============================================================================
// PROBLEM 4: Multiple mutable borrows
// ============================================================================
// Fixed: Used scopes to limit the lifetime of the first mutable borrow.
// Learning goal: Control borrow lifetimes with scopes
// ============================================================================
fn problem_4() {
    println!("Problem 4: Multiple mutable borrows");
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
        r1.push_str(" first");
    } // r1 scope ends here

    let r2 = &mut s;
    r2.push_str(" second");

    println!("  {}", r2);
}

// ============================================================================
// PROBLEM 5: Dangling reference
// ============================================================================
// Fixed: Return the owned String instead of a reference.
// Learning goal: Prevent use-after-free bugs
// ============================================================================
fn problem_5() {
    println!("Problem 5: Dangling reference");
    let r = create_string();
    println!("  Got: {}", r);
}

fn create_string() -> String {
    String::from("hello") // Return ownership (no &)
}

// ============================================================================
// PROBLEM 6: Ownership in loops
// ============================================================================
// Fixed: Changed to pass as reference instead of moving.
// Learning goal: Understand ownership with iteration
// ============================================================================
fn problem_6() {
    println!("Problem 6: Ownership in loops");
    let data = String::from("Rust");

    for i in 0..3 {
        print_with_number(&data, i);
    }
}

fn print_with_number(s: &String, n: i32) {
    println!("  {}: {}", n, s);
}

// ============================================================================
// PROBLEM 7: Lifetime extension challenge
// ============================================================================
// Fixed: Moved the String outside the inner scope so it lives long enough.
// Learning goal: Understand scope and lifetime relationships
// ============================================================================
fn problem_7() {
    println!("Problem 7: Lifetime extension");
    let s = String::from("outer scope");
    let result;
    {
        result = &s;
    }
    println!("  Result: {}", result);
}

// ============================================================================
// IMPLEMENTATION EXERCISES
// Write these functions from scratch with correct ownership/borrowing
// ============================================================================

/// Takes ownership of a String, converts it to uppercase, and returns it.
/// This demonstrates the "consume and return" pattern.
///
/// # Arguments
/// * `s` - String to convert (ownership transferred)
///
/// # Returns
/// * New String with all characters in uppercase
fn to_uppercase_owned(s: String) -> String {
    s.to_uppercase()
}

/// Borrows a String immutably and returns its length.
/// This demonstrates read-only borrowing.
///
/// # Arguments
/// * `s` - Reference to String to measure
///
/// # Returns
/// * Length of the string
fn string_length(s: &str) -> usize {
    s.len()
}

/// Borrows a String mutably and appends a suffix to it.
/// This demonstrates in-place modification through mutable borrowing.
///
/// # Arguments
/// * `s` - Mutable reference to String to modify
/// * `suffix` - String slice to append
fn append_suffix(s: &mut String, suffix: &str) {
    s.push_str(suffix);
}

/// Creates a new String by concatenating two borrowed strings.
/// This demonstrates creating owned data from borrowed data.
///
/// # Arguments
/// * `s1` - First string slice
/// * `s2` - Second string slice
///
/// # Returns
/// * New String containing s1 + s2
fn concat_strings(s1: &str, s2: &str) -> String {
    format!("{}{}", s1, s2)
}

/// Finds the first word in a string and returns it as a string slice.
/// This demonstrates returning borrowed data with implicit lifetimes.
///
/// # Arguments
/// * `s` - String slice to search
///
/// # Returns
/// * String slice containing the first word (up to first space),
///   or the entire string if no space is found
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    s
}

// ============================================================================
// TEST SUITE
// ============================================================================
// These tests verify your fixes are correct.
// Run with: cargo test
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_length_borrows() {
        let s = String::from("testing");
        let len = calculate_length(&s);
        assert_eq!(len, 7);
        // s should still be valid here
        assert_eq!(s, "testing");
    }

    #[test]
    fn test_add_to_string_mutates() {
        let mut s = String::from("hello");
        add_to_string(&mut s);
        assert_eq!(s, "hello, world");
    }

    #[test]
    fn test_create_string_returns_owned() {
        let result = create_string();
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_print_with_number_borrows() {
        let data = String::from("Rust");
        // Should work when passed as a reference
        for i in 0..3 {
            print_with_number(&data, i);
        }
        // data should still be valid
        assert_eq!(data, "Rust");
    }

    #[test]
    fn test_to_uppercase_owned() {
        let result = to_uppercase_owned(String::from("hello"));
        assert_eq!(result, "HELLO");
    }

    #[test]
    fn test_string_length() {
        let s = String::from("Rust");
        assert_eq!(string_length(&s), 4);
        // s should still be valid after borrowing
        assert_eq!(s, "Rust");
    }

    #[test]
    fn test_append_suffix() {
        let mut s = String::from("Hello");
        append_suffix(&mut s, ", World!");
        assert_eq!(s, "Hello, World!");
    }

    #[test]
    fn test_concat_strings() {
        let result = concat_strings("Hello", " Rust");
        assert_eq!(result, "Hello Rust");

        let result2 = concat_strings("abc", "def");
        assert_eq!(result2, "abcdef");
    }

    #[test]
    fn test_first_word() {
        assert_eq!(first_word("hello world"), "hello");
        assert_eq!(first_word("hello"), "hello");
        assert_eq!(first_word(""), "");
        assert_eq!(first_word("the quick brown fox"), "the");
    }
}
