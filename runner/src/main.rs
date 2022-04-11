use engine::*;

use std::ffi::CString;
use std::ptr;
use winapi::{
    um::{
        libloaderapi::*, winbase::CopyFileA
    }, 
    shared::minwindef::*
};

pub struct Win32GameCode {
    dll: Option<HINSTANCE>,
    game_update_and_render: Box<GameUpdateAndRender>, 
    get_sound_samples: Box<GetGameSoundSamples>}

type GameUpdateAndRender = fn(&mut GameMemory, &mut GameInput, &mut GameOffscreenBuffer);
type GetGameSoundSamples = fn(&mut GameMemory, &mut GameSoundOutputBuffer);


impl Win32GameCode {
    pub fn new() -> Self {
        Self {
            dll: None,
            game_update_and_render: Box::new(stub_game_update_and_render),
            get_sound_samples: Box::new(stub_get_sound_samples)
        }
    }

    pub fn load(&mut self) {
        let compiled_lib_path = CString::new("target\\app.dll").unwrap();
        let lib_path = CString::new("target\\debug\\app-temp.dll").unwrap();
        let lib_name = CString::new("app-temp.dll").unwrap();
        
        unsafe {
            if CopyFileA(
                compiled_lib_path.as_ptr() as *const i8, 
                lib_path.as_ptr() as *const i8, 
                FALSE
            ) == 0 {
                panic!("no copy");
            }
            let dll = LoadLibraryA(lib_name.as_ptr() as *const i8);
            if !dll.is_null() {
                self.dll = Some(dll);
                self.game_update_and_render = get_game_update_and_render_function_from_library(dll);
                self.get_sound_samples = get_get_sound_samples_function_from_library(dll);
            } else {
                println!("No load");
            }
        }
    }

    pub fn unload(&mut self) {
        if let Some(dll) = self.dll {
            unsafe {
                FreeLibrary(dll);

            }
            self.dll = None;
            self.game_update_and_render = Box::new(stub_game_update_and_render);
            self.get_sound_samples = Box::new(stub_get_sound_samples);
        }
    }

    pub fn game_update_and_render(
        &self,
        game_memory: &mut GameMemory, 
        game_input: &mut GameInput, 
        buffer: &mut GameOffscreenBuffer
    ) {
        (self.game_update_and_render)(game_memory, game_input, buffer);
    }
    
    pub fn get_sound_samples(
        &self,        
        game_memory: &mut GameMemory, 
        sound_buffer: &mut GameSoundOutputBuffer
    
    ) {
        (self.get_sound_samples)(game_memory, sound_buffer);
    }    
}

fn get_game_update_and_render_function_from_library(dll: HINSTANCE) -> Box::<GameUpdateAndRender> {
    unsafe {        
        let fn_name = CString::new("game_update_and_render").unwrap();
        let fn_pointer = GetProcAddress(dll, fn_name.as_ptr() as *const i8);
        if fn_pointer.is_null() {
            println!("no love");
            return Box::new(stub_game_update_and_render);
        }
        Box::new(std::mem::transmute::<*const (), GameUpdateAndRender>(fn_pointer as *const()))
    }
}

fn get_get_sound_samples_function_from_library(dll: HINSTANCE) -> Box::<GetGameSoundSamples> {
    unsafe {        
        let fn_name = CString::new("get_game_sound_samples").unwrap();
        let fn_pointer = GetProcAddress(dll, fn_name.as_ptr() as *const i8);
        if fn_pointer.is_null() {
            println!("no love");
            return Box::new(stub_get_sound_samples);
        }
        Box::new(std::mem::transmute::<*const (), GetGameSoundSamples>(fn_pointer as *const()))
    }
}

pub fn stub_game_update_and_render(
    _game_memory: &mut GameMemory, 
    _game_input: &mut GameInput, 
    _buffer: &mut GameOffscreenBuffer
) {
    println!("stub_game_update_and_render");
}

pub fn stub_get_sound_samples(
    _game_memory: &mut GameMemory, 
    _sound_buffer: &mut GameSoundOutputBuffer
) {
    println!("stub_get_sound_samples");
}

fn main() {
        
    let mut screen_buffer = GameOffscreenBuffer { 
        memory: ptr::null_mut(),
        width: 10,
        height: 10,
        pitch: 10
    };

    let mut new_input = &mut GameInput::default();
                    
    let mut game_memory = GameMemory {
        is_initialized: false,
        permanent_storage_size: 64 * 1000 * 1000,
        permanent_storage: ptr::null_mut(),
        transient_storage_size: 1000 * 1000 * 1000,
        transient_storage: ptr::null_mut(),
    };

    let mut game_sound_buffer = GameSoundOutputBuffer {
        sample_count: 10,
        samples: ptr::null_mut(),
        samples_per_second: 50
    };

    let mut load_counter = 0;
    let mut code = Win32GameCode::new();
    code.load();
       
    loop { 
        if load_counter == 1020 {
            code.unload();
            code.load();    
            load_counter = 0;       
        }

        code.game_update_and_render(&mut game_memory, &mut new_input, &mut screen_buffer);
        code.get_sound_samples(&mut game_memory, &mut game_sound_buffer);
        load_counter += 1;
    }
}
