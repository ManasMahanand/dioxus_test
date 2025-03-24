use dioxus::prelude::*;
use dioxus::prelude::Key::{Enter, Escape};
use crate::utils::checkbox::Checkbox;
use crate::utils::title::Title;

mod utils;

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[derive(Clone)]
struct Task {
    id: String,
    task: Signal<String>,
    checked: Signal<bool>
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {

    let mut editing = use_signal(|| false);
    let mut curr_editing = use_signal(|| "".to_string());

    let mut id = use_signal(|| 0);
    let mut new_task = use_signal(|| Task {
        id: id.to_string(),
        task: Signal::new("".to_string()),
        checked: Signal::new(false)
    });

    let mut tasks = use_signal::<Vec<Task>>(|| vec![]);

    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        main {
            onkeydown: move |e| {
                if editing.cloned() && (e.data.key() == Escape || e.data.key() == Enter) {
                    editing.set(false);
                    curr_editing.set("".to_string());
                }
            },
            class: "bg-zinc-800 h-screen text-zinc-100",
            div {
                class: "container mx-auto px-4 flex flex-col gap-2",
                Title {
                    "TODO"
                },
                input {
                    type: "text",
                    placeholder: "Add a task",
                    value: new_task.cloned().task.cloned(),
                    oninput: move |e| {
                        new_task.cloned().task.set(e.value());
                    },
                    onkeydown: move |e| {
                        if e.data.key() == Enter {
                            tasks.push(new_task.cloned());
                            id += 1;
                            new_task.set(Task {
                                id: id.to_string(),
                                task: Signal::new("".to_string()),
                                checked: Signal::new(false)
                            })
                        }
                    },
                    class: "px-4 py-4 border-b-zinc-600 border-b-2 focus:border-b-zinc-300 active:border-b-zinc-300 focus:outline-none mb-4"
                },
                for mut task in tasks.cloned() {
                    if editing.cloned() && curr_editing.cloned() == task.id {
                        input {
                            type: "text",
                            value: task.task.cloned(),
                            oninput: move |e| {
                                task.task.set(e.value())
                            },
                            class: "px-2 py-4 border-b-zinc-600 border-b-2 focus:border-b-zinc-300 active:border-b-zinc-300 focus:outline-none"
                        }
                    } else {
                        Checkbox {
                            id: task.id.clone(),
                            checked: task.checked.cloned(),
                            oninput: move |_| task.checked.set(!task.checked.cloned()),
                            onclick: move |_| {
                                curr_editing.set(task.id.clone());
                                editing.set(true);
                            },
                            class: "cursor-pointer hover:border-dashed hover:border-zinc-300 hover:border-2 px-2 hover:px-2 hover:-mx-0.5 py-4 hover:-my-0.5",
                            "{task.task}"
                        }
                    }
                }
            }
        }
    }
}