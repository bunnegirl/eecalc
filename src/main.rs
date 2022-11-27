use std::result;

use gloo_console::log;
use gyrator_calculator::tables::*;
use gyrator_calculator::units::*;
use gyrator_calculator::*;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

// fn main() {
//     let mut selections = calculate(
//         ToleranceValue(parse_units("100"), 0.1),
//         ToleranceValue(parse_units("4"), 0.2),
//         ComponentValue::Given(parse_units("470")),
//         ComponentValue::Series(E24, parse_units("1k"), parse_units("100k")),
//         ComponentValue::Series(E6, parse_units("1n"), parse_units("100u")),
//         ComponentValue::Series(E6, parse_units("1n"), parse_units("100u")),
//     );

//     selections.sort_by(|a, b| {
//         if a.frequency() == b.frequency() {
//             if a.q_factor() == b.q_factor() {
//                 if a.inductance() == b.inductance() {
//                     if a.r1_resistance() == b.r1_resistance() {
//                         if a.r2_resistance() == b.r2_resistance() {
//                             if a.c1_capacitance() == b.c1_capacitance() {
//                                 a.c2_capacitance().partial_cmp(&b.c2_capacitance()).unwrap()
//                             } else {
//                                 a.c1_capacitance().partial_cmp(&b.c1_capacitance()).unwrap()
//                             }
//                         } else {
//                             a.r2_resistance().partial_cmp(&b.r2_resistance()).unwrap()
//                         }
//                     } else {
//                         a.r1_resistance().partial_cmp(&b.r1_resistance()).unwrap()
//                     }
//                 } else {
//                     a.inductance().partial_cmp(&b.inductance()).unwrap()
//                 }
//             } else {
//                 a.q_factor().partial_cmp(&b.q_factor()).unwrap()
//             }
//         } else {
//             a.frequency().partial_cmp(&b.frequency()).unwrap()
//         }
//     });

//     println!(
//         "{:<10} {:<10} {:<12} {:<8} {:<8} {:<8} {:<8}",
//         "frequency", "q factor", "inductance", "r1", "r2", "c1", "c2"
//     );

//     for selection in &selections {
//         println!(
//             "{:<10} {:<10} {:<12} {:<8} {:<8} {:<8} {:<8}",
//             format_units(selection.frequency()),
//             format_units(selection.q_factor()),
//             format_units(selection.inductance()),
//             format_units(selection.r1_resistance()),
//             format_units(selection.r2_resistance()),
//             format_units(selection.c1_capacitance()),
//             format_units(selection.c2_capacitance())
//         );
//     }

//     println!("found {} combinations", selections.len());
// }

const TOLERANCE_OPTIONS: [f64; 8] = [0.01, 0.02, 0.05, 0.1, 0.15, 0.2, 0.25, 0.3];

const SERIES_OPTIONS: [Series; 4] = [Series::E6, Series::E12, Series::E24, Series::E48];

use InputType::*;

#[allow(clippy::enum_variant_names)]
#[derive(Debug, PartialEq)]
enum InputType {
    NumberInput(Option<f64>, Option<f64>),
    SeriesInput(Series, Option<f64>, Option<f64>, f64, f64),
    ToleranceInput(Option<f64>, f64, f64),
}

#[derive(Properties, PartialEq)]
struct InputProps {
    id: &'static str,
    name: &'static str,
    note: Option<&'static str>,
    value: UseStateHandle<InputType>,
}

