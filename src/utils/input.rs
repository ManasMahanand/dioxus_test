use dioxus::prelude::*;
use tailwind_fuse::tw_merge;

#[derive(PartialEq, Clone, Props)]
pub struct InputProps {
    #[props(default = String::from("text"))]
    r#type: String,
    #[props(default)]
    placeholder: String,
    #[props(default)]
    value: String,
    #[props(default)]
    oninput: EventHandler<Event<FormData>>,
    #[props(default)]
    onkeydown: EventHandler<Event<KeyboardData>>,
    #[props(default)]
    class: String
}

#[component]
pub fn Input(props: InputProps) -> Element {
    rsx! {
        input {
            r#type: props.r#type,
            placeholder: props.placeholder,
            value: props.value,
            oninput: props.oninput,
            onkeydown: props.onkeydown,
            class: tw_merge!("px-4 py-4 border-b-zinc-600 border-b-2 focus:border-b-zinc-300 active:border-b-zinc-300 focus:outline-none mb-4", props.class)
        }
    }
}