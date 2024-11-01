use super::aside_component::AsideComponentInfo;
use super::aside_props::AsideProps;
use crate::utils::{get_component_info, ComponentInfo};
use crate::SelectedComponentId;
use leptos::{ev, prelude::*};

#[component]
pub fn Aside(aside_width: RwSignal<i32>) -> impl IntoView {
    let is_mouse_move = RwSignal::new(false);
    let on_mouse_down = move |_| {
        is_mouse_move.set(true);
    };
    let on_mouse_up = window_event_listener(ev::mouseup, move |_| {
        is_mouse_move.set(false);
    });
    on_cleanup(move || on_mouse_up.remove());
    let on_mouse_move = window_event_listener(ev::mousemove, move |ev| {
        if is_mouse_move.get() {
            let ev_x = ev.x();
            let client_width = document().body().unwrap().client_width();
            if ev_x <= 320 {
                aside_width.set(client_width - 320);
            } else if ev_x >= client_width - 320 {
                aside_width.set(320);
            } else {
                aside_width.set(client_width - ev_x);
            }
        }
    });
    on_cleanup(move || on_mouse_move.remove());
    let selected_comp_id = expect_context::<RwSignal<Option<SelectedComponentId>>>();

    let info = Memo::new(move |_| {
        if let Some(comp_id) = selected_comp_id.get() {
            get_component_info(&comp_id.0)
        } else {
            None
        }
    });

    let style = Memo::new(move |_| {
        if selected_comp_id.get().is_none() {
            String::from("display: none;")
        } else {
            format!("width: {}px", aside_width.get())
        }
    });
    view! {
        <aside
            class="relative flex flex-col font-size-14px box-border border-l-1 border-l-#ddd @dark:border-l-#333"
            style=move || style.get()
        >
            <div
                class="absolute w-6px top-0 bottom-0 left--3px cursor-ew-resize"
                on:mousedown=on_mouse_down
            ></div>
            <div class="h-26px line-height-26px px-8px b-b b-b-solid b-b-#ddd flex flex-justify-between">
                {move || {
                    if let Some(info) = info.get() {
                        let ComponentInfo { name, location } = info;
                        let location_title = format!(
                            "Open {} in vscode",
                            location.clone().unwrap_or(String::new()),
                        );
                        let open_editor = move |_| {
                            if let Some(location) = location.clone() {
                                let url = format!("vscode://file/{}", location);
                                _ = window().location().assign(&url);
                            }
                        };
                        view! {
                            <span class="color-#2080f0">"<"{name}">"</span>
                            <span
                                class="flex hover-bg-#f3f3f5 cursor-pointer px-4px"
                                title=location_title
                                on:click=open_editor
                            >
                                <svg
                                    class="w-18px"
                                    xmlns="http://www.w3.org/2000/svg"
                                    xmlns:xlink="http://www.w3.org/1999/xlink"
                                    viewBox="0 0 512 512"
                                >
                                    <circle cx="256" cy="256" r="26" fill="currentColor"></circle>
                                    <circle cx="346" cy="256" r="26" fill="currentColor"></circle>
                                    <circle cx="166" cy="256" r="26" fill="currentColor"></circle>
                                    <path
                                        fill="none"
                                        stroke="currentColor"
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        stroke-width="32"
                                        d="M160 368L32 256l128-112"
                                    ></path>
                                    <path
                                        fill="none"
                                        stroke="currentColor"
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        stroke-width="32"
                                        d="M352 368l128-112l-128-112"
                                    ></path>
                                </svg>
                            </span>
                        }
                            .into()
                    } else {
                        None
                    }
                }}
            </div>
            <div class="flex-1 p-8px pt-2px overflow-auto">
                <AsideProps />
                <AsideComponentInfo />
            </div>
        </aside>
    }
}
