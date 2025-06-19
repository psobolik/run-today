#[macro_export]
macro_rules! print_info {
    ($($arg:tt)*) => {{
        let message = format!($($arg)*);
        println!("{} {message}", $crate::colored::Colorize::bright_green("→"))
    }};
}

#[macro_export]
macro_rules! print_error {
    ($($arg:tt)*) => {{
        let message = format!($($arg)*);
        eprintln!("{} {message}", $crate::colored::Colorize::bright_red("→"))
    }};
}
