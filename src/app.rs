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
        <main class="flex bg-neutral-700">
            <Sidebar />
            <ContentTopBar />
        </main>

    }
}

#[component]
pub fn Sidebar() -> impl IntoView {
    view! {
        <div id="sidebar" class="flex flex-col flex-[1] bg-neutral-950/50 h-screen  text-neutral-100 ">
            <div class=" flex bg-neutral-900/50 h-8 p-1 *:p-1 gap-1 text-neutral-100 backdrop-blur-md shadow-sm justify-center align-middle text-xs cursor-default select-none">
                <p>Godot Engine - Launcher</p>
            </div>

            <div class="flex *:bg-neutral-900/50 flex-col *:rounded-md *:p-2 gap-2 p-2 backdrop-blur-md shadow-sm justify-start flex-1">
                <button class="hover:bg-neutral-500/50 active:bg-neutral-700/20">Projects</button>
                <button class="hover:bg-neutral-500/50 active:bg-neutral-700/20">Installs</button>
            </div>

            <div class="flex *:bg-neutral-900/50 flex-col *:rounded-md *:p-2 gap-2 p-2 backdrop-blur-md shadow-sm justify-end">
                <button class="hover:bg-neutral-500/50 active:bg-neutral-700/20 content-end">Settings</button>
            </div>


        </div>

    }
}

#[component]
pub fn ContentTopBar() -> impl IntoView {
    view! {
        <div class=" flex bg-neutral-900/50 flex-row flex-[3] h-8 p-1 *:bg-neutral-900/50 *:p-1 *:rounded-md gap-1 *:w-32 text-neutral-100 backdrop-blur-md shadow-sm justify-end text-xs">
            <button class="hover:bg-neutral-400/50 active:bg-neutral-600/20">Add Project</button>
            <button class="hover:bg-neutral-400/50 active:bg-neutral-600/20">Import Project</button>
        </div>
    }
}

pub fn Metadata() -> impl IntoView {
    view! {
        <div>

        </div>
    }
}
