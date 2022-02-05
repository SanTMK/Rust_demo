// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own string data


pub fn run() {
    let prim_str = "Hello, I'm a primitive string";
    let strong_str = String::from("This is a String string");

    println!("one: {}", prim_str);
    println!("two: {}", strong_str);

    println!("the lenght of one is {} characters", prim_str.len());

    // Assertion testing

    assert_eq!(20, prim_str.len())

}