#[function_component(Input)]
fn input(
    InputProps {
        id,
        name,
        note,
        value,
    }: &InputProps,
) -> Html {
    let on_series_change = {
        let state = value.clone();

        move |event: Event| {
            let element: EventTarget = event
                .target()
                .expect("Event should have a target when dispatched");

            if let SeriesInput(_, min, max, min_fallback, max_fallback) = &*state {
                let series = Series::from(element.unchecked_into::<HtmlInputElement>().value());

                state.set(SeriesInput(
                    series,
                    *min,
                    *max,
                    *min_fallback,
                    *max_fallback,
                ));
            }
        }
    };

    let on_min_change = {
        let state = value.clone();

        move |event: Event| {
            let element: EventTarget = event
                .target()
                .expect("Event should have a target when dispatched");

            if let SeriesInput(series, _, max, min_fallback, max_fallback) = &*state {
                let value = &element.unchecked_into::<HtmlInputElement>().value();

                let min = if value.trim().is_empty() {
                    None
                } else {
                    Some(parse_units(value))
                };

                state.set(SeriesInput(
                    series.clone(),
                    min,
                    *max,
                    *min_fallback,
                    *max_fallback,
                ));
            }
        }
    };

    let on_max_change = {
        let state = value.clone();

        move |event: Event| {
            let element: EventTarget = event
                .target()
                .expect("Event should have a target when dispatched");

            if let SeriesInput(series, min, _, min_fallback, max_fallback) = &*state {
                let value = &element.unchecked_into::<HtmlInputElement>().value();

                let max = if value.trim().is_empty() {
                    None
                } else {
                    Some(parse_units(value))
                };

                state.set(SeriesInput(
                    series.clone(),
                    *min,
                    max,
                    *min_fallback,
                    *max_fallback,
                ));
            }
        }
    };

    let on_target_change = {
        let state = value.clone();

        move |event: Event| {
            let element: EventTarget = event
                .target()
                .expect("Event should have a target when dispatched");
            let value = &element.unchecked_into::<HtmlInputElement>().value();

            if let NumberInput(_, fallback) = &*state {
                if value.trim().is_empty() {
                    state.set(NumberInput(None, *fallback));
                } else {
                    state.set(NumberInput(Some(parse_units(value)), *fallback));
                }
            } else if let ToleranceInput(_, tolerance, fallback) = &*state {
                if value.trim().is_empty() {
                    state.set(ToleranceInput(None, *tolerance, *fallback));
                } else {
                    state.set(ToleranceInput(
                        Some(parse_units(value)),
                        *tolerance,
                        *fallback,
                    ));
                }
            }
        }
    };

    let on_tolerance_change = {
        let state = value.clone();

        move |event: Event| {
            let element: EventTarget = event
                .target()
                .expect("Event should have a target when dispatched");

            if let ToleranceInput(target, _, fallback) = &*state {
                let value = &element.unchecked_into::<HtmlInputElement>().value();

                state.set(ToleranceInput(*target, parse_units(value), *fallback));
            }
        }
    };

    if let SeriesInput(series, min, max, min_fallback, max_fallback) = &*(value.clone()) {
        html! {
            <>
                <div class="field">
                    <label for={format!("{}-series", id)}>{format!("{} series", name)}</label>
                    <select id={format!("{}-series", id)} onchange={on_series_change}>
                        {
                            SERIES_OPTIONS.iter().map(|item_series| html! {
                                <option selected={series == item_series}>{item_series.as_str()}</option>
                            }).collect::<Html>()
                        }
                    </select>
                </div>
                <div class="field">
                    <label for={format!("{}-range", id)}>{format!("{} range", name)}</label>
                    <input
                        id={format!("{}-range", id)}
                        placeholder={format_units(*min_fallback)}
                        value={
                            if let Some(min) = min {
                                format_units(*min)
                            } else {
                                "".into()
                            }
                        }
                        onchange={on_min_change}
                    />
                    <input
                        placeholder={format_units(*max_fallback)}
                        value={
                            if let Some(max) = max {
                                format_units(*max)
                            } else {
                                "".into()
                            }
                        }
                        onchange={on_max_change}
                    />
                </div>
            </>
        }
    } else if let NumberInput(value, fallback) = &*(value.clone()) {
        html! {
            <div class="field">
                <label for={format!("{}-target", id)}>{{format!("{} target", name)}}</label>
                <input
                    id={format!("{}-target", id)}
                    placeholder={
                        if let Some(value) = fallback {
                            format_units(*value)
                        } else {
                            "optional".into()
                        }
                    }
                    value={
                        if let Some(value) = value {
                            format_units(*value)
                        } else {
                            "".into()
                        }
                    }
                    onchange={on_target_change}
                />
                {
                    if let Some(note) = note {
                        html! {<p>{note}</p>}
                    } else {
                        html! {}
                    }
                }
            </div>
        }
    } else if let ToleranceInput(value, tolerance, fallback) = &*(value.clone()) {
        html! {
            <div class="field">
                <label for={format!("{}-target", id)}>{format!("{} target", name)}</label>
                <input
                    id={format!("{}-target", id)}
                    placeholder={format_units(*fallback)}
                    value={
                        if let Some(value) = value {
                            format_units(*value)
                        } else {
                            "".into()
                        }
                    }
                    onchange={on_target_change}
                />
                <select id={format!("{}-tolerance", id)} onchange={on_tolerance_change}>
                    {
                        TOLERANCE_OPTIONS.iter().map(|item_tolerance| html! {
                            <option
                                selected={tolerance == item_tolerance}
                                value={format_units(*item_tolerance)}>
                                {format!("±{}%", item_tolerance * 100.0)}
                            </option>
                        }).collect::<Html>()
                    }
                </select>
            </div>
        }
    } else {
        html! {}
    }
}

