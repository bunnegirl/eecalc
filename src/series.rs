pub use Series::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Series {
    E6,
    E12,
    E24,
    E48,
}

impl Series {
    pub fn as_str(&self) -> String {
        match self {
            E6 => "e6".into(),
            E12 => "e12".into(),
            E24 => "e24".into(),
            E48 => "e48".into(),
        }
    }
}

impl From<String> for Series {
    fn from(value: String) -> Self {
        match value.as_str() {
            "e6" => E6,
            "e12" => E12,
            "e24" => E24,
            "e48" => E48,
            _ => E6,
        }
    }
}

const TABLE_VALUES: [(f64, Series); 69] = [
    (1.0, E6),
    (1.5, E6),
    (2.2, E6),
    (3.3, E6),
    (4.7, E6),
    (6.8, E6),
    (1.2, E12),
    (1.8, E12),
    (2.7, E12),
    (3.9, E12),
    (5.6, E12),
    (8.2, E12),
    (1.1, E24),
    (1.3, E24),
    (1.6, E24),
    (2.0, E24),
    (2.4, E24),
    (3.0, E24),
    (3.6, E24),
    (4.3, E24),
    (5.1, E24),
    (6.2, E24),
    (7.5, E24),
    (9.1, E24),
    (1.05, E48),
    (1.15, E48),
    (1.21, E48),
    (1.27, E48),
    (1.33, E48),
    (1.4, E48),
    (1.47, E48),
    (1.54, E48),
    (1.62, E48),
    (1.69, E48),
    (1.78, E48),
    (1.87, E48),
    (1.96, E48),
    (2.05, E48),
    (2.15, E48),
    (2.26, E48),
    (2.37, E48),
    (2.49, E48),
    (2.61, E48),
    (2.74, E48),
    (2.87, E48),
    (3.01, E48),
    (3.16, E48),
    (3.32, E48),
    (3.48, E48),
    (3.65, E48),
    (3.83, E48),
    (4.02, E48),
    (4.22, E48),
    (4.42, E48),
    (4.64, E48),
    (4.87, E48),
    (5.11, E48),
    (5.36, E48),
    (5.62, E48),
    (5.90, E48),
    (6.19, E48),
    (6.49, E48),
    (6.81, E48),
    (7.15, E48),
    (7.87, E48),
    (8.25, E48),
    (8.66, E48),
    (9.09, E48),
    (9.53, E48),
];

const TABLE_MULTIPLIERS: [f64; 24] = [
    // femto
    0.000000000000001,
    0.00000000000001,
    0.0000000000001,
    // pico
    0.000000000001,
    0.00000000001,
    0.0000000001,
    // nano
    0.000000001,
    0.00000001,
    0.0000001,
    // micro
    0.000001,
    0.00001,
    0.0001,
    // milli
    0.001,
    0.01,
    0.1,
    // units
    1.0,
    10.0,
    100.0,
    // kilo
    1000.0,
    10000.0,
    100000.0,
    // mega
    1000000.0,
    10000000.0,
    100000000.0,
];

pub fn series_table(table_series: Series, min_value: f64, max_value: f64) -> Vec<f64> {
    let mut values: Vec<f64> = TABLE_VALUES
        .iter()
        .filter_map(|(value, value_series)| {
            if table_series >= *value_series {
                let values: Vec<f64> = TABLE_MULTIPLIERS
                    .iter()
                    .filter_map(|multiplier| {
                        let value = *value * multiplier;

                        if value >= min_value && value < max_value {
                            Some(value)
                        } else {
                            None
                        }
                    })
                    .collect();

                Some(values)
            } else {
                None
            }
        })
        .flatten()
        .collect();

    values.sort_by(|lhs, rhs| lhs.partial_cmp(rhs).unwrap());

    values
}
