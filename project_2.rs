use std::io;

fn main() {
    // Read experience
    println!("Is the employee experienced? (yes/no):");
    let mut experience_input = String::new();
    io::stdin()
        .read_line(&mut experience_input)
        .expect("Failed to read input");

    let is_experienced = experience_input.trim().eq_ignore_ascii_case("yes");

    // Read age
    println!("Enter the employee's age:");
    let mut age_input = String::new();
    io::stdin()
        .read_line(&mut age_input)
        .expect("Failed to read input");

    let age: u32 = age_input
        .trim()
        .parse()
        .expect("Please enter a valid age (number)");

    // Determine incentive
    let incentive = if !is_experienced {
        100_000
    } else if age >= 40 {
        1_560_000
    } else if age >= 30 && age <= 39 {
        1_480_000
    } else if age < 28 {
        1_300_000
    } else {
        // age is 28 or 29 – not covered by the given criteria
        println!("The given criteria do not cover experienced employees aged 28 or 29.");
        return;
    };

    println!("Annual incentive: ₦{}", incentive);
}