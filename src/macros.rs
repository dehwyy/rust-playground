#[macro_export]
macro_rules! esc_ansi {
    ($s:expr) => {
        format!("\x1b[{}", $s)
    };
}

#[macro_export]
macro_rules! print_esc_ansi {
    ($s:expr) => {
        print!("\x1b[{}", $s)
    };
}
