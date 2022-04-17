#![allow(non_upper_case_globals)]

use std::cell::RefCell;

use wasm_bindgen::prelude::*;
use js_sys::{JsString, Object};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = utils)]
    fn getObjectsByPrototype(proto: &Object) -> Vec<JsValue>;
    fn test();
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    fn get_creep() -> JsValue;
}

mod prototype {
    use super::*;

    #[wasm_bindgen(module = "game/prototypes")]
    extern "C" {
        pub static Creep: Object;
        pub static StructureSpawn: Object;
    }
}

#[wasm_bindgen]
extern "C" {
    pub type SpawnError;
    
    #[wasm_bindgen(method, getter)]
    pub fn object(this: &SpawnError) -> JsValue;
}


mod spawn {
    use super::*;

    #[wasm_bindgen(module = "game/prototypes")]
    extern "C" {
        #[derive(Debug)]
        pub type StructureSpawn;

        #[wasm_bindgen(method, getter)]
        pub fn my(this: &StructureSpawn) -> Option<bool>;

        #[wasm_bindgen(method)]
        pub fn spawnCreep(this: &StructureSpawn, body: Vec<JsString>) -> SpawnError;
    }
}

mod creep {
    use super::*;

    #[wasm_bindgen(module = "game/prototypes")]
    extern "C" {    
        #[derive(Debug)]
        pub type Creep;

        #[wasm_bindgen(method, getter)]
        pub fn my(this: &Creep) -> Option<bool>;

        #[wasm_bindgen(method)]
        pub fn attack(this: &Creep, target: &JsValue) -> JsValue;

        #[wasm_bindgen(method)]
        pub fn moveTo(this: &Creep, target: &JsValue) -> JsValue;
    }
}

#[wasm_bindgen(module = "game/constants")]
extern "C" {
    static ATTACK: JsString;
    static MOVE: JsString;
}


#[wasm_bindgen]
pub fn greet() {
    test();
}

#[wasm_bindgen]
pub fn display(v: JsValue) {
    log(&format!("{:?}", v));
}


#[wasm_bindgen]
pub fn count_creeps() {
    log("python works!");
    let s = getObjectsByPrototype(&prototype::Creep);
    log(&format!("{:?}", s));
}

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

thread_local! {
    static ATTACKER: RefCell<Option<JsValue>> = RefCell::new(None);
}

#[wasm_bindgen]
pub fn loop_inner() {
    let spawn = getObjectsByPrototype(&prototype::StructureSpawn)
        .into_iter()
        .map(spawn::StructureSpawn::from)
        .find(|spawn| spawn.my() == Some(true))
        .unwrap();
    log(&format!("{:?}", spawn));


    ATTACKER.with(|attacker| {
        let mut maybe_attacker = attacker.borrow_mut();
        if let Some(attacker) = maybe_attacker.as_ref() {
            let enemy_spawn = getObjectsByPrototype(&prototype::StructureSpawn)
                .into_iter()
                .map(spawn::StructureSpawn::from)
                .find(|spawn| spawn.my() == Some(false))
                .unwrap();
            let attacker = creep::Creep::from(attacker.clone());
            attacker.moveTo(&enemy_spawn);
            attacker.attack(&enemy_spawn);
        } else {
            let my_spawn = getObjectsByPrototype(&prototype::StructureSpawn)
                .into_iter()
                .map(spawn::StructureSpawn::from)
                .find(|spawn| spawn.my() == Some(true))
                .unwrap();
            let output = my_spawn.spawnCreep(vec![ATTACK.clone(), MOVE.clone()]);
            *maybe_attacker = Some(output.object());
        }
    });
}

