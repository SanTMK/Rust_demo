pub fn run() {
    // variables are immutable by default
    let name = "chad";

    println!("{name} is my name");

    //mutate variables using mut
    let mut age = 34;

    println!("{} is my age", age);
    age = 35;
    println!("my new age is {}", age);
}