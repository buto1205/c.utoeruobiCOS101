use std::io;

 fn main() {
    
    println!(" WELCOME TO YOUR RESTAURANT ");
    println!("\n Please make your choice of meal.");
    println!(" \n RESTAURANT MENU  ");
    println!(" P - Pounded Yam / Edinkaiko Soup");
    println!(" F - Fried Rice & Chicken" );
    println!(" A - Amala & Ewedu Soup");
    println!(" E - Eba & Egusi Soup");
    println!(" W - White Rice & Stew");

    println!("Enter your food choice ( P , F , A , E , W ):");
    let mut food_choice = String::new();
    io::stdin().read_line(&mut food_choice).expect("Failed to read input");

    let mut price = 0.0;

    if food_choice == "P" {
         price = 3200.0;
    } else if food_choice == "F" {
         price = 3000.0;
    } else if food_choice == "A" {
         price = 2500.0;
    } else if food_choice == "E" {
          price = 2000.0;
    } else if food_choice == "W" {
         price = 2500.0;
    } else {
        println!("Failed to recognize the choice chosen");
        return;
    }

    println!("Please enter your desired quantity");
    let mut quantity = String::new();
    io::stdin().read_line(&mut quantity).expect("Failed to read input");
    let quantity:f64 = quantity.trim().parse().expect("Invalid Number");


       let mut total = price * quantity;
       println!("Subtotal: #{}", total );

     if total > 10000.0 {
        let discount = total * 0.05;
        total = total - discount;
        println!("CONGRATULATIONS, You earned a 5% discount of #{}", discount);
    }  else {
        println!(" Discount unavailable");
    }
      println!("\n Total Amount: #{}",total );




}



