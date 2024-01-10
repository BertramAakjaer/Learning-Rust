use std::env;

fn main() {
    for (index, arguments) in env::args().enumerate() {
        println!("index value is {}, arguments value is {}", index, arguments);
    }
}
