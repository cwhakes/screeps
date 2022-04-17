pub mod creep;
pub mod game_object;
pub mod structure;
pub mod util;

use js_sys::{JsString, Object};
use wasm_bindgen::prelude::*;

use creep::Creep;

#[wasm_bindgen]
extern "C" {
    pub type SpawnError;

    #[wasm_bindgen(method, getter)]
    pub fn object(this: &SpawnError) -> Creep;
}

#[wasm_bindgen(module = "game/constants")]
extern "C" {
    pub static ATTACK: JsString;
    pub static MOVE: JsString;
}

pub trait Prototype {
    fn prototype() -> &'static Object;
}


