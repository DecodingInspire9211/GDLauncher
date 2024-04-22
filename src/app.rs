use leptos::leptos_dom::ev::SubmitEvent;
use leptos::*;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <main>
            <Sidebar />
        </main>

    }
}

#[component]
pub fn Sidebar() -> impl IntoView {
    view! {
        <div id="sidebar">
            <button>Projects</button>
            <button>Installs</button>
            <button>Settings</button>
        </div>

    }
}

pub fn Metadata() -> impl IntoView {
    view! {
        <div>

        </div>
    }
}
