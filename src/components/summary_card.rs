use dioxus::prelude::*;

use crate::types::PingResult;

#[component]
pub fn SummaryArea(results: Signal<Vec<PingResult>>) -> Element {
    let results_read = results.read();
    let sent = results_read.len();
    let success_count = results_read.iter().filter(|r| r.success).count();
    let loss_rate = if sent > 0 {
        ((sent - success_count) as f64 / sent as f64 * 100.0).round() as u32
    } else {
        0
    };
    let avg_text = {
        let times: Vec<f64> = results_read.iter().filter_map(|r| r.time_ms).collect();
        if times.is_empty() {
            "—".to_string()
        } else {
            format!("{:.1} ms", times.iter().sum::<f64>() / times.len() as f64)
        }
    };

    rsx! {
        div { class: "px-6 pb-5 flex-shrink-0",
            div { class: "grid grid-cols-4 gap-2",
                SummaryCard {
                    label: "SENT".to_string(),
                    value: format!("{sent}"),
                    accent: "#94a3b8".to_string(),
                }
                SummaryCard {
                    label: "OK".to_string(),
                    value: format!("{success_count}"),
                    accent: "#34d399".to_string(),
                }
                SummaryCard {
                    label: "LOSS".to_string(),
                    value: format!("{loss_rate}%"),
                    accent: if loss_rate > 0 { "#f87171".to_string() } else { "#94a3b8".to_string() },
                }
                SummaryCard {
                    label: "AVG".to_string(),
                    value: avg_text,
                    accent: "#60a5fa".to_string(),
                }
            }
        }
    }
}

#[component]
fn SummaryCard(label: String, value: String, accent: String) -> Element {
    rsx! {
        div {
            class: "rounded-md px-3 py-2",
            style: "background: #0d1219; border: 1px solid #1e2530;",
            p {
                class: "text-xs font-medium tracking-wide",
                style: "color: #64748b;",
                "{label}"
            }
            p {
                class: "text-base font-mono font-semibold mt-1",
                style: "color: {accent};",
                "{value}"
            }
        }
    }
}
