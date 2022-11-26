use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

#[derive(PartialEq)]
struct Calculation {
    rating: f64,
    frequency: f64,
    q_factor: f64,
    c1: f64,
    c2: f64,
    r1: f64,
    r2: f64,
}

#[derive(Properties, PartialEq)]
struct CalculationsProps {
    calculations: Vec<Calculation>,
}

#[function_component(Calculations)]
fn calculations(CalculationsProps { calculations }: &CalculationsProps) -> Html {
    calculations
        .iter()
        .map(|calculation| {
            html! {
                <tr>
                    <td>{format!("{:.2}", calculation.frequency)}</td>
                    <td>{format!("{:.2}", calculation.q_factor)}</td>
                    <td>{format!("{:.2}", calculation.c1)}</td>
                    <td>{format!("{:.2}", calculation.c2)}</td>
                    <td>{format!("{:.2}", calculation.r1)}</td>
                    <td>{format!("{:.2}", calculation.r2)}</td>
                </tr>
            }
        })
        .collect()
}

#[derive(Clone, PartialEq)]
enum SeriesTable {
    E6,
    E12,
    E24,
    E48,
}

impl std::fmt::Display for SeriesTable {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        use SeriesTable::*;

        write!(
            fmt,
            "{}",
            match self {
                E6 => "e6",
                E12 => "e12",
                E24 => "e24",
                E48 => "e48",
            }
        )
    }
}

#[derive(Clone, PartialEq)]
enum Value<T>
where
    T: std::fmt::Display + std::cmp::PartialEq,
{
    Set(T, T),
    NotSet(T),
}

#[derive(Properties, Clone, PartialEq)]
struct Series<T>
where
    T: std::fmt::Display + std::cmp::PartialEq,
{
    series: SeriesTable,
    minimum: Value<T>,
    maximum: Value<T>,
}

#[derive(Clone, PartialEq)]
struct Tolerance<T>
where
    T: std::fmt::Display + std::cmp::PartialEq,
{
    target: Value<T>,
    tolerance: f64,
}

#[derive(Clone, PartialEq)]
enum Frequency {
    Hertz(f64),
    KiloHertz(f64),
    MegaHertz(f64),
}

impl std::fmt::Display for Frequency {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        use Frequency::*;

        match self {
            Hertz(value) => write!(fmt, "{:.2}", value),
            KiloHertz(value) => write!(fmt, "{:.2}k", value),
            MegaHertz(value) => write!(fmt, "{:.2}m", value),
        }
    }
}

#[derive(Clone, PartialEq)]
enum Capacitance {
    PicoFarad(f64),
    NanoFarad(f64),
    MicroFarad(f64),
    MilliFarad(f64),
    Farad(f64),
}

impl std::fmt::Display for Capacitance {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        use Capacitance::*;

        match self {
            PicoFarad(value) => write!(fmt, "{:.2}p", value),
            NanoFarad(value) => write!(fmt, "{:.2}n", value),
            MicroFarad(value) => write!(fmt, "{:.2}u", value),
            MilliFarad(value) => write!(fmt, "{:.2}m", value),
            Farad(value) => write!(fmt, "{:.2}", value),
        }
    }
}

#[derive(Clone, PartialEq)]
enum Resistance {
    Ohm(f64),
    KiloOhm(f64),
    MegaOhm(f64),
}

impl std::fmt::Display for Resistance {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        use Resistance::*;

        match self {
            Ohm(value) => write!(fmt, "{:.2}", value),
            KiloOhm(value) => write!(fmt, "{:.2}k", value),
            MegaOhm(value) => write!(fmt, "{:.2}m", value),
        }
    }
}

#[derive(Properties, PartialEq)]
struct SeriesFieldProps<T>
where
    T: std::fmt::Display + std::cmp::PartialEq,
{
    id: String,
    title: String,
    series: Series<T>,
}

