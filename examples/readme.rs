//! Example: README.md showcase
//!
//! The example from the README.md.

use dioxus::prelude::*;

fn main() {
    dioxus::desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    let (count, set_count) = use_state(&cx, || 0);

    cx.render(rsx! {
        div {
            h1 { "High-Five counter: {count}" }
            button { onclick: move |_| set_count(count + 1), "Up high!" }
            button { onclick: move |_| set_count(count - 1), "Down low!" }
        }
    })
}
