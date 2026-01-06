fn main() {
    println!("=== 1. Basic Closure ===");

    // Closures are anonymous functions that can capture variables from their environment.
    // Syntax: |parameter| expression
    let add_one = |x: i32| x + 1;
    println!("add_one(5) = {}", add_one(5));

    println!("\n=== 2. Closure Capturing an Outer Variable ===");

    // This closure captures the variable 'factor' from the environment.
    let factor = 10;
    let multiply = |x: i32| x * factor; // 'factor' is borrowed immutably
    println!("multiply(3) = {}", multiply(3));

    println!("\n=== 3. Inline Closure in Iterator ===");

    // Closures are commonly used with iterator adapters such as map, filter, etc.
    let numbers = vec![1, 2, 3, 4];
    let doubled: Vec<i32> = numbers.iter().map(|n| n * 2).collect();
    println!("Original numbers: {:?}", numbers);
    println!("Doubled numbers: {:?}", doubled);

    println!("\n=== 4. Closure with Explicit Type Annotation ===");

    // Usually Rust can infer types, but you can specify them if needed.
    let subtract = |a: i32, b: i32| -> i32 { a - b };
    println!("subtract(10, 3) = {}", subtract(10, 3));

    println!("\n=== 5. Passing a Closure to a Function ===");

    // You can pass closures as arguments to functions.
    apply_to_number(5, |x| x * x);
    apply_to_number(8, |x| x + 100);

    println!("\n=== 6. Closure That Mutably Captures Variables ===");

    // If a closure modifies a captured variable, it must be mutable.
    let mut counter = 0;
    let mut increment = || {
        counter += 1;
        println!("Counter: {}", counter);
    };
    increment();
    increment();
    increment();

    println!("\n=== 7. Closure That Moves Ownership ===");

    // Using 'move' forces the closure to take ownership of captured variables.
    let data = vec![1, 2, 3];
    let consume = move || {
        println!("Consuming data: {:?}", data);
    };
    consume();
    // After 'move', data is no longer available outside the closure.
    // println!("{:?}", data); // Uncommenting this line will cause an error
}

// Function that accepts any closure implementing Fn(i32) -> i32
fn apply_to_number<F>(x: i32, func: F)
where
    F: Fn(i32) -> i32,
{
    let result = func(x);
    println!("Result after applying closure to {}: {}", x, result);
}
