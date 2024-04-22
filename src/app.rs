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
        <main class="bg-neutral-700">
            <Sidebar />
        </main>

    }
}

#[component]
pub fn Sidebar() -> impl IntoView {
    view! {
        <div id="sidebar" class="flex flex-col w-[33dvw] bg-neutral-800 h-screen *:bg-neutral-600 text-neutral-100 *:rounded-md *:p-2 gap-2 p-2">
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
