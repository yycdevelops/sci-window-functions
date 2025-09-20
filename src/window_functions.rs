use std::{path::Iter, sync::Arc};

use num_traits::{Float, FloatConst}; 
pub enum Window {
    HANNING,
    HAMMING
}

#[derive(Copy, Clone)]
enum WindowType<T> where T: Float + FloatConst {
    COSINE {const_a: T, const_b: T}
}

#[derive(Copy, Clone)]
struct GenericIter<T> where T: Float + FloatConst {
    size: usize, 
    index: usize,
    window: WindowType<T>
}

impl<T> Iterator for GenericIter<T> where T: Float + FloatConst {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.size {
            return None;
        }
        let y = self.calculate();
        self.index += 1;       
        Some(y)
    }
}

impl<T> GenericIter<T> where T: Float + FloatConst {
    fn calculate(self) -> T {
        match self.window {
            WindowType::COSINE { const_a , const_b} => {
                return T::from(const_a).unwrap() - T::from(const_b).unwrap() 
                    * (T::PI() * T::from(2.0).unwrap() * (T::from(self.index).unwrap()) / (T::from(self.size).unwrap() - T::from(1.0).unwrap())).cos()
            },
        }
    }

}

fn window<T>(window: Window) -> GenericIter<T> where T: Float + FloatConst {
    
    match window {
        Window::HANNING => {
                GenericIter { 
                        size: 30, 
                        index: 0,
                        window: WindowType::COSINE { const_a: T::from(0.5).unwrap(), const_b: T::from(0.5).unwrap() } 
                }
            },
        Window::HAMMING => {
            GenericIter {
                size: 30,
                index: 0,
                window: WindowType::COSINE { const_a: T::from(0.54).unwrap(), const_b: T::from(0.46).unwrap() }
            }
        },
    }
}

mod tests {
    use crate::window_functions::{window, GenericIter, Window};

    #[test]
    fn test_something() {
        let window: GenericIter<f32> = window(Window::HAMMING);

        for w in window {
            println!("{:?},", w);
        }

        assert_eq!(10, 20);
    }
}

