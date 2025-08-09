pub fn round(x: f64) -> f64 {
    let place_value: f64 = 100000000.0;
    ((x*place_value).trunc())/place_value
}

pub fn result(values: [f64;2], operator: &str) -> f64 {
    let return_value: f64= match operator {
        "+" => values[0] + values[1],
        "-" => values[0] - values[1],
        "*" => values[0] * values[1],
        "/" => values[0] / values[1],
        "**" => values[0].powf(values[1]),
        _ => f64::NAN,
    };
    round(return_value)
}
