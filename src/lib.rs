#![recursion_limit = "1024"]
#![allow(clippy::large_enum_variant)]

pub mod app;

use yew::prelude::*;
use wasm_bindgen::prelude::*;
use app::Model;

#[wasm_bindgen(start)]
pub fn start() {
    use js_sys::{global, Reflect};

    if Reflect::has(&global(), &JsValue::from_str("window")).unwrap() {
        App::<Model>::new().mount_to_body();
    } else {
        app::Worker::register();
    }
}

