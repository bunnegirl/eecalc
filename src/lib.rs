pub mod tables;
pub mod units;

use std::f64::consts::PI;
use tables::*;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct ToleranceValue(pub f64, pub f64);

impl ToleranceValue {
    pub fn target(&self) -> f64 {
        self.0
    }

    pub fn tolerance(&self) -> f64 {
        self.1
    }

    pub fn minimum(&self) -> f64 {
        let target = self.target();
        let tolerance = self.tolerance();

        target - (target * tolerance)
    }

    pub fn maximum(&self) -> f64 {
        let target = self.target();
        let tolerance = self.tolerance();

        target + (target * tolerance)
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum ComponentValue {
    Given(f64),
    Series(Series, f64, f64),
}

impl ComponentValue {
    pub fn to_table(&self) -> Vec<f64> {
        use ComponentValue::*;

        match self {
            Given(value) => vec![*value],
            Series(series, min, max) => series_table(series.clone(), *min, *max),
        }
    }
}

#[derive(Debug, PartialEq)]
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

pub fn calculate(
    frequency: ToleranceValue,
    q_factor: ToleranceValue,
    r1: ComponentValue,
    r2: ComponentValue,
    c1: ComponentValue,
    c2: ComponentValue,
) -> Vec<Selection> {
    let frequency_target = frequency.target();
    let frequency_minimum = frequency.minimum();
    let frequency_maximum = frequency.maximum();

    let q_factor_minimum = q_factor.minimum();
    let q_factor_maximum = q_factor.maximum();

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
