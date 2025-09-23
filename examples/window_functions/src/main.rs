
use window_functions::window_functions::*;


fn main() {

    let window: WindowIter<f32> = window(30, Window::ExactBlackman);

    for x in window {
        println!("{:?}", x);
    }
}
