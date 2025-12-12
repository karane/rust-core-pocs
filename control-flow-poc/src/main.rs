fn main() {
    println!("=== Control Flow Examples ===");

    // 1. if / else if / else
    let number = 7;
    if number < 5 {
        println!("Number is less than 5");
    } else if number == 5 {
        println!("Number is exactly 5");
    } else {
        println!("Number is greater than 5");
    }

    // 'if' is also an expression, so it can return a value
    let parity = if number % 2 == 0 { "even" } else { "odd" };
    println!("Number {} is {}", number, parity);

    // 2. match
    let day = "Sunday";
    match day {
        "Monday" => println!("Start of the week!"),
        "Friday" => println!("Weekend is near!"),
        "Saturday" | "Sunday" => println!("It's the weekend!"),
        _ => println!("Just another weekday."),
    }

    // match as an expression
    let score = 85;
    let grade = match score {
        90..=100 => "A",
        80..=89 => "B",
        70..=79 => "C",
        60..=69 => "D",
        _ => "F",
    };
    println!("Score: {}, Grade: {}", score, grade);

    // 3. loop (infinite until break)
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 5 {
            // You can break with a value
            break counter * 2;
        }
    };
    println!("Result from loop: {}", result);

    // 4. while loop
    let mut n = 3;
    while n > 0 {
        println!("Counting down: {}", n);
        n -= 1;
    }
    println!("Lift off!");

    // 5. for loop
    let numbers = [10, 20, 30, 40, 50];
    for num in numbers.iter() {
        println!("Array element: {}", num);
    }

    // for loop with range
    for i in 1..=5 {
        println!("Range value: {}", i);
    }

    // Nested control flow example
    println!("--- Nested control example ---");
    for i in 1..=5 {
        match i {
            1 => println!("First iteration"),
            2 => println!("Second iteration"),
            _ => println!("Something else: {}", i),
        }
    }

    println!("=== End of examples ===");
}
