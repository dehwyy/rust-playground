use std::fmt::Display;

use super::{PrintItemSize, TerminalPosition, TerminalSize};

const PRECISION: usize = 4;
const PADDING_Y: usize = 2;

#[derive(Default)]
pub enum Align {
    #[default]
    Left,
    Center,
    Right,
    TopLeft,
    TopCenter,
    TopRight,
    MiddleLeft,
    MiddleCenter,
    MiddleRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
}

impl Align {
    // fn get_left_offset(&self, (tw, th): TerminalSize, (w, h): PrintItemSize) -> usize {
    //     match () {
    //         _ if self.is_left() => 0,
    //         _ if self.is_middle_w() => (tw - w) / 2,
    //         _ => tw - w,
    //     }
    // }

    fn get_top_offset(&self, (tw, th): TerminalSize, (w, h): PrintItemSize) -> usize {
        match () {
            _ if self.is_top() => 0,
            _ if self.is_middle_h() => (th - h) / 2,
            _ => th - h,
        }
    }

    fn is_top(&self) -> bool {
        matches!(self, Align::TopLeft | Align::TopCenter | Align::TopRight)
    }

    fn is_middle_h(&self) -> bool {
        matches!(
            self,
            Align::MiddleLeft | Align::MiddleCenter | Align::MiddleRight
        )
    }

    fn is_left(&self) -> bool {
        matches!(
            self,
            Align::TopLeft | Align::MiddleLeft | Align::BottomLeft | Align::Left
        )
    }

    fn is_middle_w(&self) -> bool {
        matches!(
            self,
            Align::TopCenter | Align::MiddleCenter | Align::BottomCenter | Align::Center
        )
    }

    fn is_simple(&self) -> bool {
        matches!(self, Align::Left | Align::Center | Align::Right)
    }
}

pub struct FormatConfig {}

pub struct PrintConfig {
    precision: usize,
    align: Align,
}

impl Default for PrintConfig {
    fn default() -> Self {
        Self {
            precision: PRECISION,
            align: Align::default(),
        }
    }
}

impl PrintConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn align(mut self, align: Align) -> Self {
        self.align = align;
        self
    }

    pub fn precision(mut self, precision: usize) -> Self {
        self.precision = precision;
        self
    }
}

fn prerender(is_custom: bool) {
    super::clear();
    super::goto((1, 1));

    if !is_custom {
        (0..PADDING_Y).for_each(|_| println!());
    }
}

fn postrender() {
    let (_tw, th) = super::size();
    (0..PADDING_Y).for_each(|_| println!());
    super::goto((1, th - 1));
}

pub trait Print
where
    Self: Display,
{
    fn print(&self, cfg: &PrintConfig) {
        let s = format!("{self:.0$}", cfg.precision);
        let lines = s.lines().map(String::from).collect::<Vec<_>>();
        let (tw, th) = super::size();

        if !cfg.align.is_simple() {
            prerender(lines.len() < th);
            if lines.len() < th {
                let offset_y = cfg.align.get_top_offset(
                    (tw, th),
                    (
                        lines.iter().map(|l| l.chars().count()).max().unwrap_or(0),
                        lines.len(),
                    ),
                );

                (0..offset_y).for_each(|_| println!());
            }
        }

        for line in lines {
            match () {
                _ if cfg.align.is_left() => println!("{:<1$}", line, tw),
                _ if cfg.align.is_middle_w() => println!("{:^1$}", line, tw),
                _ => println!("{:>1$}", line, tw),
            }
        }

        if !cfg.align.is_simple() {
            postrender();
        }
    }
}
