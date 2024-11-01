use crate::SelectedComponentId;
use leptos::prelude::*;
use std::{collections::HashSet, num::NonZeroU64};

#[component]
pub fn ComponentNode(id: NonZeroU64, name: String, level: u64) -> impl IntoView {
    let selected_comp_id = expect_context::<RwSignal<Option<SelectedComponentId>>>();
    let expand_component = expect_context::<RwSignal<HashSet<NonZeroU64>>>();
    expand_component.with_untracked(|ec| ec.contains(&id));

    let arrow_click = move |_| {
        expand_component.update(|ec| {
            if ec.contains(&id) {
                ec.remove(&id);
            } else {
                ec.insert(id);
            }
        });
    };
    let selected = Memo::new(move |_| selected_comp_id.get().map_or(false, |v| v.0 == id));
    view! {
        <div
            class="text-sm leading-relaxed py-0 px-1 rounded-sm hover:bg-#ddd @dark:hover:bg-#444"
            class=(["bg-#ccc4", "@dark:bg-#333"], selected)
            on:click=move |_| selected_comp_id.set(Some(SelectedComponentId(id)))
        >
            <Indent level />
            <span class="arrow inline-block w-3.5 text-center" on:click=arrow_click>
                <span
                    class="arrow-right inline-block w-0 h-0 border-y-4 border-l-6px border-transparent"
                    class=(
                        ["arrow-bottom", "rotate-90", "transition", "duration-300"],
                        move || expand_component.with(|ec| ec.contains(&id)),
                    )
                ></span>
            </span>
            <span class="color-#2060c0 @dark:color-#60a0ff">"<" {name} ">"</span>
            {
                #[cfg(feature = "development")]
                view! { <span class="pl-12px color-#777 @dark:color-#bbb">"id=" {id}</span> }
            }
        </div>
    }
}

#[component]
fn Indent(level: u64) -> impl IntoView {
    view! {
        <span>
            <For each=move || 0..level key=|num| num.clone() let:_>
                <span class="inline-block w-16px"></span>
            </For>
        </span>
    }
}
