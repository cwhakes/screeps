use super::*;

#[wasm_bindgen(module = "game/prototypes")]
extern "C" {    
    #[derive(Debug)]
    pub type GameObject;

    #[wasm_bindgen(method, getter)]
    pub fn my(this: &GameObject) -> Option<bool>;

    #[wasm_bindgen(method, getter)]
    pub fn exists(this: &GameObject) -> bool;

    #[wasm_bindgen(method, getter)]
    pub fn id(this: &GameObject) -> String;

    #[wasm_bindgen(method, getter, js_name = ticksToDecay)]
    pub fn ticks_to_decay(this: &GameObject) -> Option<i32>;

    #[wasm_bindgen(method, getter)]
    pub fn x(this: &GameObject) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn y(this: &GameObject) -> i32;
}
