use wasm_bindgen::prelude::*;
use crate::dinomite::*;

thread_local! {
   static DINOMITE: Dinomite = Dinomite::new(10,10,10);
}

#[wasm_bindgen(js_name=getState)]
pub fn get_state()->String{
       DINOMITE.with(|dm|dm.to_string())
}