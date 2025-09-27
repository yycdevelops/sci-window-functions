use crate::Consts;

pub enum Window {
    Hanning,
    Hamming,
    Nutall,
    Blackman,
    ExactBlackman,
    BlackmanHarris,
    BlackmanNutall,
    FlatTop,
}

#[derive(Copy, Clone, Debug)]
enum WindowType<T>
where
    T: Consts,
{
    COSINE {
        const_a: T,
        const_b: T,
        const_c: T,
        const_d: T,
        const_e: T,
    },
}

#[derive(Copy, Clone, Debug)]
pub struct WindowIter<T>
where
    T: Consts,
{
    size: usize,
    index: usize,
    window: WindowType<T>,
}

impl<T> Default for WindowIter<T>
where
    T: Consts,
{
    fn default() -> Self {
        Self {
            size: Default::default(),
            index: Default::default(),
            window: WindowType::COSINE {
                const_a: T::zero(),
                const_b: T::zero(),
                const_c: T::zero(),
                const_d: T::zero(),
                const_e: T::zero(),
            },
        }
    }
}

impl<T> Iterator for WindowIter<T>
where
    T: Consts,
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

impl<T> WindowIter<T>
where
    T: Consts,
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
                            / (T::from(self.size).unwrap() - T::one()))
                        .cos()
                    + T::from(const_c).unwrap()
                        * (T::PI() * T::from(4.0).unwrap() * (T::from(self.index).unwrap())
                            / (T::from(self.size).unwrap() - T::one()))
                        .cos()
                    - T::from(const_d).unwrap()
                        * (T::PI() * T::from(6.0).unwrap() * (T::from(self.index).unwrap())
                            / (T::from(self.size).unwrap() - T::one()))
                        .cos()
                    + T::from(const_e).unwrap()
                        * (T::PI() * T::from(8.0).unwrap() * (T::from(self.index).unwrap())
                            / (T::from(self.size).unwrap() - T::one()))
                        .cos()
            }
        }
    }
}

pub fn window<T>(size: usize, window: Window) -> WindowIter<T>
where
    T: Consts,
{
    match window {
        Window::Hanning => WindowIter {
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
        Window::Hamming => WindowIter {
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
        Window::Blackman => WindowIter {
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
        Window::BlackmanHarris => WindowIter {
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
        Window::Nutall => WindowIter {
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
        Window::BlackmanNutall => WindowIter {
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
        Window::FlatTop => WindowIter {
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
        Window::ExactBlackman => WindowIter {
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

mod tests {
    #[test]
    fn test_hanning_window() {
        let expected: Vec<f32> = vec![
            0.0, 0.11697778, 0.41317594, 0.75, 0.96984637, 0.96984625, 0.74999994, 0.4131757,
            0.11697769, 0.0,
        ];

        let window_hanning: Vec<f32> =
            crate::window_functions::window(10, crate::window_functions::Window::Hanning)
                .into_iter()
                .collect();
        assert_eq!(expected, window_hanning);
    }
    #[test]
    fn test_generic_iter() {
        let x: super::WindowIter<f32> = super::WindowIter {
            size: 10,
            index: 0,
            window: super::WindowType::COSINE {
                const_a: 1.0,
                const_b: 1.0,
                const_c: 1.0,
                const_d: 1.0,
                const_e: 1.0,
            },
        };

        assert_eq!(x.size, 10);
        assert_eq!(x.index, 0);
    }
}
