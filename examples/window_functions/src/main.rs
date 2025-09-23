
use window_functions::window_functions::*;


fn main() {

    let window: GenericIter<f32> = window(50, Window::Hamming);

    for x in window {
        println!("{:?},", x);
    }
}
