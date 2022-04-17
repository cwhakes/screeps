use super::game_object::GameObject;
use super::*;

#[wasm_bindgen(module = "game/prototypes")]
extern "C" {
    #[wasm_bindgen(extends = GameObject)]
    #[derive(Debug)]
    pub type Creep;

    #[wasm_bindgen(js_name = Creep)]
    pub static CREEP_PROTOTYPE: Object;

    #[wasm_bindgen(method, getter)]
    pub fn my(this: &Creep) -> Option<bool>;

    #[wasm_bindgen(method)]
    pub fn attack(this: &Creep, target: &GameObject) -> JsValue;

    #[wasm_bindgen(method)]
    pub fn moveTo(this: &Creep, target: &GameObject) -> JsValue;
}

impl Prototype for Creep {
    fn prototype() -> &'static Object {
        &CREEP_PROTOTYPE
    }
}
