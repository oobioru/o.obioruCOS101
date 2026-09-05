fn main() {
    let p: f64 = 210_000.0; // original price
    let r: f64 = 5.0; // Depreciation rate (%)
    let n: f64 = 3.0; // Number of years

    let a = p * (1.0 - (r / 100.0)).powf(n);

    println!("Original price: N{:.2}", p);
    println!("Value after 3 years: N{:.2}", a);
}
output
Original price: N210000.00
Value after 3 years: N180048.75