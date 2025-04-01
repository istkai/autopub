// use serde::{Deserialize, Serialize};
// use wasm_bindgen::prelude::*;
// use wasm_bindgen_futures::spawn_local;
use autopub_ui::video::Video;
use yew::prelude::*;

// #[wasm_bindgen]
// extern "C" {
//     #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
//     async fn invoke(cmd: &str, args: JsValue) -> JsValue;
// }
//
// #[derive(Serialize, Deserialize)]
// struct GreetArgs<'a> {
//     name: &'a str,
// }
//
// #[function_component(App)]
// pub fn app() -> Html {
//     html! {
//         <>
//             <h1> { "Test HTML" } </h1>
//             <div>
//                 <h3>{ "HTML Test" }</h3>
//             </div>
//         </>
//     }
// }

let videos = vec![
    Video {
        id: 1,
        title: "Building and breaking things".to_string(),
        speaker: "John Doe".to_string(),
        url: "https://youtu.be/PsaFVLr8t4E".to_string(),
    },
    Video {
        id: 2,
        title: "The development process".to_string(),
        speaker: "Jane Smith".to_string(),
        url: "https://youtu.be/PsaFVLr8t4E".to_string(),
    },
    Video {
        id: 3,
        title: "The Web 7.0".to_string(),
        speaker: "Matt Miller".to_string(),
        url: "https://youtu.be/PsaFVLr8t4E".to_string(),
    },
    Video {
        id: 4,
        title: "Mouseless development".to_string(),
        speaker: "Tom Jerry".to_string(),
        url: "https://youtu.be/PsaFVLr8t4E".to_string(),
    }
];

#[function_component(App)]
pub fn app() -> Html {
    html! {
    <>
        <h1>{ "RustConf Explorer" }</h1>
        <div>
            <h3>{ "Videos to watch" }</h3>
            <p>{ "John Doe: Building and breaking things" }</p>
            <p>{ "Jane Smith: The development process" }</p>
            <p>{ "Matt Miller: The Web 7.0" }</p>
            <p>{ "Tom Jerry: Mouseless development" }</p>
        </div>
        <div>
            <h3>{ "John Doe: Building and breaking things" }</h3>
            <img
            src="https://placehold.co/640x360.png?text=Video+Player+Placeholder"
            alt="video thumbnail" />
        </div>
    </>
    }
}
