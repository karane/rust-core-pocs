// === Regular functions ===
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// A function with an explicit return statement
fn multiply(a: i32, b: i32) -> i32 {
    return a * b;
}

// Function returning nothing (unit type)
fn print_message() {
    println!("This function returns nothing (unit).");
}

// Using if as an expression
fn absolute_value(x: i32) -> i32 {
    if x >= 0 { x } else { -x }
}

// Nested function example
fn outer_function(x: i32) -> i32 {
    fn inner_function(y: i32) -> i32 {
        y * y
    }
    inner_function(x) + 1
}

fn main() {
    println!("=== Functions, Expressions, and Closures Demo ===");

    // --- Regular functions ---
    let sum = add(5, 3);
    println!("add(5, 3) = {}", sum);

    let product = multiply(4, 6);
    println!("multiply(4, 6) = {}", product);

    print_message();

    let abs1 = absolute_value(-10);
    let abs2 = absolute_value(7);
    println!("absolute_value(-10) = {}", abs1);
    println!("absolute_value(7) = {}", abs2);

    let result = outer_function(4);
    println!("outer_function(4) = {}", result);

    // --- Expression block example ---
    let x = {
        let a = 2;
        let b = 3;
        a * b + 1
    };
    println!("Result of block expression = {}", x);

    // --- Closures (anonymous functions) ---
    // Basic closure syntax: |args| expression
    let square = |n: i32| n * n;
    println!("square(5) = {}", square(5));

    // Closure with multiple parameters
    let add_numbers = |a: i32, b: i32| -> i32 { a + b };
    println!("add_numbers(2, 3) = {}", add_numbers(2, 3));

    // Closures can capture variables from the outer scope
    let factor = 10;
    let multiply_by_factor = |x: i32| x * factor;
    println!("multiply_by_factor(4) = {}", multiply_by_factor(4));

    // A mutable closure that modifies its captured variable
    let mut counter = 0;
    let mut increment = || {
        counter += 1;
        println!("Counter is now {}", counter);
    };
    increment();
    increment();

    // Closure as a parameter
    fn apply_twice<F>(f: F, x: i32) -> i32
    where
        F: Fn(i32) -> i32,
    {
        f(f(x))
    }

    let double = |n: i32| n * 2;
    let result = apply_twice(double, 3);
    println!("apply_twice(double, 3) = {}", result);
}
