use std::io::Write;

fn main() {

    let header = "LARGER,STOUT,NON-ALCHOLIC\n";
    let row1 = "33 Export,Legend,Maltina\n";
    let row2 = "Desperados,Turbo King,Amstel Malta\n";
    let row3 = "Goldberg,Willams,Malta Gold\n";
    let row4 = "Gulder,,Fayrouz\n";
    let row5 = "Heineken,,,\n";
    let row6 = "star,,,\n";


    let mut file = std::fs::File::create("data.csv").expect("failed to create");
    file.write_all(header.as_bytes()).expect("input failed");
    file.write_all(row1.as_bytes()).expect("input failed");
    file.write_all(row2.as_bytes()).expect("input failed");
    file.write_all(row3.as_bytes()).expect("input failed");
    file.write_all(row4.as_bytes()).expect("input failed");
    file.write_all(row5.as_bytes()).expect("input failed");
    file.write_all(row6.as_bytes()).expect("input failed");

}

