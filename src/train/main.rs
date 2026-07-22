use ft_linear_regression::{estimate_price, normalize_vec, read_csv, save_thetas};

const LEARNING_RATE: f64 = 0.1;
const EPSILON: f64 = 1e-6;
const MAX_ITER: i32 = 1000;

pub struct DataPoint {
    pub vec: Vec<f64>,
    pub min: f64,
    pub max: f64,
}

impl DataPoint {
    pub fn from_slice(data: &[f64]) -> DataPoint {
        let min = data.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        DataPoint {
            vec: normalize_vec(data),
            min,
            max,
        }
    }
}

fn main() {
    // load csv
    let csv = read_csv("data.csv");

    // Create thetas
    let mut theta0: f64 = 0.0;
    let mut theta1: f64 = 0.0;

    // Normalize vectors
    let mileage = DataPoint::from_slice(&csv[0]);
    let price = DataPoint::from_slice(&csv[1]);

    // run gradient descent

    let mut iterations = 0;
    loop {
        let m = mileage.vec.len();
        let mut sum0 = 0.0;
        let mut sum1 = 0.0;

        for i in 0..m {
            let error = estimate_price(mileage.vec[i], theta0, theta1) - price.vec[i];
            sum0 += error;
            sum1 += error * mileage.vec[i];
        }

        let tmp_theta0 = LEARNING_RATE * (sum0 / m as f64);
        let tmp_theta1 = LEARNING_RATE * (sum1 / m as f64);

        theta0 -= tmp_theta0;
        theta1 -= tmp_theta1;

        iterations += 1;

        if tmp_theta0.abs() < EPSILON && tmp_theta1.abs() < EPSILON || iterations >= MAX_ITER {
            break;
        }
    }

    // convert normalized thetas back to real scale
    let real_theta1 = (price.max - price.min) * theta1 / (mileage.max - mileage.min);
    let real_theta0 = price.min + (price.max - price.min) * theta0 - real_theta1 * mileage.min;

    // call save_thetas
    save_thetas("thetas", real_theta0, real_theta1);

    println!("Final thetas {}, {}", real_theta0, real_theta1);
}
