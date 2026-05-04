use dioxus::prelude::*;

use crate::types::PingResult;

#[component]
pub fn OutputList(results: Signal<Vec<PingResult>>, count: u32) -> Element {
    let results_read = results.read();
    let sent = results_read.len();
    let is_empty = results_read.is_empty();

    rsx! {
        // ラベル
        div { class: "px-6 flex items-center justify-between flex-shrink-0",
            p {
                class: "text-xs font-medium uppercase tracking-wide",
                style: "color: #94a3b8;",
                "Output"
            }
            p { class: "text-xs font-mono", style: "color: #475569;", "{sent} / {count}" }
        }
        // リスト本体
        div {
            class: "mx-6 mt-2 mb-4 font-mono text-xs rounded-md overflow-y-auto",
            style: "height: 200px; background: #0a0e14; border: 1px solid #1e2530; padding: 12px;",
            if is_empty {
                div {
                    class: "h-full flex items-center justify-center",
                    style: "color: #475569;",
                    "$ awaiting input..."
                }
            } else {
                for r in results_read.iter() {
                    {
                        let seq = r.seq;
                        let host = r.host.clone();
                        let success = r.success;
                        let time_ms = r.time_ms.unwrap_or(0.0);
                        let error = r.error.clone().unwrap_or_default();
                        rsx! {
                            div { class: "flex justify-between py-1", key: "{seq}",
                                span { style: "color: #94a3b8;",
                                    span { style: "color: #475569;", "[{seq:02}] " }
                                    "{host}"
                                }
                                if success {
                                    span { style: "color: #34d399;", "✓ {time_ms:.1} ms" }
                                } else {
                                    span { style: "color: #f87171;", title: "{error}", "✗ {error}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
