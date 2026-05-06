#![allow(non_snake_case)]

//! NetHack as a Unity native plugin (cdylib)
//! 
//! This library exports C-compatible functions that Unity can call via DllImport.

use nethack_core::GameState;
use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref GAME_STATE: Mutex<GameState> = Mutex::new(GameState::new());
}

/// Initialize NetHack
#[no_mangle]
pub extern "C" fn nethack_init() -> i32 {
    *GAME_STATE.lock().unwrap() = GameState::new();
    0
}

/// Send a command to the game
#[no_mangle]
pub extern "C" fn nethack_send_command(cmd: u8) -> i32 {
    let mut state = GAME_STATE.lock().unwrap();
    match cmd {
        b'h' => state.move_player(-1, 0), // left
        b'j' => state.move_player(0, 1),  // down
        b'k' => state.move_player(0, -1), // up
        b'l' => state.move_player(1, 0),  // right
        _ => {}
    }
    0
}

/// Get game state (serialized)
#[no_mangle]
pub extern "C" fn nethack_get_state(buf: *mut u8, len: usize) -> i32 {
    if buf.is_null() {
        return -1;
    }
    let state = GAME_STATE.lock().unwrap();
    match bincode::serialize(&*state) {
        Ok(data) => {
            if data.len() <= len {
                unsafe {
                    std::ptr::copy_nonoverlapping(data.as_ptr(), buf, data.len());
                }
                data.len() as i32
            } else {
                -1
            }
        }
        Err(_) => -1,
    }
}

/// Cleanup
#[no_mangle]
pub extern "C" fn nethack_free() {
    // Cleanup if needed
}
