use std::io;


fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();


    println!("Enter a value for a");
    io::stdin().read_line(&mut input1).expect("Failed to recognize string");
    let a:f32 = input1.trim().parse().expect("invalid number");

    println!("Enter a value for b");
    io::stdin().read_line(&mut input2).expect("Failed to recognize string");
    let b:f32 = input2.trim().parse().expect("invalid number");

    println!("Enter a value for c");
    io::stdin().read_line(&mut input3).expect("Failed to recognize string");
    let c:f32 = input3.trim().parse().expect("invalid number");


    let d:f32 = (b * b) - (4.0 * a * c);
    if d > 0.00 {
        let root1:f32 = (-b + d.sqrt()) / (2.0 * a);
        let root2:f32 = (-b - d.sqrt()) / (2.0 * a);
        println!("Two distinct roots which are: {} and {}", root1, root2);
    } else if d == 0.00 {
        let root:f32 = -b / (2.0 * a);
        println!("One real root which is: {}", root );
    } else{
        println!("No real roots");
    }
}

