use std::fmt::Display;

const PRECISION: usize = 4;

#[derive(Default)]
pub enum Align {
    #[default]
    Left,
    Center,
    Right,
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

pub trait Print
where
    Self: Display,
{
    fn print(&self, cfg: &PrintConfig) {
        let s = format!("{self:.0$}", cfg.precision);
        let lines = s.lines().map(String::from).collect::<Vec<_>>();
        let (tw, _) = super::size();

        for line in lines {
            match cfg.align {
                Align::Left => println!("{:<1$}", line, tw),
                Align::Center => println!("{:^1$}", line, tw),
                Align::Right => println!("{:>1$}", line, tw),
            }
        }
    }
}
