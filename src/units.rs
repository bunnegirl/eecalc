const GIGA: f64 = 1000000000.0;
const MEGA: f64 = 1000000.0;
const KILO: f64 = 1000.0;
const MILLI: f64 = 0.001;
const MICRO: f64 = 0.000001;
const NANO: f64 = 0.000000001;
const PICO: f64 = 0.000000000001;
const FEMTO: f64 = 0.000000000000001;

pub fn parse_units(value: &str) -> f64 {
    let parser = |value: &str, unit: Option<char>| {
        let value = if let Some(unit) = unit {
            value.trim_end_matches(unit).replace(unit, ".")
        } else {
            value.into()
        };

        value.parse::<f64>().unwrap_or(0.0)
    };

    if value.contains(['f']) {
        parser(value, Some('f')) * FEMTO
    } else if value.contains(['p']) {
        parser(value, Some('p')) * PICO
    } else if value.contains('n') {
        parser(value, Some('n')) * NANO
    } else if value.contains('u') {
        parser(value, Some('u')) * MICRO
    } else if value.contains('m') {
        parser(value, Some('m')) * MILLI
    } else if value.contains('k') {
        parser(value, Some('k')) * KILO
    } else if value.contains('M') {
        parser(value, Some('M')) * MEGA
    } else if value.contains('G') {
        parser(value, Some('G')) * GIGA
    } else {
        parser(value, None)
    }
}

pub fn format_units(value: f64) -> String {
    let (value, unit) = if value >= GIGA {
        (value / GIGA, "G")
    } else if value >= MEGA {
        (value / MEGA, "M")
    } else if value >= KILO {
        (value / KILO, "k")
    } else if value >= 1.0 {
        (value, "")
    } else if value >= MILLI {
        (value / MILLI, "m")
    } else if value >= MICRO {
        (value / MICRO, "u")
    } else if value >= NANO {
        (value / NANO, "n")
    } else if value >= PICO {
        (value / PICO, "p")
    } else if value >= FEMTO {
        (value / FEMTO, "f")
    } else {
        (value, "")
    };

    format!(
        "{}{}",
        format!("{:.2}", value)
            .trim_end_matches('0')
            .trim_end_matches('.'),
        unit
    )
}
