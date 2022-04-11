use engine::*;

#[no_mangle]
pub extern "C" fn game_update_and_render(
    game_memory: &mut GameMemory, 
    game_input: &mut GameInput, 
    buffer: &mut GameOffscreenBuffer
) {
    println!("game_update_and_render1 {:?}", game_memory.permanent_storage);    
}

#[no_mangle]
pub extern "C" fn get_game_sound_samples(    
    game_memory: &mut GameMemory, 
    sound_buffer: &mut GameSoundOutputBuffer

) {
    println!("get_game_sound_samples1 {:?} {:?}", game_memory.permanent_storage, sound_buffer.samples);    
}