#[function_component]
fn App() -> Html {
    let results = use_state(|| None);
    let capacitance_value = use_state(|| {
        SeriesInput(
            Series::E6,
            None,
            None,
            parse_units("1n"),
            parse_units("100u"),
        )
    });
    let resistance_value = use_state(|| {
        SeriesInput(
            Series::E24,
            None,
            None,
            parse_units("1k"),
            parse_units("100k"),
        )
    });
    let frequency_value = use_state(|| ToleranceInput(None, 0.1, 100.0));
    let q_factor_value = use_state(|| ToleranceInput(None, 0.2, 4.0));
    let r1_value = use_state(|| NumberInput(None, Some(470.0)));
    let r2_value = use_state(|| NumberInput(None, None));
    let c1_value = use_state(|| NumberInput(None, None));
    let c2_value = use_state(|| NumberInput(None, None));

    let onclick = {
        let results = results.clone();
        let capacitance_value = capacitance_value.clone();
        let resistance_value = resistance_value.clone();
        let frequency_value = frequency_value.clone();
        let q_factor_value = q_factor_value.clone();
        let r1_value = r1_value.clone();
        let r2_value = r2_value.clone();
        let c1_value = c1_value.clone();
        let c2_value = c2_value.clone();

        move |_| {
            let capacitance_value =
                if let SeriesInput(series, min, max, min_fallback, max_fallback) =
                    &*capacitance_value
                {
                    let (min, max) = if let (Some(min), Some(max)) = (min, max) {
                        (min, max)
                    } else if let (None, Some(max)) = (min, max) {
                        (min_fallback, max)
                    } else if let (Some(min), None) = (min, max) {
                        (min, max_fallback)
                    } else {
                        (min_fallback, max_fallback)
                    };

                    ComponentValue::Series(series.clone(), *min, *max)
                } else {
                    unreachable!()
                };

            let resistance_value =
                if let SeriesInput(series, min, max, min_fallback, max_fallback) =
                    &*resistance_value
                {
                    let (min, max) = if let (Some(min), Some(max)) = (min, max) {
                        (min, max)
                    } else if let (None, Some(max)) = (min, max) {
                        (min_fallback, max)
                    } else if let (Some(min), None) = (min, max) {
                        (min, max_fallback)
                    } else {
                        (min_fallback, max_fallback)
                    };

                    ComponentValue::Series(series.clone(), *min, *max)
                } else {
                    unreachable!()
                };

            let frequency_value =
                if let ToleranceInput(target, tolerance, fallback) = &*frequency_value {
                    let target = if let Some(target) = target {
                        target
                    } else {
                        fallback
                    };

                    ToleranceValue(*target, *tolerance)
                } else {
                    unreachable!()
                };

            let q_factor_value =
                if let ToleranceInput(target, tolerance, fallback) = &*q_factor_value {
                    let target = if let Some(target) = target {
                        target
                    } else {
                        fallback
                    };

                    ToleranceValue(*target, *tolerance)
                } else {
                    unreachable!()
                };

            let r1_value = if let NumberInput(target, fallback) = &*r1_value {
                match (target, fallback) {
                    (Some(target), _) => ComponentValue::Given(*target),
                    (None, Some(fallback)) => ComponentValue::Given(*fallback),
                    (None, None) => resistance_value.clone(),
                }
            } else {
                unreachable!()
            };

            let r2_value = if let NumberInput(target, fallback) = &*r2_value {
                match (target, fallback) {
                    (Some(target), _) => ComponentValue::Given(*target),
                    (None, Some(fallback)) => ComponentValue::Given(*fallback),
                    (None, None) => resistance_value,
                }
            } else {
                unreachable!()
            };

            let c1_value = if let NumberInput(target, fallback) = &*c1_value {
                match (target, fallback) {
                    (Some(target), _) => ComponentValue::Given(*target),
                    (None, Some(fallback)) => ComponentValue::Given(*fallback),
                    (None, None) => capacitance_value.clone(),
                }
            } else {
                unreachable!()
            };

            let c2_value = if let NumberInput(target, fallback) = &*c2_value {
                match (target, fallback) {
                    (Some(target), _) => ComponentValue::Given(*target),
                    (None, Some(fallback)) => ComponentValue::Given(*fallback),
                    (None, None) => capacitance_value,
                }
            } else {
                unreachable!()
            };

            results.set(Some(calculate(
                frequency_value,
                q_factor_value,
                r1_value,
                r2_value,
                c1_value,
                c2_value,
            )));
        }
    };

    html! {
        <form>
            <h1>{"gyrator calculator"}</h1>
            <p>{"this calculator aids in the design of gyrator based filters by selecting appropriate values for the desired q and frequency"}</p>

            <h2>{"component ranges"}</h2>
            <div class="fieldset">
                <Input id="capacitance" name="capacitance" value={capacitance_value.clone()} />
                <Input id="resistance" name="resistance" value={resistance_value.clone()} />
            </div>

            <h2>{"gyrator values"}</h2>
            <p>{"avoid setting too many optional fields as it will limit the results significantly"}</p>
            <div class="fieldset">
                <Input id="frequency" name="frequency" value={frequency_value} />
                <Input id="q-factor" name="q factor" value={q_factor_value} />
                <Input id="r1" name="r1" note="the value of r1 sets the gain of the gyrator" value={r1_value.clone()} />
                <Input id="r2" name="r2" note="use a specific r2 value" value={r2_value.clone()} />
                <Input id="c1" name="c1" note="use a specific c1 value" value={c1_value.clone()} />
                <Input id="c2" name="c2" note="use a specific c2 value" value={c2_value.clone()} />
            </div>

            <button type="button" {onclick}>{"calculate"}</button>

            <h2>{"results"}</h2>

            <table>
                <thead>
                    <tr>
                        <th class="frequency">{"frequency"}</th>
                        <th class="q-factor">{"q factor"}</th>
                        <th class="inductance">{"inductance"}</th>
                        <th class="r1-resistance">{"r1"}</th>
                        <th class="r2-resistance">{"r2"}</th>
                        <th class="c1-capacitance">{"c1"}</th>
                        <th class="c2-capacitance">{"c2"}</th>
                    </tr>
                </thead>
                <tbody>
                    {
                        if let Some(results) = &*results {
                            if results.is_empty() {
                                html!{
                                    <tr>
                                        <td class="msg" colspan="7">{"no results found"}</td>
                                    </tr>
                                }
                            } else {
                                results.iter().map(|result| {
                                    html!{<tr>
                                        <td class="frequency">{format_units(result.frequency())}</td>
                                        <td class="q-factor">{format_units(result.q_factor())}</td>
                                        <td class="inductance">{format_units(result.inductance())}</td>
                                        <td class="r1-resistance">{format_units(result.r1_resistance())}</td>
                                        <td class="r2-resistance">{format_units(result.r2_resistance())}</td>
                                        <td class="c1-capacitance">{format_units(result.c1_capacitance())}</td>
                                        <td class="c2-capacitance">{format_units(result.c1_capacitance())}</td>
                                    </tr>}
                                }).collect::<Html>()
                            }
                        } else {
                            html!{
                                <tr>
                                    <td class="msg" colspan="7">{"click \"calculate\" above to begin"}</td>
                                </tr>
                            }
                        }
                    }
                </tbody>
            </table>
        </form>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
