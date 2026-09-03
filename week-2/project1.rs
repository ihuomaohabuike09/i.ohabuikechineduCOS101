fn main() {
let principal: f64 = 520_000_000.0;
let rate: f64 = 10.0;
let years: i32 = 5;
// Formula: A = P * (1 + R/100)^n
let amount = principal * (1.0 + rate / 100.0).powi(years);
let compound_interest = amount - principal;
println!("Total Amount: {}", amount);
println!("Compound Interest: {}", compound_interest);
}