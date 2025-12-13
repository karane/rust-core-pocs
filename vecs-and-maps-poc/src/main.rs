use std::collections::{VecDeque, HashMap, BTreeMap, HashSet, BTreeSet};

fn main() {
    println!("=== Vec and VecDeque ===");
    vec_and_vecdeque_example();

    println!("\n=== HashMap and BTreeMap ===");
    hashmap_and_btreemap_example();

    println!("\n=== HashSet and BTreeSet ===");
    hashset_and_btreeset_example();
}

fn vec_and_vecdeque_example() {
    // Vec: dynamic array (contiguous in memory)
    let mut v = vec![10, 20, 30];
    v.push(40);
    println!("Vec: {:?}", v);

    // Access elements
    println!("First: {}", v[0]);
    if let Some(last) = v.last() {
        println!("Last: {}", last);
    }

    // Iterate and sum
    let sum: i32 = v.iter().sum();
    println!("Sum of Vec elements: {}", sum);

    // VecDeque: double-ended queue (fast push/pop from both ends)
    let mut dq: VecDeque<i32> = VecDeque::new();
    dq.push_back(1);
    dq.push_back(2);
    dq.push_front(0);
    println!("VecDeque: {:?}", dq);

    dq.pop_front();
    dq.push_back(3);
    println!("After modifications: {:?}", dq);
}

fn hashmap_and_btreemap_example() {
    // HashMap: fast unordered key–value store
    let mut scores = HashMap::new();
    scores.insert("Alice", 50);
    scores.insert("Bob", 30);
    scores.insert("Charlie", 70);

    println!("HashMap (unordered):");
    for (name, score) in &scores {
        println!("{} => {}", name, score);
    }

    // Access and modify
    if let Some(score) = scores.get_mut("Alice") {
        *score += 10;
    }
    println!("After Alice’s update: {:?}", scores);

    // BTreeMap: ordered by key
    let mut grades = BTreeMap::new();
    grades.insert("Math", 9.5);
    grades.insert("Physics", 8.7);
    grades.insert("Biology", 7.8);

    println!("BTreeMap (ordered by key):");
    for (subject, grade) in &grades {
        println!("{} => {}", subject, grade);
    }
}

fn hashset_and_btreeset_example() {
    // HashSet: unique unordered collection
    let mut primes: HashSet<i32> = HashSet::new();
    primes.insert(2);
    primes.insert(3);
    primes.insert(5);
    primes.insert(3); // duplicate ignored

    println!("HashSet (unordered unique): {:?}", primes);
    println!("Contains 5? {}", primes.contains(&5));

    // BTreeSet: unique ordered collection
    let mut evens: BTreeSet<i32> = BTreeSet::new();
    evens.extend([2, 4, 6, 8, 10]);
    println!("BTreeSet (sorted): {:?}", evens);

    // Set operations
    let odds: BTreeSet<i32> = [1, 3, 5, 7, 9].into_iter().collect();
    let union: BTreeSet<_> = evens.union(&odds).cloned().collect();
    let intersection: BTreeSet<_> = evens.intersection(&odds).cloned().collect();

    println!("Union: {:?}", union);
    println!("Intersection: {:?}", intersection);
}
