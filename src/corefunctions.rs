pub fn round(x: f64) -> f64 {
    let place_value: f64 = 1000000.0;
    ((x * place_value).trunc()) / place_value
}
