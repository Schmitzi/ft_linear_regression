use std::fs;

/// The hypothesis function: predicts a price from a mileage,
/// given the current model parameters.
pub fn estimate_price(mileage: f64, theta0: f64, theta1: f64) -> f64 {
    theta0 + (theta1 * mileage)
}

/// Loads theta0 and theta1 from disk. Returns (0.0, 0.0) if the
/// file doesn't exist yet (i.e. training hasn't run).
pub fn load_thetas(path: &str) -> (f64, f64) {
    let theta0: f64;
    let theta1: f64;
    // load_thetas
    if !fs::exists(path).expect("File not found") {
        theta0 = 0.0;
        theta1 = 0.0;
    } else {
        let input = fs::read_to_string(path).unwrap();
        let parts: Vec<&str> = input.trim().split(',').collect();
        theta0 = parts[0].parse().unwrap();
        theta1 = parts[1].parse().unwrap();
    }
    return (theta0, theta1);
}

/// Saves theta0 and theta1 to disk for `predict` to pick up later.
pub fn save_thetas(path: &str, theta0: f64, theta1: f64) {
    let content = format!("{},{}", theta0, theta1);
    let err = fs::write(path, content);
    if err.is_err() {
        println!("Write Error!");
    }
}

pub fn read_csv(path: &str) -> Vec<Vec<f64>> {
    let file = std::fs::File::open(path).unwrap();
    let mut rdr = csv::Reader::from_reader(file);

    let mut vec1: Vec<f64> = Vec::new();
    let mut vec2: Vec<f64> = Vec::new();

    for result in rdr.records() {
        match result {
            Ok(line) => {
                let mileage_str = line.get(0).unwrap();
                let price_str = line.get(1).unwrap();
                vec1.push(mileage_str.parse().unwrap());
                vec2.push(price_str.parse().unwrap());
            }
            _ => panic!("CSV Error"),
        };
    }

    let mut ret: Vec<Vec<f64>> = Vec::new();
    ret.push(vec1);
    ret.push(vec2);
    return ret;
}

pub fn normalize_vec(data: &[f64]) -> Vec<f64> {
    let mut ret: Vec<f64> = Vec::new();
    let min: f64 = data.iter().cloned().fold(f64::INFINITY, f64::min);
    let max: f64 = data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

    for i in 0..data.len() {
        ret.push((data[i] - min) / (max - min));
    }

    ret
}
