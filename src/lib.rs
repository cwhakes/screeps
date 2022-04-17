#![allow(non_upper_case_globals)]

mod creep;
mod game_object;
mod structure;
mod util;

use std::cell::RefCell;
use structure::StructureSpawn;
use creep::Creep;

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
    pub fn object(this: &SpawnError) -> Creep;
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

#[derive(Default)]
struct Memory {
    //spawner: Option<StructureSpawn>,
    enemy_spawn: Option<StructureSpawn>,
    attacker: Option<Creep>,
}

thread_local! {
    static MEMORY: RefCell<Memory> = RefCell::new(Memory::default());
}

#[wasm_bindgen]
pub fn loop_inner() {
    MEMORY.with(|memory| {
        let Memory { ref mut enemy_spawn, ref mut attacker} = *memory.borrow_mut();

        let enemy_spawn = enemy_spawn.get_or_insert_with(||{
            util::get_objects_by_prototype::<StructureSpawn>()
                .into_iter()
                .find(|spawn| spawn.my() == Some(false))
                .unwrap()
        });

        if let Some(attacker) = attacker.as_ref() {
            attacker.moveTo(&enemy_spawn);
            attacker.attack(&enemy_spawn);
        } else {
            let my_spawn = util::get_objects_by_prototype::<StructureSpawn>()
                .into_iter()
                .find(|spawn| spawn.my() == Some(true))
                .unwrap();
            let output = my_spawn.spawnCreep(vec![ATTACK.clone(), MOVE.clone()]);
            *attacker = Some(output.object());
        }
    });
}
