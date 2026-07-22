use ft_linear_regression::{estimate_price, load_thetas};
use std::io;

fn main() {
    // Load thetas from file
    let thetas = load_thetas("thetas");

    // prompt stdin
    println!("Please enter a mileage: ");

    // Create buffer
    let mut buffer = String::new();

    // Read from the stdin buffer
    let err = io::stdin().read_line(&mut buffer);
    if buffer.is_empty() || err.is_err() {
        return;
    }

    // Parse mileage from the input
    match buffer.trim().parse::<f64>() {
        Ok(mileage) => {
            let res = estimate_price(mileage, thetas.0, thetas.1);
            println!("Res: {}", res);
        }
        _ => println!("Error in prediction"),
    }
}
