use num_traits::{Float, FloatConst};

#[derive(Clone, Copy)]
pub enum Window {
    HANNING,
    HAMMING, 
    BLACKMAN, 
    BLACKMAN_HARRIS,
    FLAT_TOP,
    // BARTLETT,
    // COSINE,
    // LANCZOS,
    // NUTTAL,
    // RECTANGLE,
    // WELCH
}

enum WindowType<T> {
    COSINE {point_a: T, point_b: T}
}

pub struct WindowFunctionIterator {
    length: usize, 
    index: usize,
    window_type: Window
}

impl Iterator for WindowFunctionIterator {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.length {
            return None;
        }
        self.index += 1;
        let val = self.window_type.calculate_point(self.index as f32, self.length);        
        return Some(val);
    }
}

impl<T> WindowType<T> {     
    pub fn calculate(self, length: usize, point: T) -> T where T: Float + FloatConst {
        let len = T::from(length).unwrap();
        match self {
            WindowType::COSINE { point_a, point_b } => {
                T::from(point_a).unwrap() - T::from(point_b).unwrap() * (T::PI() * T::from(2.0).unwrap() * T::from(point).unwrap() / (len - T::from(1.0).unwrap())).cos()
            },
        }
    }
}

impl Window {
    pub fn calculate_point(self, point: f32, length: usize) -> f32 {
        
        let PI: f32 = FloatConst::PI();

        match self {
            Window::HANNING => WindowType::COSINE { point_a: 0.5, point_b: 0.5 }.calculate(length, point),
            Window::HAMMING => WindowType::COSINE { point_a: 0.54, point_b: 0.46 }.calculate(length, point),
            Window::BLACKMAN => todo!(),
            Window::BLACKMAN_HARRIS => {
                        WindowType::COSINE { point_a: 0.35875, point_b: 0.48829 }.calculate(length, point) 
                            + 0.1365995 * (4_f32 * PI * point / (length as f32 - 1.0)).cos() 
                            - 0.01168 * (6_f32 * PI * point / (length as f32 - 1.0)).cos()
                    },
            Window::FLAT_TOP => {
                WindowType::COSINE { point_a: 1.0, point_b: 1.93 }.calculate(length, point) + 1.29 * (4_f32 * PI / (length as f32 - 1.0)).cos() - 0.388 * (6_f32 * PI / (length as f32 - 1.0)).cos() + 0.032 * (8_f32 * PI / (length as f32 - 1.0)).cos()
            },
        }

            //Window::HANNING =>  { 0.5 - 0.5 * (PI * 2_f32 * point / (length as f32 - 1_f32)).cos() }
        //     Window::HAMMING =>  { 0.54 - 0.46 * (PI * 2_f32 * point / (length as f32 - 1_f32)).cos()},
        //     Window::BARTLETT => { 1_f32 - (2_f32 * (point - 0.5 * (length as f32- 1_f32)) / (length as f32 - 1_f32)).abs()},
        //     Window::COSINE =>   { (PI * point / (length as f32 - 1.0)).sin() },
        //     Window::LANCZOS => {  (|value: f32| { PI * value.sin() / (PI * value)}) ((2_f32 * point / (length as f32 - 1.0))-1.0)},
        //     Window::NUTTAL => todo!(),
        //     Window::RECTANGLE => { 1.0 },
        //     Window::WELCH => todo!(),
        // }
    }
    
}
pub fn window(length: usize, window_type: Window) -> WindowFunctionIterator {
    WindowFunctionIterator { length: length, index: 0, window_type: window_type }
}


mod tests {
    use crate::window_functions::{window, Window};

    #[test]
    fn test_something() {
        let window = window(100, Window::FLAT_TOP);

        for x in window {
            println!("{:?},", x);
        }

        assert_eq!(10, 20);
    }
}