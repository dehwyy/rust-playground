use crate::print_esc_ansi;

pub mod print;

pub type TerminalSize = (usize, usize);
pub type TerminalPosition = TerminalSize;
pub type PrintItemSize = (usize, usize);

pub fn size() -> TerminalSize {
    term_size::dimensions().expect("cannot get terminal size")
}

pub fn clear() {
    print_esc_ansi!("2J");
}

pub fn goto((x, y): TerminalPosition) {
    print_esc_ansi!(format!("{y};{x}H"));
}
