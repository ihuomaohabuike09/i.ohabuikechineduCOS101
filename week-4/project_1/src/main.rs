//Rust program to find quadratic roots of an equation

use std::io;
fn main() {
    let mut input_a = String::new();
    let mut input_b = String::new();
    let mut input_c = String::new();

    println!("Enter value for a:");
    io::stdin().read_line(&mut input_a).expect("Failed to read line");
    let a: f64 = input_a.trim().parse().expect("Please enter a valid number");

    println!("Enter value for b:");
    io::stdin().read_line(&mut input_b).expect("Failed to read line");
    let b: f64 = input_b.trim().parse().expect("Please enter a valid number");

    println!("Enter value for c");
    io::stdin().read_line(&mut input_c).expect("Failed to read line");
    let c: f64 = input_c.trim().parse().expect("Please enter a valid number");

    //Formula: d = b*b - 4.0*a*c
    let d: f64 = (b * b) - (4.0 * a * c);
    println!("Discriminant (d) = {}",d);

    if d > 0.0 {
        let root1 = (-b + d.sqrt()) / (2.0 * a);
        let root2 = (-b - d.sqrt()) / (2.0 * a);

        println!("Root 1 = {}", root1);
        println!("Root 2 = {}", root2);
        
    }
}