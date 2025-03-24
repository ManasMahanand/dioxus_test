use dioxus::prelude::*;
use tailwind_fuse::tw_merge;

#[derive(PartialEq, Clone, Props)]
pub struct TitleProps {
    #[props(default)]
    class: String,
    children: Element
}

#[component]
pub fn Title(props: TitleProps) -> Element {
    rsx! {
        h1 {
            class: tw_merge!("py-2 text-lg font-black", props.class),
            {props.children}
        }
    }
}