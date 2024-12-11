fn main() {
    let mut v = vec![1, 2, 3];
    //Avoid unsafe code in this scenario.
    v[0] = 10; 
    println!("The first element is: {}", v[0]);
}
