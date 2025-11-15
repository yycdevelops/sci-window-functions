use num_traits::{Float, FloatConst};

pub mod window_functions;

pub trait Consts: Float + FloatConst {}

impl Consts for f32 {}
