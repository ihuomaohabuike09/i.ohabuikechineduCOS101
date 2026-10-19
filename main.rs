use std::io::Write;
fn main() {
    // Create the file where the table data will be saved
    let mut file = std::fs::File::create("nigerian_breweries.txt").expect("create failed");

    //Write the table headers and contents into the file
    file.write_all(b"Lager\tStout\tNon-Alcoholic\n").expect("write failed");
    file.write_all(b"33 Export\tLegend\tAltina\n").expect("write failed");
    file.write_all(b"Desperados\tTurbo king\tAmstel Malta\n").expect("write failed");
    file.write_all(b"Goldberg\tWilliams\tMalta Gold\n").expect("write failed");
    file.write_all(b"Gulder\t\tFayrouz\n").expect("write failed");
    file.write_all(b"Heineken\t\t\n").expect("write failed");
    file.write_all(b"Star\t\t\n").expect("write failed");
    println!("nTable data successfully written to file!");
}
