pub fn chance(prob: f64) -> bool {
    rand::random::<f64>() < prob
}

pub fn exp_rand_val(lambda: f64) -> f64 {
    (-rand::random::<f64>().ln()) / lambda
}

pub fn exp_rand_int(expected: f64) -> u64 {
    exp_rand_val(1.0 / expected).ceil() as u64
}
