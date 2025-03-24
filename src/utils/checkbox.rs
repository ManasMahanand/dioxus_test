use dioxus::prelude::*;
use tailwind_fuse::merge::tw_merge;
use tailwind_fuse::tw_merge;

#[derive(PartialEq, Clone, Props)]
pub struct CheckboxProps {
    id: String,
    checked: bool,
    oninput: EventHandler<Event<FormData>>,
    onclick: Option<EventHandler<Event<MouseData>>>,
    #[props(default)]
    class: String,
    children: Element
}

#[component]
pub fn Checkbox(props: CheckboxProps) -> Element {
    rsx! {
        div {
            class: tw_merge!("flex items-center gap-4", props.class),
            input {
                id: "{props.id}",
                type: "checkbox",
                checked: {props.checked},
                oninput: props.oninput,
                class: "cursor-pointer appearance-none relative w-5 h-5 border checked:after:absolute checked:after:bg-zinc-100 checked:after:content:'' checked:after:w-3 checked:after:h-3 checked:after:left-1/2 checked:after:top-1/2 checked:after:-translate-x-1/2 checked:after:-translate-y-1/2"
            }
            label {
                for: "{props.id}",
                onclick: props.onclick.unwrap_or_default(),
                class: "cursor-pointer",
                {props.children}
            }
        }
    }
}