/*
Primitive Types--
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory)
Floats: f32, f64
Boolean (bool)
Characters (char)
Tuples
Arrays
*/

pub fn run() {

    // add explicit type
    let n: i128 = 123456789010;
    println!("{} is a large number", n*2);
    println!("Max i128: {}", std::i128::MAX);
    
    let mut is_active: bool = true;
    println!("the variable is {}", is_active);

    is_active = false;
    println!("now I changed it to {}", is_active);

}