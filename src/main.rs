mod action;
mod cmd;
mod helpers;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() {
    cmd::run().unwrap();
}
