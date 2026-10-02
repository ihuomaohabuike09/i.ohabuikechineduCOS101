use std::io;
fn main() {
    let mut experience = String::new();
    let mut age = String::new();

    println!("Are you experienced? (yes/no)");
    io::stdin().read_line(&mut experience).expect("Failed to read line");

    println!("Enter your age:");
    io::stdin().read_line(&mut age).expect("Failed to read line");

    let age: u32 = age.trim().parse::<u32>().unwrap();

    if experience.trim().to_lowercase() == "yes" {
        if age >= 40 {
            println!("Annual incentive = #1,560,000");
        } else if age >= 30 {
            println!("Annual incentive = #1,480,00");
        } else {
            println!("Annual incentive = #1,300,000");
        }
    } else {
        println!("Annual incentive = #100,000");
    }
}