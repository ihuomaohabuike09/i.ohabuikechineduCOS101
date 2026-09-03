fn main() {
let p: f64 = 210000.0;
let r: f64 = 5.0;
let n: i32 = 3;

let value = p * (1.0 - r / 100.0).powi(n);

println!("Value of the TV after 3 years: #{:.2}", value);
}