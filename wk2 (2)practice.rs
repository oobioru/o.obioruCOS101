fn main() {
    let amounts: [f64; 5] = [
        450_000.0,  // Toshiba
        1_500_000.0, // Mac
        750_000.0,  // HP
        2_850_000.0, // Dell
        250_000.0,  // Acer
    ];

    let sum: f64 = amounts.iter().sum();
    let average = sum / amounts.len() as f64;

    println!("Total sum: N{:.2}", sum);
    println!("Average: N{:.2}", average);
output
Total sum: N5800000.00
Average: N1160000.00