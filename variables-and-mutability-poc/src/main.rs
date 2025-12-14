fn main() {
    println!("=== Variables, Constants, and Mutability ===");

    // 1. Immutable variable (default)
    let x = 5;
    println!("x = {}", x);
    // x = 6; // ERROR: cannot assign twice to immutable variable

    // 2. Mutable variable
    let mut y = 10;
    println!("y before = {}", y);
    y = 15; // OK: `y` is mutable
    println!("y after  = {}", y);

    // 3. Constant
    // Constants are *always* immutable and must have a type annotation.
    const MAX_POINTS: u32 = 100_000;
    println!("MAX_POINTS = {}", MAX_POINTS);

    // 4. Shadowing
    // You can reuse a variable name to create a *new* variable.
    let z = 3;
    println!("z = {}", z);

    let z = z + 1; // shadows previous `z`
    println!("z after shadowing = {}", z);

    // Shadowing allows changing type, unlike mutability.
    let z = "Now I'm a string!";
    println!("z now holds a string: {}", z);

    // 5. Scope demonstration
    let a = 42;
    {
        let a = a + 1; // shadows outer `a` only inside this block
        println!("inner a = {}", a);
    }
    println!("outer a = {}", a);

    println!("=== Done ===");
}
