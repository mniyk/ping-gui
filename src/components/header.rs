use dioxus::prelude::*;

#[component]
pub fn Header(running: bool) -> Element {
    rsx! {
        div {
            class: "px-6 pt-5 pb-4 flex-shrink-0",
            style: "border-bottom: 1px solid #232b38;",
            div { class: "flex items-center gap-2",
                div {
                    class: "w-2 h-2 rounded-full",
                    style: if running { "background: #fbbf24; box-shadow: 0 0 8px #fbbf24;" } else { "background: #34d399; box-shadow: 0 0 8px #34d399;" },
                }
                h1 {
                    class: "text-lg font-semibold tracking-tight",
                    style: "color: #f1f5f9;",
                    "ping-gui"
                }
            }
            p { class: "text-xs mt-1", style: "color: #64748b;",
                "A simple ping GUI · Rust + Dioxus"
            }
        }
    }
}
