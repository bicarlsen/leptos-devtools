mod components;
mod message;
mod utils;

use crate::{
    message::on_message,
    utils::{gen_nodes, Node},
};
use components::{Aside, ComponentNode, Crumb};
use leptos::prelude::*;
use std::{collections::HashSet, num::NonZeroU64};

fn main() {
    mount_to_body(App);
}

#[derive(Clone, PartialEq)]
pub struct SelectedComponentId(NonZeroU64);

#[component]
fn App() -> impl IntoView {
    let message_component_update = Trigger::new();
    let selected_component_id = RwSignal::<Option<SelectedComponentId>>::new(None);
    let expand_component = RwSignal::new(HashSet::<NonZeroU64>::new());
    let aside_width = RwSignal::new(320);
    provide_context(selected_component_id);
    provide_context(expand_component);
    on_message(message_component_update);

    let nodes = Memo::new(move |_| {
        message_component_update.track();
        gen_nodes(None, 0)
    });

    let nodes_filter = Memo::new(move |_| {
        nodes.with(|nodes| {
            let mut nodes_filter = vec![];
            let mut level_filter = None;
            for node in nodes {
                if let Some(level) = level_filter {
                    if level < node.level {
                        continue;
                    } else {
                        level_filter = None;
                    }
                }
                if !expand_component.with(|ec| ec.contains(&node.id)) {
                    level_filter = Some(node.level);
                }
                nodes_filter.push(node.clone());
            }
            nodes_filter
        })
    });
    view! {
        <section class="flex h-screen">
            <main class="flex-1 flex flex-col">
                <div class="flex-1 p-8px overflow-auto">
                    <For
                        each=move || nodes_filter.get()
                        key=|node| node.id.clone()
                        children=|node| {
                            let Node { id, name, level } = node;
                            view! {
                                <ComponentNode id name level/>
                            }
                        }
                    />
                </div>
                <Crumb aside_width/>
            </main>
            <Aside aside_width/>
        </section>
    }
}
