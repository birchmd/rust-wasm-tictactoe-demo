//This file defines the tictactoe player's behaviour.
//It is compiled down to wasm using `cargo build --target wasm32-unknown-unknown`,
//then run in the game engine.

#![no_std]
#![feature(lang_items)]

mod external {
    extern "C" {
        pub fn get(idx: i32) -> i32;
        pub fn set(idx: i32);
        pub fn panic(payload_ptr: *const u8, payload_len: u32) -> !;
    }

    #[lang = "eh_personality"]
    extern "C" fn eh_personality() {}
}

#[no_mangle]
#[panic_handler]
pub fn panic_fmt(_info: &crate::core::panic::PanicInfo) -> ! {
    unsafe {
        external::panic(crate::core::ptr::null(), 0u32);
    }
}

fn get(idx: i32) -> i32 {
    unsafe { external::get(idx) }
}
fn set(idx: i32) {
    unsafe { external::set(idx) }
}

fn choose_cell<I>(mut board: I) -> i32
where
    I: Iterator<Item = i32>,
{
    // a dumb strategy -- just fill in the first empty cell
    let result = match board.position(|c| c == 0) {
        Some(idx) => idx,
        None => 0,
    };
    result as i32
}

#[no_mangle]
pub fn mk_turn() {
    let cells = [0, 1, 2, 3, 4, 5, 6, 7, 8];
    let cell = choose_cell(cells.iter().map(|idx| get(*idx)));
    set(cell);
}
