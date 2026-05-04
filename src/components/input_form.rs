use dioxus::prelude::*;

#[component]
pub fn InputForm(
    host: Signal<String>,
    count: Signal<u32>,
    running: bool,
    on_start: EventHandler<()>,
) -> Element {
    let host_value = host();
    let count_value = count();

    rsx! {
        div { class: "px-6 pt-4 flex-shrink-0",
            div {
                class: "grid gap-3 mb-3",
                style: "grid-template-columns: 1fr 100px;",
                // ホスト名/IP
                div {
                    label {
                        class: "block text-xs font-medium mb-1.5 uppercase tracking-wide",
                        style: "color: #94a3b8;",
                        "Host / IP"
                    }
                    input {
                        r#type: "text",
                        class: "w-full px-3 py-2 text-sm rounded-md font-mono outline-none transition-colors",
                        style: "background: #0d1219; border: 1px solid #232b38; color: #e2e8f0;",
                        value: "{host_value}",
                        disabled: running,
                        oninput: move |e| host.set(e.value()),
                    }
                }
                // 回数
                div {
                    label {
                        class: "block text-xs font-medium mb-1.5 uppercase tracking-wide",
                        style: "color: #94a3b8;",
                        "Count"
                    }
                    div {
                        class: "flex items-stretch rounded-md overflow-hidden",
                        style: "background: #0d1219; border: 1px solid #232b38;",
                        input {
                            r#type: "number",
                            min: "1",
                            class: "flex-1 w-full px-3 py-2 text-sm font-mono outline-none text-center",
                            style: "background: transparent; border: none; color: #e2e8f0; min-width: 0;",
                            value: "{count_value}",
                            disabled: running,
                            oninput: move |e| {
                                if let Ok(v) = e.value().parse::<u32>() {
                                    count.set(v);
                                }
                            },
                        }
                        div {
                            class: "flex flex-col",
                            style: "border-left: 1px solid #232b38;",
                            button {
                                r#type: "button",
                                class: "flex-1 px-2 text-xs font-mono transition-colors disabled:cursor-not-allowed",
                                style: "background: transparent; border: none; color: #94a3b8; border-bottom: 1px solid #232b38; line-height: 1;",
                                disabled: running,
                                onclick: move |_| {
                                    let v = count();
                                    count.set(v.saturating_add(1));
                                },
                                "▲"
                            }
                            button {
                                r#type: "button",
                                class: "flex-1 px-2 text-xs font-mono transition-colors disabled:cursor-not-allowed",
                                style: "background: transparent; border: none; color: #94a3b8; line-height: 1;",
                                disabled: running,
                                onclick: move |_| {
                                    let v = count();
                                    if v > 1 {
                                        count.set(v - 1);
                                    }
                                },
                                "▼"
                            }
                        }
                    }
                }
            }

            button {
                class: "w-full py-2.5 mb-4 text-sm font-semibold rounded-md tracking-wide transition-all disabled:cursor-not-allowed",
                style: if running { "background: #1e293b; color: #64748b;" } else { "background: linear-gradient(135deg, #10b981 0%, #059669 100%); color: white; box-shadow: 0 2px 8px rgba(16,185,129,0.3);" },
                disabled: running,
                onclick: move |_| on_start.call(()),
                if running {
                    "RUNNING..."
                } else {
                    "▶  PING"
                }
            }
        }
    }
}
