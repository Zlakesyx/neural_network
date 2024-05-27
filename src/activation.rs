use std::f64::consts::E;

#[derive(Clone)]
pub struct Activation {
    pub function: &'static dyn Fn(f64) -> f64,
    pub derivative: &'static dyn Fn(f64) -> f64,
}

pub const SIGMOID: Activation = Activation {
    function: &|x| 1.0 / (1.0 + E.powf(-x)),
    derivative: &|x| x * (1.0 - x),
};

#[test]
fn test() {
    let x = 1.0;
}
