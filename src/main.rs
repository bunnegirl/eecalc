use gyrator_calculator::series::*;
use gyrator_calculator::units::*;
use gyrator_calculator::*;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
struct InputProps {
    id: &'static str,
    name: &'static str,
    note: Option<&'static str>,
    value: UseStateHandle<Input>,
}

#[function_component(InputField)]
fn input(
    InputProps {
        id,
        name,
        note,
        value,
    }: &InputProps,
) -> Html {
    let value = value.clone();

    let on_series_change = {
        let state = value.clone();

        move |event: Event| {
            let element: EventTarget = event
                .target()
                .expect("Event should have a target when dispatched");

            if let InputWithSeries(_, min, max, min_fallback, max_fallback) = &*state {
                let series = Series::from(element.unchecked_into::<HtmlInputElement>().value());

                state.set(InputWithSeries(
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

            if let InputWithSeries(series, _, max, min_fallback, max_fallback) = &*state {
                let value = &element.unchecked_into::<HtmlInputElement>().value();

                let min = if value.trim().is_empty() {
                    None
                } else {
                    Some(parse_units(value))
                };

                state.set(InputWithSeries(
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

            if let InputWithSeries(series, min, _, min_fallback, max_fallback) = &*state {
                let value = &element.unchecked_into::<HtmlInputElement>().value();

                let max = if value.trim().is_empty() {
                    None
                } else {
                    Some(parse_units(value))
                };

                state.set(InputWithSeries(
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

            if let InputWithExact(_, fallback) = &*state {
                if value.trim().is_empty() {
                    state.set(InputWithExact(None, *fallback));
                } else {
                    state.set(InputWithExact(Some(parse_units(value)), *fallback));
                }
            } else if let InputWithTolerance(_, tolerance, fallback) = &*state {
                if value.trim().is_empty() {
                    state.set(InputWithTolerance(None, *tolerance, *fallback));
                } else {
                    state.set(InputWithTolerance(
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

            if let InputWithTolerance(target, _, fallback) = &*state {
                let value = &element.unchecked_into::<HtmlInputElement>().value();

                state.set(InputWithTolerance(*target, parse_units(value), *fallback));
            }
        }
    };

    let format_id = |id: &str, suffix: &str| format!("{}-{}", id, suffix);

    let format_value = |value: &Option<f64>| {
        if let Some(value) = value {
            format_units(*value)
        } else {
            "".into()
        }
    };

    let format_fallback = |fallback: &Option<f64>| {
        if let Some(value) = fallback {
            format_units(*value)
        } else {
            "optional".into()
        }
    };

    match &*value {
        InputWithSeries(series, min, max, min_fallback, max_fallback) => html! {
            <>
                <div class="field">
                    <label for={format_id(id, "series")}>{format!("{} series", name)}</label>
                    <select id={format_id(id, "series")} onchange={on_series_change}>
                        {
                            SERIES_OPTIONS.iter().map(|item_series| html! {
                                <option selected={series == item_series}>{item_series.as_str()}</option>
                            }).collect::<Html>()
                        }
                    </select>
                </div>
                <div class="field">
                    <label for={format_id(id, "range")}>{format!("{} range", name)}</label>
                    <input
                        id={format_id(id, "range")}
                        placeholder={format_units(*min_fallback)}
                        value={format_value(min)}
                        onchange={on_min_change}
                    />
                    <input
                        placeholder={format_units(*max_fallback)}
                        value={format_value(max)}
                        onchange={on_max_change}
                    />
                </div>
            </>
        },
        InputWithExact(target, fallback) => html! {
            <div class="field">
                <label for={format_id(id, "target")}>{{format!("{} target", name)}}</label>
                <input
                    id={format_id(id, "target")}
                    placeholder={format_fallback(fallback)}
                    value={format_value(target)}
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
        },
        InputWithTolerance(target, tolerance, fallback) => html! {
            <div class="field">
                <label for={format_id(id, "target")}>{format!("{} target", name)}</label>
                <input
                    id={format_id(id, "target")}
                    placeholder={format_units(*fallback)}
                    value={format_value(target)}
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
        },
    }
}

#[derive(Clone, PartialEq)]
enum SortBy {
    Frequency,
    QFactor,
    Inductance,
    R1Resistance,
    R2Resistance,
    C1Capacitance,
    C2Capacitance,
}

#[derive(PartialEq)]
enum SortOrder {
    Ascending,
    Descending,
}

#[derive(Properties, PartialEq)]
struct ResultsColumnProp {
    class: &'static str,
    name: &'static str,
    column: SortBy,
    sort_by: UseStateHandle<SortBy>,
    sort_order: UseStateHandle<SortOrder>,
    on_sort: Callback<SortBy>,
}

#[function_component(ResultsColumn)]
fn results_column(
    ResultsColumnProp {
        class,
        name,
        column,
        sort_by,
        sort_order,
        on_sort,
    }: &ResultsColumnProp,
) -> Html {
    let sort_by = sort_by.clone();
    let sort_order = sort_order.clone();

    let onclick = |column: SortBy| on_sort.reform(move |_| column.clone());

    html! {
        <th
            class={*class}
            onclick={onclick(column.clone())}>
            {name}
            {
                if *sort_by == *column {
                    match *sort_order {
                        SortOrder::Ascending => html!{<span class="sort">{"⏷"}</span>},
                        SortOrder::Descending => html!{<span class="sort">{"⏶"}</span>},
                    }
                } else {
                    html!{}
                }
            }
        </th>
    }
}

#[derive(Properties, PartialEq)]
struct ResultsProps {
    results: UseStateHandle<Option<Vec<Selection>>>,
}

#[function_component(Results)]
fn results(ResultsProps { results }: &ResultsProps) -> Html {
    let results = results.clone();
    let sort_by = use_state(|| SortBy::Frequency);
    let sort_order = use_state(|| SortOrder::Ascending);

    let set_sort = {
        let sort_by = sort_by.clone();
        let sort_order = sort_order.clone();

        Callback::from(move |column: SortBy| {
            if *sort_by == column {
                sort_order.set(if *sort_order == SortOrder::Ascending {
                    SortOrder::Descending
                } else {
                    SortOrder::Ascending
                })
            } else {
                sort_by.set(column)
            }
        })
    };

    let sort = |a: &Selection, b: &Selection| {
        use SortBy::*;
        use SortOrder::*;

        let (a, b) = match *sort_by {
            Frequency => (a.frequency(), b.frequency()),
            QFactor => (a.q_factor(), b.q_factor()),
            Inductance => (a.inductance(), b.inductance()),
            R1Resistance => (a.r1_resistance(), b.r1_resistance()),
            R2Resistance => (a.r2_resistance(), b.r2_resistance()),
            C1Capacitance => (a.c1_capacitance(), b.c1_capacitance()),
            C2Capacitance => (a.c2_capacitance(), b.c2_capacitance()),
        };

        match *sort_order {
            Ascending => a.partial_cmp(&b).unwrap(),
            Descending => b.partial_cmp(&a).unwrap(),
        }
    };

    let results = if let Some(mut results) = (*results).clone() {
        results.sort_by(sort);

        Some(results)
    } else {
        None
    };

    if let Some(results) = results {
        if results.is_empty() {
            html! {<p>{"no results found"}</p>}
        } else {
            html! {
                <>
                <h2>{format!("{} results", results.len())}</h2>
                <table>
                    <thead>
                        <tr>
                            <ResultsColumn
                                class="frequency"
                                name="frequency"
                                column={SortBy::Frequency}
                                sort_by={sort_by.clone()}
                                sort_order={sort_order.clone()}
                                on_sort={set_sort.clone()} />
                            <ResultsColumn
                                class="q-factor"
                                name="q factor"
                                column={SortBy::QFactor}
                                sort_by={sort_by.clone()}
                                sort_order={sort_order.clone()}
                                on_sort={set_sort.clone()} />
                            <ResultsColumn
                                class="inductance"
                                name="inductance"
                                column={SortBy::Inductance}
                                sort_by={sort_by.clone()}
                                sort_order={sort_order.clone()}
                                on_sort={set_sort.clone()} />
                            <ResultsColumn
                                class="r1-resistance"
                                name="r1"
                                column={SortBy::R1Resistance}
                                sort_by={sort_by.clone()}
                                sort_order={sort_order.clone()}
                                on_sort={set_sort.clone()} />
                            <ResultsColumn
                                class="r2-resistance"
                                name="r2"
                                column={SortBy::R2Resistance}
                                sort_by={sort_by.clone()}
                                sort_order={sort_order.clone()}
                                on_sort={set_sort.clone()} />
                            <ResultsColumn
                                class="c1-capacitance"
                                name="c1"
                                column={SortBy::C1Capacitance}
                                sort_by={sort_by.clone()}
                                sort_order={sort_order.clone()}
                                on_sort={set_sort.clone()} />
                            <ResultsColumn
                                class="c2-capacitance"
                                name="c2"
                                column={SortBy::C2Capacitance}
                                sort_by={sort_by.clone()}
                                sort_order={sort_order.clone()}
                                on_sort={set_sort.clone()} />
                        </tr>
                    </thead>
                    <tbody>
                        {
                            results.iter().map(|result| {
                                html!{<tr>
                                    <td class="frequency">{format_units(result.frequency())}</td>
                                    <td class="q-factor">{format_units(result.q_factor())}</td>
                                    <td class="inductance">{format_units(result.inductance())}</td>
                                    <td class="r1-resistance">{format_units(result.r1_resistance())}</td>
                                    <td class="r2-resistance">{format_units(result.r2_resistance())}</td>
                                    <td class="c1-capacitance">{format_units(result.c1_capacitance())}</td>
                                    <td class="c2-capacitance">{format_units(result.c2_capacitance())}</td>
                                </tr>}
                            }).collect::<Html>()
                        }
                    </tbody>
                </table>
                </>
            }
        }
    } else {
        html! {
            <p>{"click \"calculate\" above to begin"}</p>
        }
    }
}

#[function_component]
fn App() -> Html {
    let results = use_state(|| None);
    let capacitance_value = use_state(|| {
        InputWithSeries(
            Series::E6,
            None,
            None,
            parse_units("1n"),
            parse_units("100u"),
        )
    });
    let resistance_value = use_state(|| {
        InputWithSeries(
            Series::E24,
            None,
            None,
            parse_units("1k"),
            parse_units("100k"),
        )
    });
    let frequency_value = use_state(|| InputWithTolerance(None, 0.1, 100.0));
    let q_factor_value = use_state(|| InputWithTolerance(None, 0.2, 4.0));
    let r1_value = use_state(|| InputWithExact(None, Some(470.0)));
    let r2_value = use_state(|| InputWithExact(None, None));
    let c1_value = use_state(|| InputWithExact(None, None));
    let c2_value = use_state(|| InputWithExact(None, None));

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
            let capacitance_value = capacitance_value.to_arg().unwrap();
            let resistance_value = resistance_value.to_arg().unwrap();

            results.set(Some(calculate(
                frequency_value.to_arg().unwrap(),
                q_factor_value.to_arg().unwrap(),
                r1_value.to_arg().unwrap_or(resistance_value.clone()),
                r2_value.to_arg().unwrap_or(resistance_value),
                c1_value.to_arg().unwrap_or(capacitance_value.clone()),
                c2_value.to_arg().unwrap_or(capacitance_value),
            )));
        }
    };

    html! {
        <form>
            <h1>{"gyrator calculator"}</h1>
            <p>{"this calculator aids in the design of gyrator based filters by selecting appropriate values for the desired q and frequency"}</p>

            <h2>{"component ranges"}</h2>
            <div class="fieldset">
                <InputField id="capacitance" name="capacitance" value={capacitance_value} />
                <InputField id="resistance" name="resistance" value={resistance_value} />
            </div>

            <h2>{"gyrator values"}</h2>
            <p>{"avoid setting too many optional fields as it will limit the results significantly"}</p>
            <div class="fieldset">
                <InputField id="frequency" name="frequency" value={frequency_value} />
                <InputField id="q-factor" name="q factor" value={q_factor_value} />
                <InputField id="r1" name="r1" note="the value of r1 sets the gain of the gyrator" value={r1_value} />
                <InputField id="r2" name="r2" note="use a specific r2 value" value={r2_value} />
                <InputField id="c1" name="c1" note="use a specific c1 value" value={c1_value} />
                <InputField id="c2" name="c2" note="use a specific c2 value" value={c2_value} />
            </div>

            <button type="button" {onclick}>{"calculate"}</button>

            <Results results={results} />
        </form>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
