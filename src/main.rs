mod action;
mod cmd;
mod helpers;

pub use std::io::Error;

pub type Result<T> = std::io::Result<T>;

fn main() {
    cmd::run().unwrap();
}
