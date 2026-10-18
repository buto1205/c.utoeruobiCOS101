fn main() {
    let apply: Vec<(String,u16)> = Vec::new();

    println!("WELCOME TO EY GLOBAL LIMITED");

    println!("How many candidates do you want to interview ?");



    let mut cand = String::new();
    std::io::stdin().read_line(&mut cand).expect ("Failed to read input");
    let cand:u16 = cand.trim().parse().expect("Invalid input");


    for count in 0..cand {
        let mut name = String::new();

        println!("Please input candidates name {}",count+1 );
        std::io::stdin().read_line(&mut name).expect("Failed to read input");
        name = name.trim().to_string();

        let mut exp = String::new();
        println!("How many years of programming experience do you have?");
        std::io::stdin().read_line(&mut exp).expect("Failed to read input");
        exp = exp.trim().parse().expect("Invalid input");

        apply.push((name,exp));

        if let Some((name,exp)) = apply.iter().max_by_key(| (_, expect)|exp 2){
            println!("The most experienced worker is {} with {} years of work",name, exp );



    }


         
}
