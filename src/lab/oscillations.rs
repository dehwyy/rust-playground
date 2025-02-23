use std::io::Write;

struct Oscillation {
    period: f64,
    mean_deviation: f64,
}

impl Oscillation {
    fn new(period: f64, mean: f64) -> Self {
        Self {
            period,
            mean_deviation: period - mean,
        }
    }

    fn get_squared_deviation(&self) -> f64 {
        self.mean_deviation.powi(2)
    }
}

impl std::fmt::Display for Oscillation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Period: {:>8.2}, MeanDeviation: {:>8.4}, MeanDerivation^2: {:>8.5}",
            self.period,
            self.mean_deviation,
            self.get_squared_deviation()
        )
    }
}

macro_rules! number {
    () => {{
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        input.trim().parse().expect("Not a valid number")
    }};
}

macro_rules! prompt {
    ($($arg:tt)*) => {{
        print!($($arg)*);
        std::io::stdout().flush().unwrap();
    }};
}

pub fn run() {
    let mut data = Vec::<f64>::new();

    prompt!("Enter the number of oscillations: ");
    for i in 0..number!() {
        prompt!("Enter the period of oscillation n = {}: ", i + 1);
        data.push(number!());
    }

    let mean = data.iter().sum::<f64>() / data.len() as f64;

    let mut oscillations = data
        .into_iter()
        .map(|x| Oscillation::new(x, mean))
        .collect::<Vec<_>>();

    oscillations
        .iter()
        .enumerate()
        .for_each(|(i, osc)| println!("{n} {osc}", n = i + 1));

    println!("Mean: {mean:.4}");
    println!(
        "Deviation^2: {:.8}",
        oscillations
            .iter()
            .map(|x| x.get_squared_deviation())
            .sum::<f64>()
    );
}