#[function_component(SeriesField)]
fn series_field<T>(props: &SeriesFieldProps<T>) -> Html
where
    T: std::fmt::Display + std::cmp::PartialEq,
{
    let minimum_handle = use_state(String::default);
    let minimum = (*minimum_handle).clone();

    let series_options = vec![
        SeriesTable::E6,
        SeriesTable::E12,
        SeriesTable::E24,
        SeriesTable::E48,
    ];

    let on_minimum_change = Callback::from(move |e: Event| {
        let target: EventTarget = e
            .target()
            .expect("Event should have a target when dispatched");

        minimum_handle.set(target.unchecked_into::<HtmlInputElement>().value());
    });

    html! {
        <>
            <div class="field">
                <label for={format!("{}-series", props.id)}>{&props.title}</label>
                <select id={format!("{}-series", props.id)}>
                    {
                        series_options.clone().into_iter().map(|series_option| {
                            html!{<option selected={props.series.series == series_option}>{ series_option }</option>}
                        }).collect::<Html>()
                    }
                </select>
            </div>
            <div class="field">
                <label for={format!("minimum-{}", props.id)}>{"capacitance range"}</label>
                {
                    match &props.series.minimum {
                        Value::Set(value, placeholder) => {
                            html! {
                                <input
                                    id={format!("minimum-{}", props.id)}
                                    onchange={on_minimum_change}
                                    value={format!("{}", value)}
                                    placeholder={format!("{}", placeholder)} />
                            }
                        }
                        Value::NotSet(placeholder) => {
                            html! {
                                <input
                                    id={format!("minimum-{}", props.id)}
                                    onchange={on_minimum_change}
                                    placeholder={format!("{}", placeholder)} />
                            }
                        }
                    }
                }
                {
                    match &props.series.maximum {
                        Value::Set(value, placeholder) => {
                            html! {
                                <input
                                    id={format!("maximum-{}", props.id)}
                                    value={format!("{}", value)}
                                    placeholder={format!("{}", placeholder)} />
                            }
                        }
                        Value::NotSet(placeholder) => {
                            html! {
                                <input
                                    id={format!("maximum-{}", props.id)}
                                    placeholder={format!("{}", placeholder)} />
                            }
                        }
                    }
                }
            </div>
        </>
    }
}

#[derive(Properties, PartialEq)]
struct FormProps {
    capacitance_series: Series<Capacitance>,
    resistance_series: Series<Resistance>,
    frequency_target: Tolerance<Frequency>,
    // q_factor_target: Tolerance<f64>,
    // c1_target: Capacitance,
    // c2_target: Capacitance,
    // r1_target: Resistance,
    // r2_target: Resistance,
}

#[function_component(Form)]
fn form(props: &FormProps) -> Html {
    let series_options = vec![
        SeriesTable::E6,
        SeriesTable::E12,
        SeriesTable::E24,
        SeriesTable::E48,
    ];

    html! {
        <>
            <h1>{"gyrator calculator"}</h1>
            <p>{"this calculator aids in the design of gyrator based filters by selecting appropriate values for the desired q and frequency"}</p>

            <h2>{"component ranges"}</h2>
            <div class="fieldset">
                <SeriesField<Capacitance>
                    id="capacitance"
                    title="capacitance series"
                    series={props.capacitance_series.clone()}
                />
                <SeriesField<Resistance>
                    id="resistance"
                    title="resistance series"
                    series={props.resistance_series.clone()}
                />
            </div>
        </>
    }
}

#[function_component(App)]
fn app() -> Html {
    // let selected_video = use_state(|| None);

    let capacitance_series = Series {
        series: SeriesTable::E6,
        minimum: Value::NotSet(Capacitance::NanoFarad(1.0)),
        maximum: Value::NotSet(Capacitance::MicroFarad(100.0)),
    };
    let resistance_series = Series {
        series: SeriesTable::E24,
        minimum: Value::NotSet(Resistance::KiloOhm(1.0)),
        maximum: Value::NotSet(Resistance::KiloOhm(100.0)),
    };
    let frequency_target = Tolerance {
        target: Value::NotSet(Frequency::Hertz(100.0)),
        tolerance: 0.1,
    };

    let calculations = vec![Calculation {
        rating: 0.0,
        frequency: 40.0,
        q_factor: 4.0,
        c1: 1.0,
        c2: 1.0,
        r1: 1.0,
        r2: 1.0,
    }];

    html! {
        <main>
            <Form
                capacitance_series={capacitance_series} resistance_series={resistance_series}
                frequency_target={frequency_target}
            />
            <table>
                <Calculations calculations={calculations} />
            </table>
        </main>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
