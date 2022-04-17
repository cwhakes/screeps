use super::*;
use super::game_object::GameObject;

#[wasm_bindgen(module = "game/prototypes")]
extern "C" {    
    #[wasm_bindgen(extends = GameObject)]
    #[derive(Debug)]
    pub type Creep;

    #[wasm_bindgen(method, getter)]
    pub fn my(this: &Creep) -> Option<bool>;

    #[wasm_bindgen(method)]
    pub fn attack(this: &Creep, target: &JsValue) -> JsValue;

    #[wasm_bindgen(method)]
    pub fn moveTo(this: &Creep, target: &JsValue) -> JsValue;
}