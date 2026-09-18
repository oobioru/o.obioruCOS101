use std::io;

fn main() {
    let mut input = String::new();
    
    println!("Enter a, b and c separated by spaces:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let values: Vec<f64> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Please enter valid numbers"))
        .collect();

    if values.len() != 3 {
        println!("Please enter exactly three numbers");
        return;
    }

    let a = values[0];
    let b = values[1];
    let c = values[2];

    // Special case: if a is zero, it's not quadratic
    if a == 0.0 {
        println!("This is not a quadratic equation (a = 0)");
        return;
    }

    let d = b * b - 4.0 * a * c;

    if d > 0.0 {
        let root1 = (-b + d.sqrt()) / (2.0 * a);
        let root2 = (-b - d.sqrt()) / (2.0 * a);
        println!("Two distinct roots: {:.4} and {:.4}", root1, root2);
    } 
    else if d == 0.0 {
        let root = -b / (2.0 * a);
        println!("Exactly one real root: {:.4}", root);
    } 
    else {
        println!("No real roots");
    }



(ps: i wont lie i actually used AI to debug this because it was hard. i could only go so far:( )
