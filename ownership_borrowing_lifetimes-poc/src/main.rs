// 1️1. Ownership and Move Semantics
fn ownership_example() {
    let s1 = String::from("Hello"); // s1 owns the string data on the heap
    let s2 = s1; // ownership of the heap data moves to s2

    // println!("{}", s1); // X Error: s1 is no longer valid!
    println!("s2 = {}", s2);

    let s3 = s2.clone(); // clone() copies the heap data
    println!("s3 (cloned) = {}", s3);
}

// 2 Borrowing (`&T` and `&mut T`)
fn borrowing_example() {
    let s = String::from("world");

    // Immutable borrow (read-only)
    print_length(&s);

    // Mutable borrow (exclusive write access)
    let mut msg = String::from("Hi");
    append_exclamation(&mut msg);
    println!("msg after mutation: {}", msg);
}

fn print_length(s: &String) {
    // We can read from `s`, but not modify it
    println!("Length = {}", s.len());
}

fn append_exclamation(s: &mut String) {
    s.push('!');
}

// 3 Lifetimes (`'a`) and Lifetime Elision
// Lifetimes ensure that references do not outlive the data they point to.

fn lifetimes_example() {
    let s1 = String::from("Rust");
    let s2 = String::from("Ownership");

    let result = longest(&s1, &s2);
    println!("Longest string: {}", result);
}

// Explicit lifetime annotation `'a`
// `'a` means: the returned reference is valid as long as BOTH inputs are valid.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 4 Lifetime Elision Rules
// Many functions don’t need explicit lifetimes because the compiler can infer them.
//
// Rule 1: Each parameter gets its own lifetime.
// Rule 2: If there’s exactly one input lifetime, it’s assigned to all output references.
// That’s why this function works without writing `'a`.
fn first_char(s: &str) -> &str {
    &s[0..1]
}

fn main() {
    println!("--- Ownership Example ---");
    ownership_example();

    println!("\n--- Borrowing Example ---");
    borrowing_example();

    println!("\n--- Lifetimes Example ---");
    lifetimes_example();

    println!("\n--- Lifetime Elision Example ---");
    let word = String::from("Hey!");
    println!("First char: {}", first_char(&word));
}
