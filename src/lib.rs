#![allow(non_upper_case_globals)]

mod creep;
mod game_object;
mod structure;
mod util;

use std::cell::RefCell;

use wasm_bindgen::prelude::*;
use js_sys::{JsString, Object};

#[wasm_bindgen]
extern "C" {
    fn test();
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    fn get_creep() -> JsValue;
}

#[wasm_bindgen]
extern "C" {
    pub type SpawnError;
    
    #[wasm_bindgen(method, getter)]
    pub fn object(this: &SpawnError) -> JsValue;
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
    let s = util::get_objects_by_prototype::<creep::Creep>();
    log(&format!("{:?}", s));
}

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub trait Prototype {
    fn prototype() -> &'static Object;
}

thread_local! {
    static ATTACKER: RefCell<Option<JsValue>> = RefCell::new(None);
}

#[wasm_bindgen]
pub fn loop_inner() {
    let spawn = util::get_objects_by_prototype::<structure::StructureSpawn>()
        .into_iter()
        .find(|spawn| spawn.my() == Some(true))
        .unwrap();
    log(&format!("{:?}", spawn));


    ATTACKER.with(|attacker| {
        let mut maybe_attacker = attacker.borrow_mut();
        if let Some(attacker) = maybe_attacker.as_ref() {
            let enemy_spawn = util::get_objects_by_prototype::<structure::StructureSpawn>()
                .into_iter()
                .find(|spawn| spawn.my() == Some(false))
                .unwrap();
            let attacker = creep::Creep::from(attacker.clone());
            log(&format!("x: {}, y: {}", attacker.x(), attacker.y()));
            log(&format!("{:?}", attacker));
            attacker.moveTo(&enemy_spawn);
            attacker.attack(&enemy_spawn);
        } else {
            let my_spawn = util::get_objects_by_prototype::<structure::StructureSpawn>()
                .into_iter()
                .find(|spawn| spawn.my() == Some(true))
                .unwrap();
            let output = my_spawn.spawnCreep(vec![ATTACK.clone(), MOVE.clone()]);
            *maybe_attacker = Some(output.object());
        }
    });
}
