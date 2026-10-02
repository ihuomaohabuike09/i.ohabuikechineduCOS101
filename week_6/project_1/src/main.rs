use std::io;

fn main() {
    println!("========== RESTAURANT MENU ==========");
    println!("A - Amala & Ewedu Soup     ₦2500");
    println!("E - Eba & Egusi Soup       ₦2000");
    println!("W - White Rice & Stew      ₦2500");
    println!("======================================");

    println!("Enter food type (A, E, or W):");

    let mut food_type = String::new();
    io::stdin()
        .read_line(&mut food_type)
        .expect("Failed to read input");

    let food_type = food_type.trim().to_uppercase();

    let price: f64;

    if food_type == "A" {
        price = 2500.0;
    } else if food_type == "E" {
        price = 2000.0;
    } else if food_type == "W" {
        price = 2500.0;
    } else {
        println!("Invalid food type.");
        return;
    }

    println!("Enter quantity:");

    let mut quantity = String::new();
    io::stdin()
        .read_line(&mut quantity)
        .expect("Failed to read input");

    let quantity: f64 = quantity.trim().parse().expect("Please enter a number");

    let total = price * quantity;

    println!("Total before discount: ₦{:.2}", total);

    if total > 10000.0 {
        let discount = total * 0.05;
        let final_total = total - discount;

        println!("Discount: ₦{:.2}", discount);
        println!("Final total: ₦{:.2}", final_total);
    } else {
        println!("No discount.");
        println!("Final total: ₦{:.2}", total);
    }
}