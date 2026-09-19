use std::io;


fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter your age");
    io::stdin().read_line(&mut input1).expect("Invalid string");
    let age:u8 = input1.trim().parse().expect("Invalid age");

    println!("What is your experience level");
    io::stdin().read_line(&mut input2).expect("Not are a valid string");
    let experienced:bool = true;
    if experienced {
        println!("You are experienced");
    } else {
        println!("You are not experienced");
    }
    if experienced && age >= 40 {
        let inc_exp:u32 = 1_560_000;
        println!("Your annual incentive is {}", inc_exp ); //inc_exp is incentive for experienced
    } else if experienced && age < 40 && age >= 30 {
        let inc_fexp:u32 = 1_480_000;
        println!("Your annual inceptive is {}", inc_fexp ); //inc_fexp is inceptive for fairly experienced
    } else if experienced && age < 28{
         let in_nsexp:u32 = 1_300_000;
         println!("Your annual inceptive is {}", in_nsexp ); //in_nsexp is inceptive for not so experienced
    } else {
        let inc_nexp:u32 = 100_000;
        println!("Your annual inceptive is {}", inc_nexp ); //inc_nexp is inceptive for not  experienced
    }
}
