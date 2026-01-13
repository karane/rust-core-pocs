// 1. Defining a simple trait
trait Greet {
    fn greet(&self) -> String;
}

struct Person {
    name: String,
}

impl Greet for Person {
    fn greet(&self) -> String {
        format!("Hello, my name is {}", self.name)
    }
}


// 2. Trait with multiple implementations
trait Area {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

struct Rectangle {
    w: f64,
    h: f64,
}

impl Area for Circle {
    fn area(&self) -> f64 {
        3.1415 * self.radius * self.radius
    }
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.w * self.h
    }
}


// 3. Trait with default methods
trait Describe {
    fn describe(&self) -> String {
        "No description available".into()
    }
}

struct Car {
    model: String,
}

struct Something;

impl Describe for Car {
    fn describe(&self) -> String {
        format!("This is a car model '{}'", self.model)
    }
}

impl Describe for Something {} // uses default method


// 4. Trait bounds (generics + trait)
trait Printable {
    fn print(&self);
}

impl Printable for i32 {
    fn print(&self) {
        println!("i32 value = {}", self);
    }
}

impl Printable for String {
    fn print(&self) {
        println!("String: {}", self);
    }
}

fn show<T: Printable>(item: T) {
    item.print();
}


// 5. Trait objects (dyn Trait)
trait Animal {
    fn speak(&self) -> String;
}

struct Dog;
struct Cat;

impl Animal for Dog {
    fn speak(&self) -> String {
        "woof".into()
    }
}

impl Animal for Cat {
    fn speak(&self) -> String {
        "meow".into()
    }
}

fn random_animal(n: u32) -> Box<dyn Animal> {
    if n % 2 == 0 {
        Box::new(Dog)
    } else {
        Box::new(Cat)
    }
}


// 6. Static vs dynamic dispatch
fn speak_static<T: Animal>(a: T) {
    println!("Static dispatch: {}", a.speak());
}

fn speak_dynamic(a: &dyn Animal) {
    println!("Dynamic dispatch: {}", a.speak());
}


// 7. Trait inheritance
trait Base {
    fn base(&self);
}

trait Extended: Base {
    fn extended(&self) {
        println!("Default extended() method");
    }
}

struct Number(i32);

impl Base for Number {
    fn base(&self) {
        println!("Base number = {}", self.0);
    }
}

impl Extended for Number {} // uses default method


fn main() {
    println!("=== 1. Simple Trait ===");
    let p = Person { name: "Alice".into() };
    println!("{}", p.greet());

    println!("\n=== 2. Multiple Implementations ===");
    let c = Circle { radius: 3.0 };
    let r = Rectangle { w: 4.0, h: 2.0 };
    println!("Circle area = {}", c.area());
    println!("Rectangle area = {}", r.area());

    println!("\n=== 3. Default Methods ===");
    let car = Car { model: "Tesla".into() };
    let x = Something;
    println!("{}", car.describe());
    println!("{}", x.describe());

    println!("\n=== 4. Trait Bounds ===");
    show(42);
    show("hello".to_string());

    println!("\n=== 5. Trait Objects ===");
    let a = random_animal(5);
    println!("Trait object says: {}", a.speak());

    println!("\n=== 6. Static vs Dynamic Dispatch ===");
    speak_static(Dog);
    let cat = Cat;
    speak_dynamic(&cat);

    println!("\n=== 7. Trait Inheritance ===");
    let n = Number(10);
    n.base();
    n.extended();
}
