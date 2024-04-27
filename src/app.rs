use std::{env::args, ptr::null};

use leptos::leptos_dom::ev::SubmitEvent;
use leptos::*;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::{convert::js_value_vector_into_abi, prelude::*};
use web_sys::{Event, FocusEvent};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <main class="transition-all flex bg-neutral-700 subpixel-antialiased">
            <Sidebar />
            <ContentTopBar />
        </main>

    }
}

#[derive(Serialize, Deserialize)]
struct OSArgs<'a> {
    platform: &'a str,
}

#[component]
pub fn Sidebar() -> impl IntoView {
    window_event_listener(event, cb)

    view! {
        <div id="sidebar" class="flex flex-col flex-[1] bg-neutral-950/50 h-screen  text-neutral-100 z-40">
            <div class=" flex bg-neutral-900/50 h-8 p-1 *:p-1 gap-1 text-neutral-100 backdrop-blur-md z-50 shadow-sm justify-center align-middle text-xs cursor-default select-none">
                <p>GDLauncher - 0.0.0</p>
            </div>

            <div class="flex *:bg-neutral-900/50 *:border-b-2 *:border-neutral-900/50 flex-col *:rounded-md *:p-2 gap-2 p-2 backdrop-blur-md shadow-sm justify-start flex-1">
                <button class="hover:bg-neutral-500/50 active:bg-neutral-700/20 active:border-neutral-700/0">Projects</button>
                <button class="hover:bg-neutral-500/50 active:bg-neutral-700/20 active:border-neutral-700/0">Installs</button>
            </div>

            <div class="flex *:bg-neutral-900/50 flex-col *:border-b-2 *:border-neutral-900/50 *:rounded-md *:p-2 gap-2 p-2 backdrop-blur-md shadow-sm justify-end">
                <button class="hover:bg-neutral-500/50 active:bg-neutral-700/20 active:border-neutral-700/0 content-end">Settings</button>
            </div>

            <div>
                <Metadata author/>
            </div>


        </div>

    }
}

#[component]
pub fn ContentTopBar() -> impl IntoView {
    view! {
        <div class="z-30 flex bg-neutral-900/50 flex-row flex-[3] h-8 p-1 *:bg-neutral-900/50 *:border-b-2 *:border-neutral-900/50 *:p-1 *:rounded-md gap-1 *:w-32 text-neutral-100 backdrop-blur-md shadow-sm justify-end text-xs">
            <button class="hover:bg-neutral-400/50 active:bg-neutral-600/20 active:border-neutral-600/0">New Project</button>
            <button class="hover:bg-neutral-400/50 active:bg-neutral-600/20 active:border-neutral-600/0">Import Project</button>
        </div>
    }
}

#[component]
pub fn Metadata(author: String) -> impl IntoView {
    view! {
        <div class="bg-neutral-900/50 h-8 p-1 *:p-1 gap-1 text-neutral-100 backdrop-blur-md z-50 shadow-sm justify-center align-middle text-xs cursor-default select-none text-center">
            <p>{author}</p>
        </div>
    }
}

// fn version() -> String {
//     return Manager::package_info().name.to_string();
// }
