use std::env;

pub mod e1;
pub mod e2;
pub mod e3;
pub mod e4;

pub mod tools;
fn main() {
    let args = env::args().collect::<Vec<String>>();
    match args.len() {
        1 => panic!("Pass exercise number as argument: e1, e2, ..."),
        2 => match args[1].as_str() {
            "e1" => e1::run(),
            "e2" => e2::run(),
            "e3" => e3::run(),
            "e4" => e4::run(),
            _ => panic!("Unknown exercise"),
        },
        _ => panic!("Too much arguments"),
    };
}
