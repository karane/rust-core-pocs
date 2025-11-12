fn main() {
    // ---------- ARRAYS ----------
    // Arrays have a fixed length known at compile time.
    let arr = [10, 20, 30, 40, 50];

    println!("Array: {:?}", arr);
    println!("Length: {}", arr.len());
    println!("First element: {}", arr[0]);
    println!("Last element: {}", arr[arr.len() - 1]);

    // You can iterate over an array by reference
    for val in arr.iter() {
        println!("Value: {}", val);
    }

    // ---------- SLICES ----------
    // Slices are *views* into arrays (no ownership or copying)
    let slice_full: &[i32] = &arr;           // borrow the whole array
    let slice_part: &[i32] = &arr[1..4];     // borrow a portion (indices 1 to 3)

    println!("\nFull slice: {:?}", slice_full);
    println!("Partial slice: {:?}", slice_part);

    // Modifying through mutable slices
    let mut arr_mut = [1, 2, 3, 4, 5];
    {
        let slice_mut = &mut arr_mut[1..4]; // mutable slice of middle elements
        for elem in slice_mut.iter_mut() {
            *elem *= 10; // multiply by 10
        }
    }
    println!("\nModified array after mutable slice: {:?}", arr_mut);

    // ---------- FUNCTION EXAMPLES ----------
    print_slice(&arr[2..]);     // pass a slice (borrowing part of the array)
    print_first_two(&arr);      // slices can also be created inside a function

    // ---------- SAFETY ----------
    // Out-of-bounds access causes a panic
    // println!("{}", arr[99]); // Uncomment to see panic at runtime
}

fn print_slice(slice: &[i32]) {
    println!("\nprint_slice() -> {:?}", slice);
}

fn print_first_two(slice: &[i32]) {
    let first_two = &slice[..2];
    println!("First two elements: {:?}", first_two);
}
