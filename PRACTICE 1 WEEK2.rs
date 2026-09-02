fn main() {
    let p: f64 = 520_000_000.0;
    let r: f64 = 10.0;
    let n: i32 = 5;

    let amount = p * (1.0 + r / 100.0).powi(n);
    let compound_interest = amount - p;

    println!("Amount = {:.2}", amount);
    println!("Compound Interest = {:.2}", compound_interest);
}