use super::*;
use super::game_object::GameObject;

#[wasm_bindgen(module = "game/prototypes")]
extern "C" {
    #[wasm_bindgen(extends = GameObject)]
    #[derive(Debug)]
    pub type Structure;

    #[wasm_bindgen(method, getter)]
    pub fn hits(this: &Structure) -> i32;
    
    #[wasm_bindgen(method, getter, js_name = hitsMax)]
    pub fn hits_max(this: &Structure) -> i32;
}

#[wasm_bindgen(module = "game/prototypes")]
extern "C" {
    #[wasm_bindgen(extends = Structure)]
    #[derive(Debug)]
    pub type OwnedStructure;

    #[wasm_bindgen(method, getter)]
    pub fn my(this: &OwnedStructure) -> Option<bool>;
}

#[wasm_bindgen(module = "game/prototypes")]
extern "C" {
    #[wasm_bindgen(extends = OwnedStructure)]
    #[derive(Debug)]
    pub type StructureSpawn;

    #[wasm_bindgen(method)]
    pub fn spawnCreep(this: &StructureSpawn, body: Vec<JsString>) -> SpawnError;
}

