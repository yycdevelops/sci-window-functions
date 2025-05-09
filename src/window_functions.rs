use core::f64;
use std::f32::consts::PI;

static PI_F: f64 = f64::consts::PI;


pub fn hanning(num: usize) -> Vec<f32> {
    (0..num).map(|x| {0.5 - 0.5 * (PI * 2_f32 * (x as f32) / (num as f32 - 1_f32)).cos()}).collect()
}

pub fn hamming(num: usize) -> Vec<f32> {
    (0..num).map(|x| { 0.54 - 0.46 * (PI * 2_f32 * x as f32 / (num as f32 - 1_f32)).cos()}).collect()
}

pub fn bartlett(num: usize) -> Vec<f32> {
    (0..num).map(|x|{1_f32 - (2_f32 * (x as f32 - 0.5 * (num as f32- 1_f32)) / (num as f32 - 1_f32)).abs()}).collect()
}

pub fn triangular(num: usize) -> Vec<f32> {
    let demoninator: i32 = if num % 2 != 0 { 
        num as i32 + 1 as i32
    }else{
        num as i32
    };
    (0..num).map(|val| {
        (1.0 - (2.0 * val as f32 - (num as f32 - 1.0)).abs() / demoninator as f32) as f32
    }).collect()
}

pub fn cosine(num: usize) -> Vec<f32> {
    (0..num).map(|x| {(PI * x as f32 / (num as f32 - 1.0)).sin()}).collect()
}

pub fn lanczos(num: usize) -> Vec<f32> {
    (0..num).map(|x| {
        (|value: f32| { PI * value.sin() / (PI * value)})((2_f32 * x as f32 / (num as f32 - 1.0))-1.0)
    }).collect()
}

pub fn nuttall(num: usize) -> Vec<f32> {
    (0..num).map(|x| {
        0.355768 - 0.487396 * point(2_f32, x, num).cos() 
            + 0.144232 * point(4_f32, x, num).cos()  
            - 0.012604 * point(6_f32, x, num).cos()   
    }).collect()
}

pub fn blackman(num: usize) -> Vec<f64> {
    (0..num).map(|x| {
        let f: f64 = PI_F*2.0*x as f64/num as f64 - 1.0;
        0.42 - 0.5 * cos(f) + 0.08 * cos(2_f64*f)
    }).collect()
}

pub fn rectangle(num: usize) -> Vec<i32> {
    std::iter::repeat(1).take(num).collect()
}

pub fn blackman_harris(num: usize) -> Vec<f32> {
    (0..num).map(|x| {
        0.35875 - 0.48829 * point(2_f32, x, num).cos()
        + 0.14128 * point(4_f32, x, num).cos()
        - 0.01168 * point(6_f32, x, num).cos()
    }).collect()
}

pub fn blackman_nuttall(num: usize) -> Vec<f32> {
    (0..num).map(|x| {
        0.3635819 - 0.3635819*point(2_f32, x, num).cos()
        + 0.1365995*point(4_f32, x, num).cos()
        - 0.0106411*point(6_f32, x, num).cos()
    }).collect()
}

pub fn flat_top(num: usize) -> Vec<f32> {
    (0..num).map(|x| {
        1.0 - 1.93*point(2_f32, x, num).cos()
        + 1.29*point(4_f32, x, num).cos()
        - 0.388*point(6_f32, x, num).cos()
        + 0.032*point(8_f32, x, num).cos()
    }).collect()
}

pub fn welch(num: usize) -> Vec<f64> {
    let num12: f64 = 0.5 * (num as f64-1.0);
    (0..num).map(|x| {
        let f: f64 = (x as f64 - num12)/num12;
        1.0 - f*f
    }).collect()
}


fn point(data_point: f32, x: usize, num: usize) -> f32 {
    data_point * PI * x as f32 / (num as f32 - 1.0)
}

fn cos(entry: f64) -> f64 {
    entry.cos()
}

mod tests {

    #[test]
    fn test_point() {
        let point = crate::window_functions::point(2_f32, 10, 10); 
        assert_eq!((2_f32 * std::f32::consts::PI * 10.0 / (10.0 - 1.0)), point);
    }

    #[test]
    fn test_cos() {
        let cos = (10.0 as f64).cos();
        assert_eq!(cos, crate::window_functions::cos(10.0));
    }
}