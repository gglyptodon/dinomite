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
pub fn new_game() {
    DINOMITE.with(|dm| dm.borrow_mut().reconfigure(10, 10, 10))
}
