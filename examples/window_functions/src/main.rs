
use window_functions::window_functions::*;


fn main() {

    let window: WindowIter<f32> = window(50, Window::Hanning);

    for x in window {
        println!("{:?},", x);
    }
}
