use ft_linear_regression::{estimate_price, load_thetas, read_csv};

fn main() {
    let csv = read_csv("data.csv");
    let (theta0, theta1) = load_thetas("thetas");

    let mileage = &csv[0];
    let price = &csv[1];
    let m = mileage.len();

    // mean of actual prices
    let mean_price: f64 = price.iter().sum::<f64>() / m as f64;

    let mut ss_res = 0.0;
    let mut ss_tot = 0.0;

    for i in 0..m {
        let predicted = estimate_price(mileage[i], theta0, theta1);
        ss_res += (price[i] - predicted) * (price[i] - predicted);
        ss_tot += (price[i] - mean_price) * (price[i] - mean_price);
    }

    let r_squared = 1.0 - (ss_res / ss_tot);

    println!("R²: {}", r_squared);
}
