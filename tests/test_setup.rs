// Generic Enum for all of the supported Window Functions supported
// Put this into its own file that is shared throughout the crate. 
// This should not be associated to any test files 

use std::vec;

pub enum Window { 
    HANNING,
    HAMMING,
    BARTLETT,
    TRIANGULAR,
    COSINE,
    LANCZOS,
    NUTTALL,
    BLACKMAN_HARRIS,
}

// Window sampling from Python library
// parameter is `Window` which is an enum 
pub fn get_window_sample(window: Window) -> Vec<f32> {
    match window {
        Window::HANNING => {
            sample_hanning_window()
        },
        Window::HAMMING => {
            sample_hamming_window()
        },
        Window::BARTLETT => {
            sample_bartlett_window()
        },
        Window::BLACKMAN_HARRIS => {
            sample_blackman_window()
        }
        _ => {
            vec![]
        }
    }
}

fn sample_hanning_window() -> Vec<f32> {
    vec![0.0, 0.11697778, 0.41317591, 0.75, 0.96984631, 0.96984631, 0.75, 0.41317591, 0.11697778, 0.0]
}

fn sample_hamming_window() -> Vec<f32> {
    vec![0.08, 0.18761956, 0.46012184, 0.77, 0.97225861, 0.97225861, 0.77, 0.46012184, 0.18761956, 0.08]
}

fn sample_bartlett_window() -> Vec<f32> {
    vec![0.0,0.22222222,0.44444444,0.66666667,0.88888889,0.88888889 ,0.66666667,0.44444444,0.22222222,0.0]
}

fn sample_blackman_window() -> Vec<f32> {
    vec![-1.38777878e-17, 5.08696327e-02,2.58000502e-01,6.30000000e-01
    ,9.51129866e-01,9.51129866e-01,6.30000000e-01,2.58000502e-01
    ,5.08696327e-02, -1.38777878e-17]
}