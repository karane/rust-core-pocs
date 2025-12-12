fn main() {
    println!("=== Tuples in Rust ===");

    // 1. Creating tuples
    let person: (&str, i32, f64) = ("Alice", 30, 5.7);
    println!("Person tuple: {:?}", person);

    // 2. Accessing tuple elements by index
    println!("Name: {}", person.0);
    println!("Age: {}", person.1);
    println!("Height: {}", person.2);

    // 3. Destructuring tuples into variables
    let (name, age, height) = person;
    println!("Destructured => Name: {}, Age: {}, Height: {}", name, age, height);

    // 4. Nested tuples
    let nested = ("Bob", (2025, "Engineer"));
    println!("Nested tuple: {:?}", nested);
    println!("Year: {}", (nested.1).0);
    println!("Job: {}", (nested.1).1);

    // 5. Tuples can be returned from functions
    let (sum, product, average) = calculate_stats(4, 7);
    println!("Sum: {}, Product: {}, Average: {}", sum, product, average);

    // 6. Unit tuple (empty tuple) represents "nothing"
    let unit: () = ();
    println!("Unit value: {:?}", unit);

    // 7. Tuples can be compared if their elements implement PartialEq
    let t1 = (1, "apple");
    let t2 = (1, "apple");
    let t3 = (2, "orange");
    println!("t1 == t2 ? {}", t1 == t2);
    println!("t1 == t3 ? {}", t1 == t3);

    // 8. Mutable tuple example
    let mut point = (3, 4);
    println!("Before: {:?}", point);
    point.0 = 10;
    point.1 = 20;
    println!("After:  {:?}", point);
}

// Function returning multiple values using a tuple
fn calculate_stats(a: i32, b: i32) -> (i32, i32, f64) {
    let sum = a + b;
    let product = a * b;
    let average = (sum as f64) / 2.0;
    (sum, product, average)
}
