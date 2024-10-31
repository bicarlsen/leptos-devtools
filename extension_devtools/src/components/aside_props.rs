use crate::{utils::get_component_props, SelectedComponentId};
use leptos::{either::Either, prelude::*};
use serde_json::Value;

#[component]
pub fn AsideProps() -> impl IntoView {
    let selected_comp_id = expect_context::<RwSignal<Option<SelectedComponentId>>>();
    let props = Memo::new(move |_| {
        if let Some(comp_id) = selected_comp_id.get() {
            get_component_props(&comp_id.0)
        } else {
            vec![]
        }
    });

    move || {
        let props = props.get();
        if props.is_empty() {
            None
        } else {
            view! {
                <div class="my-6px">"props"</div>
                {
                    props.into_iter().map(|prop| {
                        view! {
                            <div class="ml-14px min-h-20px line-height-20px">
                                <span class="color-#8128e8">{prop.name}</span>
                                <span class="mr-0.5em">":"</span>
                                {
                                    if let Some(err) = prop.error {
                                        Some(Either::Left(view! {
                                                <span
                                                    title=err
                                                    class="prop-value-tag prop-value-tag--error"
                                                >
                                                    "Error"
                                                </span>
                                        }))
                                    } else if let Some(value) = prop.value {
                                        Some(Either::Right(view! {
                                                <Value value/>
                                        }))
                                    } else {
                                        None
                                    }
                                }
                            </div>
                        }
                    })
                    .collect::<Vec<_>>()
                }
            }
            .into()
        }
    }
}

#[component]
fn Value(value: Value) -> impl IntoView {
    match value {
        Value::Null => view! {
            <span>"null"</span>
        }
        .into_any(),
        Value::Bool(value) => view! {
            <span class="color-#03c">{value}</span>
        }
        .into_any(),
        Value::Number(value) => view! {
            <span class="color-#03c">{value.to_string()}</span>
        }
        .into_any(),
        Value::String(value) => view! {
            <span class="white-space-nowrap">{format!(r#""{value}""#)}</span>
        }
        .into_any(),
        Value::Array(arr) => view! {
            <div class="ml-14px">
                {
                    arr.into_iter().enumerate().map(|(index, value)| {
                        view! {
                            <div class="min-h-20px line-height-20px">
                                <span class="color-#8128e8">{index}</span>
                                <span class="mr-0.5em">":"</span>
                                <Value value/>
                            </div>
                        }
                    })
                    .collect::<Vec<_>>()
                }
            </div>
        }
        .into_any(),
        Value::Object(obj) => view! {
            <div class="ml-14px">
                {
                    obj.into_iter().map(|(key, value)| {
                        view! {
                            <div class="min-h-20px line-height-20px">
                                <span class="color-#8128e8">{format!(r#""{key}""#)}</span>
                                <span class="mr-0.5em">":"</span>
                                <Value value/>
                            </div>
                        }
                    })
                    .collect::<Vec<_>>()
                }
            </div>
        }
        .into_any(),
    }
}
