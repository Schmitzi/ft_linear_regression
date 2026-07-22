use ft_linear_regression::{load_thetas, read_csv};
use plotters::prelude::*;

pub fn plot(
    mileage: &[f64],
    price: &[f64],
    theta0: f64,
    theta1: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    // real axis ranges from your data, not hardcoded demo values
    let mileage_min = mileage.iter().cloned().fold(f64::INFINITY, f64::min);
    let mileage_max = mileage.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let price_min = price.iter().cloned().fold(f64::INFINITY, f64::min);
    let price_max = price.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

    let root = BitMapBackend::new("regression.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Mileage vs Price", ("sans-serif", 30).into_font())
        .margin(20)
        .x_label_area_size(30)
        .y_label_area_size(40)
        .build_cartesian_2d(mileage_min..mileage_max, price_min..price_max)?;

    chart.configure_mesh().draw()?;

    // your data points, as a scatter
    chart.draw_series(
        mileage
            .iter()
            .zip(price.iter())
            .map(|(&x, &y)| Circle::new((x, y), 3, BLUE.filled())),
    )?;

    // regression line: just two endpoints, since it's a straight line
    chart.draw_series(LineSeries::new(
        vec![
            (mileage_min, theta0 + theta1 * mileage_min),
            (mileage_max, theta0 + theta1 * mileage_max),
        ],
        &RED,
    ))?;

    root.present()?;
    Ok(())
}

fn main() {
    let csv = read_csv("data.csv");
    let (theta0, theta1) = load_thetas("thetas");

    if let Err(e) = plot(&csv[0], &csv[1], theta0, theta1) {
        println!("Plotting error: {}", e);
    }
}
