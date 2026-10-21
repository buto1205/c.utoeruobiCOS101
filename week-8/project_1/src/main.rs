use std::io;

fn vadmin(){
    let vadmin = vec!["Intern", "Administrator", "Senior Administration", "Office Manager", "Director", "CEO"];
    println!("What is your current position, Enter: \n 0 for intern, \n 1 for Administrator, \n 2 for Senior Adminitrator, \n 3 for Office Manager, \n 4 for Director, \n 5 for CEO");
    let mut irole = String::new();
    io::stdin().read_line(&mut irole).expect("Failed to read input");
    let irole:usize = irole.trim().parse().expect("Invalid input");
    let role = vadmin[irole];
    println!("Your role is {}",role );
    let mut iexp = String::new();
    println!("How many years of experience do you have ?");
    io::stdin().read_line(&mut iexp).expect("Failed to read input");
    let iexp:u8 = iexp.trim().parse().expect("Invalid input");
    if irole == 0 || iexp>= 1 && iexp <=2 {
        println!("Your APS level is APS 1-2");
    }else if irole == 1 || iexp>= 3 && iexp <=5{
        println!("Your APS level is APS 3-5");
    }else if irole == 2 || iexp>= 5 && iexp <=8{
       println!("Your APS level is APS 5-8");
    }else if irole == 3 || iexp>= 8 && iexp <=10{
       println!("Your EL1 level is EL1 8-10");
    }else if irole == 4 || iexp>= 10 && iexp <=13{
        println!("Your EL1 level is EL1 10-13");
    }else if irole == 5 || iexp >= 13 {
        println!("SES");
    }
}

fn vacademic(){
    let vacademic = vec!["None", "Research Assistance", "PHD Candidate", "Post-Doc Researcher", "Senior Lecturer", "Dean"];
    println!("What is your current position, Enter: \n 0 for None, \n 1 for Research Assistance, \n 2 for PHD Candidate, \n 3 for Post-Doc Researcher, \n 4 for Senior Lecturer, \n 5 for Dean");
    let mut irole = String::new();
    io::stdin().read_line(&mut irole).expect("Failed to read input");
    let irole:usize = irole.trim().parse().expect("Invalid input");
    let role = vacademic[irole];
    println!("Your role is {}",role );
    let mut iexp = String::new();
    println!("How many years of experience do you have ?");
    io::stdin().read_line(&mut iexp).expect("Failed to read input");
    let iexp:u8 = iexp.trim().parse().expect("Invalid input");
    if irole == 0 || iexp>= 1 && iexp <=2 {
        println!("Your APS level is APS 1-2");
    }else if irole == 1 || iexp>= 3 && iexp <=5{
        println!("Your APS level is APS 3-5");
    }else if irole == 2 || iexp>= 5 && iexp <=8{
       println!("Your APS level is APS 5-8");
    }else if irole == 3 || iexp>= 8 && iexp <=10{
       println!("Your EL1 level is EL1 8-10");
    }else if irole == 4 || iexp>= 10 && iexp <=13{
        println!("Your EL1 level is EL1 10-13");
    }else if irole == 5 || iexp >= 13 {
        println!("SES");
    }
}

fn vlawyer(){
    let vlawyer = vec!["Paralegal", "Junior Associate", "Associate", "Senior Associate 1-2", "Senior Associate 3-4", "Partner"];
    println!("What is your current position, Enter: \n 0 for Paralegal, \n 1 for Junior Associate , \n 2 for Associate , \n 3 for Senior Associate 1-2, \n 4 for Senior Associate 3-4, \n 5 for Partner");
    let mut irole = String::new();
    io::stdin().read_line(&mut irole).expect("Failed to read input");
    let irole:usize = irole.trim().parse().expect("Invalid input");
    let role = vlawyer[irole];
    println!("Your role is {}",role );
    let mut iexp = String::new();
    println!("How many years of experience do you have ?");
    io::stdin().read_line(&mut iexp).expect("Failed to read input");
    let iexp:u8 = iexp.trim().parse().expect("Invalid input");
    if irole == 0 || iexp>= 1 && iexp <=2 {
        println!("Your APS level is APS 1-2");
    }else if irole == 1 || iexp>= 3 && iexp <=5{
        println!("Your APS level is APS 3-5");
    }else if irole == 2 || iexp>= 5 && iexp <=8{
       println!("Your APS level is APS 5-8");
    }else if irole == 3 || iexp>= 8 && iexp <=10{
       println!("Your EL1 level is EL1 8-10");
    }else if irole == 4 || iexp>= 10 && iexp <=13{
        println!("Your EL1 level is EL1 10-13");
    }else if irole == 5 || iexp >= 13 {
        println!("SES");
    }
}


fn vteacher(){
    let vteacher = vec!["Placement", "Classroom teacher", "Senior teacher", "Leading teacher", "Deputy principal", "Principal"];
    println!("What is your current position, Enter: \n 0 for placement, \n 1 for Classroom teacher, \n 2 for Senior teacher, \n 3 for Leading teacher, \n 4 for Deputy principal, \n 5 for Principal");
    let mut irole = String::new();
    io::stdin().read_line(&mut irole).expect("Failed to read input");
    let irole:usize = irole.trim().parse().expect("Invalid input");
    let role = vteacher[irole];
    println!("Your role is {}",role );
    let mut iexp = String::new();
    println!("How many years of experience do you have ?");
    io::stdin().read_line(&mut iexp).expect("Failed to read input");
    let iexp:u8 = iexp.trim().parse().expect("Invalid input");
    if irole == 0 || iexp>= 1 && iexp <=2 {
        println!("Your APS level is APS 1-2");
    }else if irole == 1 || iexp>= 3 && iexp <=5{
        println!("Your APS level is APS 3-5");
    }else if irole == 2 || iexp>= 5 && iexp <=8{
       println!("Your APS level is APS 5-8");
    }else if irole == 3 || iexp>= 8 && iexp <=10{
       println!("Your EL1 level is EL1 8-10");
    }else if irole == 4 || iexp>= 10 && iexp <=13{
        println!("Your EL1 level is EL1 10-13");
    }else if irole == 5 || iexp >= 13 {
        println!("SES");
    }
 }
 fn main() {
    let mainv = vec!["Office Admin", "Academics", "Lawyer", "Teacher"];
    println!("Select your sector, Enter \n0 for Office Admin, \n1 for Academics, \n2 for Lawyer, \n3 for Teacher");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let vmain:usize = input1.trim().parse().expect("Invalid input");
    let sector = mainv[vmain];
    match sector.trim() {
        "Office Admin" => {
            vadmin() }

        "Academics" => {
            vacademic()}

        "Lawyer" => {
            vlawyer()}

        "Teacher" => {
            vteacher()}

       _ => {
            println!("Invalid input");
        }
    }


    
}


