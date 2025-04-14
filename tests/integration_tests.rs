use window_functions::window_functions;
use ::window_functions::{assert_with_decimal_places, macros};

mod test_setup;

#[test]
// test to see if the window returned matches the sample window
fn test_hanning_window_function() {
    assert_with_decimal_places!(window_functions::hanning(10), 
        test_setup::get_window_sample(test_setup::Window::HANNING));
}
#[test]
fn test_hamming_window_function() {
    assert_with_decimal_places!(window_functions::hamming(10),
    test_setup::get_window_sample(test_setup::Window::HAMMING));
}

#[test]
fn test_bartlett_window_function() {
    assert_with_decimal_places!(window_functions::bartlett(10), 
    test_setup::get_window_sample(test_setup::Window::BARTLETT));
}

#[test]
fn test_rectangle_window_function() {
    let result: Vec<i32> = std::iter::repeat(1).take(10).collect();
    assert_eq!(window_functions::rectangle(10),result)
}