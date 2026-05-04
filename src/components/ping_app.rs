use std::time::Duration;

use dioxus::prelude::*;

use crate::components::header::Header;
use crate::components::input_form::InputForm;
use crate::components::output_list::OutputList;
use crate::components::summary_card::SummaryArea;

use crate::ping_impl;
use crate::types::{resolve_host, PingResult};

#[component]
pub fn PingApp() -> Element {
    let host = use_signal(|| String::from("8.8.8.8"));
    let count = use_signal(|| 3u32);
    let mut results = use_signal(Vec::<PingResult>::new);
    let mut is_running = use_signal(|| false);

    let count_value = count();
    let running = is_running();

    let mut start_ping = move |_| {
        if is_running() {
            return;
        }
        is_running.set(true);
        results.write().clear();

        let host_v = host();
        let count_v = count();

        spawn(async move {
            // ホスト解決
            let ip = match resolve_host(&host_v) {
                Ok(ip) => ip,
                Err(e) => {
                    results.write().push(PingResult {
                        seq: 1,
                        host: host_v.clone(),
                        success: false,
                        time_ms: None,
                        error: Some(format!("resolve failed: {e}")),
                    });
                    is_running.set(false);
                    return;
                }
            };

            // pingクライアント作成
            let client = match ping_impl::create_client() {
                Ok(c) => c,
                Err(e) => {
                    results.write().push(PingResult {
                        seq: 1,
                        host: host_v.clone(),
                        success: false,
                        time_ms: None,
                        error: Some(e),
                    });
                    is_running.set(false);
                    return;
                }
            };

            // 指定回数pingを実行
            for i in 1..=count_v {
                if i > 1 {
                    async_std::task::sleep(Duration::from_secs(1)).await;
                }

                let result = ping_impl::ping_once(&client, ip, i as u16).await;
                let entry = match result {
                    Ok(dur) => PingResult {
                        seq: i,
                        host: host_v.clone(),
                        success: true,
                        time_ms: Some(dur.as_secs_f64() * 1000.0),
                        error: None,
                    },
                    Err(e) => PingResult {
                        seq: i,
                        host: host_v.clone(),
                        success: false,
                        time_ms: None,
                        error: Some(e),
                    },
                };
                results.write().push(entry);
            }
            is_running.set(false);
        });
    };

    rsx! {
        div {
            class: "h-screen w-screen overflow-hidden flex items-center justify-center p-6",
            style: "background: linear-gradient(135deg, #0a0e14 0%, #131820 100%);",

            div {
                class: "w-full max-w-2xl rounded-xl flex flex-col",
                style: "background: #151b24; border: 1px solid #232b38; box-shadow: 0 8px 32px rgba(0,0,0,0.4);",

                Header { running }
                InputForm {
                    host,
                    count,
                    running,
                    on_start: move |_| start_ping(()),
                }
                OutputList { results, count: count_value }
                SummaryArea { results }
            }
        }
    }
}
