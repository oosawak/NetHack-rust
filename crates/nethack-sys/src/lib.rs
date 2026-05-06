//! FFI wrapper for NetHack C library
//! 
//! This crate provides safe Rust bindings to the NetHack game engine.
//! NetHack is written in C (~250k lines), and we use FFI to call it directly.
//! 
//! The game initialization is a multi-stage process:
//!   1. early_init() - Initialize global state
//!   2. choose_windows() - Select UI system
//!   3. initoptions() - Read configuration
//!   4. init_nhwindows() - Initialize window system
//!   5. dlb_init() - Initialize dungeon database
//!   6. vision_init() - Initialize vision system
//!   7. newgame() or restore - Start game
//!   8. moveloop() - Main game loop

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// Include auto-generated bindings from build.rs
include!(concat!(env!("OUT_DIR"), "/ffi.rs"));

pub mod globals;

use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref GAME_STATE: Mutex<GameState> = Mutex::new(GameState::new());
}

/// Game initialization stages
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum InitStage {
    Uninitialized,
    EarlyInit,
    WindowsChosen,
    OptionsInitialized,
    WindowsInitialized,
    DlbInitialized,
    VisionInitialized,
    GameReady,
}

/// Safe wrapper around NetHack game state
pub struct GameState {
    stage: InitStage,
}

impl GameState {
    fn new() -> Self {
        GameState {
            stage: InitStage::Uninitialized,
        }
    }

    /// Stage 1: Initialize global state
    pub fn early_init(&mut self) -> Result<(), String> {
        if self.stage != InitStage::Uninitialized {
            return Err(format!("Already at stage {:?}", self.stage));
        }

        unsafe {
            early_init();
        }

        self.stage = InitStage::EarlyInit;
        Ok(())
    }

    /// Stage 2: Choose window system
    pub fn choose_windows(&mut self, window_sys: i32) -> Result<(), String> {
        if self.stage != InitStage::EarlyInit {
            return Err(format!("Expected EarlyInit, got {:?}", self.stage));
        }

        unsafe {
            choose_windows(window_sys);
        }

        self.stage = InitStage::WindowsChosen;
        Ok(())
    }

    /// Stage 3: Initialize options
    pub fn init_options(&mut self) -> Result<(), String> {
        if self.stage != InitStage::WindowsChosen {
            return Err(format!("Expected WindowsChosen, got {:?}", self.stage));
        }

        unsafe {
            initoptions();
        }

        self.stage = InitStage::OptionsInitialized;
        Ok(())
    }

    /// Stage 4: Initialize window system
    pub fn init_nhwindows(&mut self) -> Result<(), String> {
        if self.stage != InitStage::OptionsInitialized {
            return Err(format!("Expected OptionsInitialized, got {:?}", self.stage));
        }

        unsafe {
            init_nhwindows(&mut 0, std::ptr::null_mut());
        }

        self.stage = InitStage::WindowsInitialized;
        Ok(())
    }

    /// Stage 5: Initialize dungeon database
    pub fn init_dlb(&mut self) -> Result<(), String> {
        if self.stage != InitStage::WindowsInitialized {
            return Err(format!("Expected WindowsInitialized, got {:?}", self.stage));
        }

        unsafe {
            dlb_init();
        }

        self.stage = InitStage::DlbInitialized;
        Ok(())
    }

    /// Stage 6: Initialize vision system
    pub fn init_vision(&mut self) -> Result<(), String> {
        if self.stage != InitStage::DlbInitialized {
            return Err(format!("Expected DlbInitialized, got {:?}", self.stage));
        }

        unsafe {
            vision_init();
            init_sound_disp_gamewindows();
        }

        self.stage = InitStage::VisionInitialized;
        Ok(())
    }

    /// Stage 7: Create new game (or restore from save)
    pub fn new_game(&mut self) -> Result<(), String> {
        if self.stage != InitStage::VisionInitialized {
            return Err(format!("Expected VisionInitialized, got {:?}", self.stage));
        }

        unsafe {
            newgame();
        }

        self.stage = InitStage::GameReady;
        Ok(())
    }

    /// Get current initialization stage
    pub fn stage(&self) -> InitStage {
        self.stage
    }

    /// Get current dungeon level (only after GameReady)
    pub fn dungeon_level(&self) -> Result<i32, String> {
        if self.stage < InitStage::GameReady {
            return Err("Game not initialized".to_string());
        }

        unsafe { Ok(dlevel) }
    }
}

/// Public API - Stage-aware initialization

pub fn init_stage_1_early() -> Result<(), String> {
    let mut state = GAME_STATE.lock().map_err(|e| e.to_string())?;
    state.early_init()
}

pub fn init_stage_2_choose_windows() -> Result<(), String> {
    let mut state = GAME_STATE.lock().map_err(|e| e.to_string())?;
    state.choose_windows(1)  // DEFAULT_WINDOW_SYS = 1
}

pub fn init_stage_3_options() -> Result<(), String> {
    let mut state = GAME_STATE.lock().map_err(|e| e.to_string())?;
    state.init_options()
}

pub fn init_stage_4_nhwindows() -> Result<(), String> {
    let mut state = GAME_STATE.lock().map_err(|e| e.to_string())?;
    state.init_nhwindows()
}

pub fn init_stage_5_dlb() -> Result<(), String> {
    let mut state = GAME_STATE.lock().map_err(|e| e.to_string())?;
    state.init_dlb()
}

pub fn init_stage_6_vision() -> Result<(), String> {
    let mut state = GAME_STATE.lock().map_err(|e| e.to_string())?;
    state.init_vision()
}

pub fn init_stage_7_newgame() -> Result<(), String> {
    let mut state = GAME_STATE.lock().map_err(|e| e.to_string())?;
    state.new_game()
}

pub fn current_stage() -> Result<InitStage, String> {
    let state = GAME_STATE.lock().map_err(|e| e.to_string())?;
    Ok(state.stage())
}

pub fn get_dungeon_level() -> Result<i32, String> {
    let state = GAME_STATE.lock().map_err(|e| e.to_string())?;
    state.dungeon_level()
}

/// Convenience function: Full initialization through game ready
pub fn full_init() -> Result<(), String> {
    init_stage_1_early()?;
    init_stage_2_choose_windows()?;
    init_stage_3_options()?;
    init_stage_4_nhwindows()?;
    init_stage_5_dlb()?;
    init_stage_6_vision()?;
    init_stage_7_newgame()?;
    Ok(())
}
