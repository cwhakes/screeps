use wasm_bindgen::prelude::*;

use js_sys::Object;

use crate::Prototype;

#[wasm_bindgen(module = "game/utils")]
extern "C" {
    fn getObjectsByPrototype(proto: &Object) -> Vec<JsValue>;
}

pub fn get_objects_by_prototype<T: Prototype + From<JsValue>>() -> Vec<T> {
    let values = getObjectsByPrototype(T::prototype());
    values.into_iter().map(T::from).collect()
}
