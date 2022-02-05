pub fn run() {
    // print to console
    println!("hello from the print.rs file");
    println!("a number goes there: {}, it's so easy", 3);

    // positional arguments
    println!("{name} likes to eat {food}", name = "Tracy", food = "burgers");

    // print for binary w/ placeholder
    println!("binary number {:b}", 102);

    // now a placeholder debug trait
    println!("{:?}", (10, "tapas", 2.4));

    // basic math

    /*
    10 + 10
    */

}