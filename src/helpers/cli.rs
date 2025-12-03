pub use colored::Colorize;

macro_rules! info {
    ($($arg:tt)*) => {
        println!("{} {}", "::".blue().bold(), format_args!($($arg)*))
    };
}
pub(crate) use info;

macro_rules! err {
    ($($arg:tt)*) => {
	eprintln!("{} {}", "Error:".red().bold(), format_args!($($arg)*))
    };
}
pub(crate) use err;
