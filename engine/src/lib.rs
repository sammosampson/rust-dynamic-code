use std::ffi::*;

#[repr(C)]
pub struct GameMemory {
    pub is_initialized: bool,
    pub permanent_storage_size: u64,
    pub permanent_storage: *mut c_void,
    pub transient_storage_size: u64,
    pub transient_storage: *mut c_void 
}

#[repr(C)]
pub struct GameOffscreenBuffer {
    pub memory: *mut c_void,
    pub width: u32,
    pub height: u32,
    pub pitch: u32,
}

#[repr(C)]
pub struct GameSoundOutputBuffer {
    pub samples_per_second: u32,
    pub sample_count: u32,
    pub samples: *mut std::ffi::c_void
}

#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct GameButtonState {
    pub half_tansition_count: i32,
    pub ended_down: bool,
}

pub union GameControllerButtons {
    pub all: [GameButtonState; 12],
    pub distinct: GameControllerDistinctButtons,
}

impl Default for GameControllerButtons {
    fn default() -> Self {
        Self {
            all: [GameButtonState::default(); 12]
        }
    }
}

#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct GameControllerDistinctButtons {
    pub move_up: GameButtonState,
    pub move_down: GameButtonState,
    pub move_left: GameButtonState,
    pub move_right: GameButtonState,
    pub action_up: GameButtonState,
    pub action_down: GameButtonState,
    pub action_left: GameButtonState,
    pub action_right: GameButtonState,
    pub left_shoulder: GameButtonState,
    pub right_shoulder: GameButtonState,
    pub back: GameButtonState,
    pub start: GameButtonState,
    pub terminator: GameButtonState
}

#[repr(C)]
#[derive(Default)]
pub struct GameControllerInput {
    pub is_connected: bool,
    pub is_analog: bool,
    pub stick_average_x: f32,
    pub stick_average_y: f32,
    pub buttons: GameControllerButtons,
}

#[repr(C)]
#[derive(Default)]
pub struct GameInput {
    pub controllers: [GameControllerInput; 5]
}

