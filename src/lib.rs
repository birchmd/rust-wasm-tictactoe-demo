//This file defines the tictactoe player's behaviour.
//It is compiled down to wasm using `wasm-pack`, then run in the game engine.

extern crate wasm_bindgen;

use std::iter::Iterator;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    fn get(idx: i32) -> i32;
    fn set(idx: i32);
}

fn choose_cell<I>(mut board: I) -> i32 where I: Iterator<Item = i32> {
    // a dumb strategy -- just fill in the first empty cell
    let result = match board.position(|c| c == 0) {
        Some(idx) => idx,
        None => 0,
    };
    result as i32
}

#[wasm_bindgen]
pub fn mk_turn() {
    let cells = [0, 1, 2, 3, 4, 5, 6, 7, 8];
    let cell = choose_cell( cells.iter().map(|idx| get(*idx)) );
    set(cell);
}
