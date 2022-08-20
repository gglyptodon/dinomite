use crate::dinomite::*;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;

thread_local! {
   static DINOMITE: RefCell<Dinomite> = RefCell::new(Dinomite::new(10,10,10));
}

#[wasm_bindgen(js_name=getState)]
pub fn get_state() -> String {
    DINOMITE.with(|dm| dm.borrow().to_string())
}
#[wasm_bindgen(js_name=newGame)]
pub fn new_game(width: usize, height: usize, num_dinos: usize) {
    DINOMITE.with(|dm| dm.borrow_mut().reconfigure(height, width, num_dinos))
}
#[wasm_bindgen(js_name=newGameDefault)]
pub fn new_game_default() {
    DINOMITE.with(|dm| dm.borrow_mut().reconfigure(10, 10, 10))
}

#[wasm_bindgen(js_name=toggleFlag)]
pub fn toggle_flag(x: usize, y: usize) {
    DINOMITE.with(|dm| dm.borrow_mut().toggle_flag(&Position(x, y)))
}
#[wasm_bindgen(js_name=checkPosition)]
pub fn check_position(x: usize, y: usize) {
    DINOMITE.with(|dm| dm.borrow_mut().check_position(&Position(x, y)));
}
#[wasm_bindgen(js_name=checkNeighbors)]
pub fn check_neighbors(x: usize, y: usize) {
    DINOMITE.with(|dm| dm.borrow_mut().check_neighbors(&Position(x, y)));
}

#[wasm_bindgen(js_name=isGameOver)]
pub fn is_game_over() -> bool {
    DINOMITE.with(|dm| dm.borrow().is_game_over())
}

#[wasm_bindgen(js_name=isWon)]
pub fn is_won() -> bool {
    DINOMITE.with(|dm| dm.borrow().is_won())
}
