pub fn run() {
    let mut numbers: Vec<i32> = vec![1,2,3,4,5];
    let _arr: Vec<i32> = vec![2,4,6,8,10];

    println!("{:?}", numbers);
    numbers.push(20);
    

    println!("{:?}", numbers);

    println!("the vector occupies {} bytes", std::mem::size_of_val(&numbers));

}