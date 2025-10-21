fn main() {
    println!("=== BASIC TYPES ===");

    // === Explicit typing ===
    let age: i32 = 30;
    let temperature: f64 = 22.5;
    let is_sunny: bool = true;
    let letter: char = 'R';
    let name: String = String::from("Rustacean");

    println!("Age (i32): {}", age);
    println!("Temperature (f64): {}", temperature);
    println!("Is it sunny? {}", is_sunny);
    println!("Favorite letter (char): {}", letter);
    println!("Name (String): {}", name);

    // === Type inference ===
    let year = 2025;            // inferred as i32
    let pi = 3.14159;           // inferred as f64
    let is_coding_fun = true;   // inferred as bool
    let initial = 'R';          // inferred as char
    let language = String::from("Rust"); // inferred as String

    println!("\n=== TYPE INFERENCE ===");
    println!("Year: {}", year);
    println!("Pi: {}", pi);
    println!("Is coding fun? {}", is_coding_fun);
    println!("Initial: {}", initial);
    println!("Language: {}", language);

    // === Type conversion (casting) ===
    let age_f64 = age as f64;
    let doubled_temp = temperature * 2.0;
    let temp_as_int = temperature as i32;

    println!("\n=== TYPE CONVERSION ===");
    println!("Age as f64: {}", age_f64);
    println!("Doubled temperature (f64): {}", doubled_temp);
    println!("Temperature truncated to i32: {}", temp_as_int);

    // === Combining values ===
    let summary = format!(
        "In {}, a {}-year-old {} developer feels {:.1}°C",
        year, age, language, temperature
    );
    println!("\n{}", summary);

    // === Example with casting during arithmetic ===
    let a: i32 = 5;
    let b: f64 = 2.5;
    let result = a as f64 + b;
    println!("\n5 (i32) + 2.5 (f64) = {}", result);

    // === Parsing Strings into basic types ===
    let number_str = "42";
    let float_str = "3.14";
    let bool_str = "true";
    let invalid_str = "oops";

    let number: i32 = number_str.parse().unwrap();
    let float: f64 = float_str.parse().unwrap();
    let truth: bool = bool_str.parse().unwrap();

    println!("\n=== PARSING STRINGS ===");
    println!("'{}' -> i32: {}", number_str, number);
    println!("'{}' -> f64: {}", float_str, float);
    println!("'{}' -> bool: {}", bool_str, truth);

    // Safe error handling
    let invalid_result: Result<i32, _> = invalid_str.parse();
    match invalid_result {
        Ok(value) => println!("Parsed successfully: {}", value),
        Err(e) => println!("Error parsing '{}': {}", invalid_str, e),
    }

    // === Final message ===
    let final_summary = format!(
        "\n{} ({}) is {} years old, loves {}, and says temperature {:.1}°C is fine.",
        name, letter, age, language, temperature
    );
    println!("{}", final_summary);
}
