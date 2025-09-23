use std::{path::Iter, sync::Arc};

use num_traits::{Float, FloatConst};
pub enum Window {
    HANNING,
    HAMMING,
    NUTTALL,
    BLACKMAN,
    EXACT_BLACKMAN,
    BLACKMAN_HARRIS,
    BLACKMAN_NUTALL,
    FLAT_TOP,
}

#[derive(Copy, Clone)]
enum WindowType<T>
where
    T: Float + FloatConst,
{
    COSINE {
        const_a: T,
        const_b: T,
        const_c: T,
        const_d: T,
        const_e: T,
    },
}

#[derive(Copy, Clone)]
struct GenericIter<T>
where
    T: Float + FloatConst,
{
    size: usize,
    index: usize,
    window: WindowType<T>,
}

impl<T> Iterator for GenericIter<T>
where
    T: Float + FloatConst,
{
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

impl<T> GenericIter<T>
where
    T: Float + FloatConst,
{
    fn calculate(self) -> T {
        match self.window {
            WindowType::COSINE {
                const_a,
                const_b,
                const_c,
                const_d,
                const_e,
            } => {
                T::from(const_a).unwrap()
                    - T::from(const_b).unwrap()
                        * (T::PI() * T::from(2.0).unwrap() * (T::from(self.index).unwrap())
                            / (T::from(self.size).unwrap() - T::from(1.0).unwrap()))
                        .cos()
                    + T::from(const_c).unwrap()
                        * (T::PI() * T::from(4.0).unwrap() * (T::from(self.index).unwrap())
                            / (T::from(self.size).unwrap() - T::from(1.0).unwrap()))
                        .cos()
                    - T::from(const_d).unwrap()
                        * (T::PI() * T::from(6.0).unwrap() * (T::from(self.index).unwrap())
                            / (T::from(self.size).unwrap() - T::from(1.0).unwrap()))
                        .cos()
                    + T::from(const_e).unwrap()
                        * (T::PI() * T::from(8.0).unwrap() * (T::from(self.index).unwrap())
                            / (T::from(self.size).unwrap() - T::from(1.0).unwrap()))
                        .cos()
            }
        }
    }
}

fn window<T>(size: usize, window: Window) -> GenericIter<T>
where
    T: Float + FloatConst,
{
    match window {
        Window::HANNING => GenericIter {
            size: size,
            index: 0,
            window: WindowType::COSINE {
                const_a: T::from(0.5).unwrap(),
                const_b: T::from(0.5).unwrap(),
                const_c: T::zero(),
                const_d: T::zero(),
                const_e: T::zero(),
            },
        },
        Window::HAMMING => GenericIter {
            size: size,
            index: 0,
            window: WindowType::COSINE {
                const_a: T::from(0.54).unwrap(),
                const_b: T::from(0.46).unwrap(),
                const_c: T::zero(),
                const_d: T::zero(),
                const_e: T::zero(),
            },
        },
        Window::BLACKMAN => GenericIter {
            size: size,
            index: 0,
            window: WindowType::COSINE {
                const_a: T::from(0.42).unwrap(),
                const_b: T::from(0.5).unwrap(),
                const_c: T::from(0.08).unwrap(),
                const_d: T::zero(),
                const_e: T::zero(),
            },
        },
        Window::BLACKMAN_HARRIS => GenericIter {
            size: size,
            index: 0,
            window: WindowType::COSINE {
                const_a: T::from(0.35875).unwrap(),
                const_b: T::from(0.48829).unwrap(),
                const_c: T::from(0.14128).unwrap(),
                const_d: T::from(0.01168).unwrap(),
                const_e: T::zero(),
            },
        },
        Window::NUTTALL => GenericIter {
            size: size,
            index: 0,
            window: WindowType::COSINE {
                const_a: T::from(0.355768).unwrap(),
                const_b: T::from(0.487396).unwrap(),
                const_c: T::from(0.144232).unwrap(),
                const_d: T::from(0.012604).unwrap(),
                const_e: T::zero(),
            },
        },
        Window::BLACKMAN_NUTALL => GenericIter {
            size: size,
            index: 0,
            window: WindowType::COSINE {
                const_a: T::from(0.3635819).unwrap(),
                const_b: T::from(0.3635819).unwrap(),
                const_c: T::from(0.1365995).unwrap(),
                const_d: T::from(0.0106411).unwrap(),
                const_e: T::zero(),
            },
        },
        Window::FLAT_TOP => GenericIter {
            size: size,
            index: 0,
            window: WindowType::COSINE {
                const_a: T::from(1.0).unwrap(),
                const_b: T::from(1.93).unwrap(),
                const_c: T::from(1.29).unwrap(),
                const_d: T::from(0.388).unwrap(),
                const_e: T::from(0.032).unwrap(),
            },
        },
        Window::EXACT_BLACKMAN => GenericIter {
            size: size,
            index: 0,
            window: WindowType::COSINE {
                const_a: T::from(0.4243801).unwrap(),
                const_b: T::from(0.4973406).unwrap(),
                const_c: T::from(0.0782793).unwrap(),
                const_d: T::zero(),
                const_e: T::zero(),
            },
        },
    }
}

// mod tests {
//     use crate::window_functions::{GenericIter, Window, window};

//     #[test]
//     fn test_something() {
//         let window: GenericIter<f32> = window(20, Window::EXACT_BLACKMAN);

//         for w in window {
//             println!("{:?},", w);
//         }

//         assert_eq!(10, 20);
//     }
// }
