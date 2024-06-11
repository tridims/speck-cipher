use statrs::distribution::{ChiSquared, ContinuousCDF};

fn main() {
    const NUM_BINS: usize = 256;
    let degrees_of_freedom = NUM_BINS - 1;
    let significance_level = 0.05;
    let critical_value = ChiSquared::new(degrees_of_freedom as f64)
        .unwrap()
        .inverse_cdf(1.0 - significance_level);
    println!("Critical value: {}", critical_value)
}
