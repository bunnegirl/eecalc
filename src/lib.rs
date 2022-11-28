pub mod series;
pub mod units;

use series::*;
use std::f64::consts::PI;
pub use Arg::*;
pub use Input::*;

pub const SERIES_OPTIONS: [Series; 4] = [Series::E6, Series::E12, Series::E24, Series::E48];

pub const TOLERANCE_OPTIONS: [f64; 8] = [0.01, 0.02, 0.05, 0.1, 0.15, 0.2, 0.25, 0.3];

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Arg {
    ArgWithExact(f64),
    ArgWithTolerance(f64, f64),
    ArgWithSeries(Series, f64, f64),
}

impl Arg {
    pub fn to_table(&self) -> Vec<f64> {
        match self {
            ArgWithExact(value) => vec![*value],
            ArgWithTolerance(value, _) => vec![*value],
            ArgWithSeries(series, min, max) => series_table(series.clone(), *min, *max),
        }
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, PartialEq)]
pub enum Input {
    InputWithExact(Option<f64>, Option<f64>),
    InputWithTolerance(Option<f64>, f64, f64),
    InputWithSeries(Series, Option<f64>, Option<f64>, f64, f64),
}

impl Input {
    pub fn to_arg(&self) -> Option<Arg> {
        match self {
            InputWithSeries(series, min, max, min_fallback, max_fallback) => {
                let (min, max) = if let (Some(min), Some(max)) = (min, max) {
                    (min, max)
                } else if let (None, Some(max)) = (min, max) {
                    (min_fallback, max)
                } else if let (Some(min), None) = (min, max) {
                    (min, max_fallback)
                } else {
                    (min_fallback, max_fallback)
                };

                Some(ArgWithSeries(series.clone(), *min, *max))
            }
            InputWithTolerance(target, tolerance, fallback) => {
                let target = if let Some(target) = target {
                    target
                } else {
                    fallback
                };

                Some(ArgWithTolerance(*target, *tolerance))
            }
            InputWithExact(target, fallback) => match (target, fallback) {
                (Some(target), _) => Some(ArgWithExact(*target)),
                (None, Some(fallback)) => Some(ArgWithExact(*fallback)),
                (None, None) => None,
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Selection(f64, f64, f64, f64, f64, f64, f64);

impl Selection {
    pub fn frequency(&self) -> f64 {
        self.0
    }

    pub fn q_factor(&self) -> f64 {
        self.1
    }

    pub fn inductance(&self) -> f64 {
        self.2
    }

    pub fn r1_resistance(&self) -> f64 {
        self.3
    }

    pub fn r2_resistance(&self) -> f64 {
        self.4
    }

    pub fn c1_capacitance(&self) -> f64 {
        self.5
    }

    pub fn c2_capacitance(&self) -> f64 {
        self.6
    }
}

fn value_to_tolerance(value: Arg) -> (f64, f64, f64) {
    match value {
        ArgWithExact(target) => (target, target, target),
        ArgWithTolerance(target, tolerance) => (
            target,
            target - (target * tolerance),
            target + (target * tolerance),
        ),
        ArgWithSeries(_, min, max) => (min, min, max),
    }
}

pub fn calculate(
    frequency: Arg,
    q_factor: Arg,
    r1: Arg,
    r2: Arg,
    c1: Arg,
    c2: Arg,
) -> Vec<Selection> {
    let (frequency_target, frequency_minimum, frequency_maximum) = value_to_tolerance(frequency);
    let (_q_factor_target, q_factor_minimum, q_factor_maximum) = value_to_tolerance(q_factor);

    let r1_table = r1.to_table();
    let r2_table = r2.to_table();
    let c1_table = c1.to_table();
    let c2_table = c2.to_table();
    let mut results = Vec::new();

    for r1_value in &r1_table {
        let inductance_minimum = (r1_value / 10.0) / frequency_target;
        let inductance_maximum = (r1_value * 10.0) / frequency_target;

        for r2_value in &r2_table {
            for c2_value in &c2_table {
                let inductance = r1_value * r2_value * c2_value;

                if inductance >= inductance_minimum && inductance <= inductance_maximum {
                    for c1_value in &c1_table {
                        let frequency = 1.0 / (2.0 * PI * (inductance * c1_value).sqrt());
                        let q_factor = 2.0 * PI * frequency * inductance / r1_value;

                        if frequency >= frequency_minimum
                            && frequency <= frequency_maximum
                            && q_factor >= q_factor_minimum
                            && q_factor <= q_factor_maximum
                            && !results.contains(&Selection(
                                frequency, q_factor, inductance, *r1_value, *r2_value, *c1_value,
                                *c2_value,
                            ))
                        {
                            results.push(Selection(
                                frequency, q_factor, inductance, *r1_value, *r2_value, *c1_value,
                                *c2_value,
                            ));
                        }
                    }
                }
            }
        }
    }

    results.sort_by(|a, b| {
        if a.frequency() == b.frequency() {
            if a.q_factor() == b.q_factor() {
                if a.inductance() == b.inductance() {
                    if a.r1_resistance() == b.r1_resistance() {
                        if a.r2_resistance() == b.r2_resistance() {
                            if a.c1_capacitance() == b.c1_capacitance() {
                                a.c2_capacitance().partial_cmp(&b.c2_capacitance()).unwrap()
                            } else {
                                a.c1_capacitance().partial_cmp(&b.c1_capacitance()).unwrap()
                            }
                        } else {
                            a.r2_resistance().partial_cmp(&b.r2_resistance()).unwrap()
                        }
                    } else {
                        a.r1_resistance().partial_cmp(&b.r1_resistance()).unwrap()
                    }
                } else {
                    a.inductance().partial_cmp(&b.inductance()).unwrap()
                }
            } else {
                a.q_factor().partial_cmp(&b.q_factor()).unwrap()
            }
        } else {
            a.frequency().partial_cmp(&b.frequency()).unwrap()
        }
    });

    results
}
