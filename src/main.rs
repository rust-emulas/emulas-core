mod memory;

use std::ops::Not;

use std::{
    ffi::{CStr, CString},
    mem::zeroed,
};

use sdl3_sys::hints::{SDL_HINT_VIDEO_DRIVER, SDL_SetHint};
use sdl3_sys::{
    error::SDL_GetError,
    events::{SDL_EVENT_QUIT, SDL_PollEvent},
    init::{SDL_INIT_VIDEO, SDL_Init, SDL_Quit},
    video::{SDL_CreateWindow, SDL_DestroyWindow, SDL_GetCurrentVideoDriver, SDL_WINDOW_OPENGL},
};

fn main() {
    if unsafe { SDL_Init(SDL_INIT_VIDEO).not() } {
        panic!("SDL_Init failed: {:?}", unsafe {
            CStr::from_ptr(SDL_GetError())
        });
    }

    unsafe {
        let video_driver = SDL_GetCurrentVideoDriver();
        let video_driver_str = CStr::from_ptr(video_driver).to_str().unwrap();

        println!("Current video driver: {}", video_driver_str);
        SDL_SetHint(SDL_HINT_VIDEO_DRIVER, video_driver);
        SDL_Init(SDL_INIT_VIDEO);
    }

    // Create window
    let title = CString::new("SDL3 Bindgen Example").unwrap();
    let window = unsafe { SDL_CreateWindow(title.as_ptr(), 800, 600, SDL_WINDOW_OPENGL) };

    if window.is_null() {
        panic!("SDL_CreateWindow failed: {:?}", unsafe {
            CStr::from_ptr(SDL_GetError())
        });
    }

    // Main loop
    let mut event = unsafe { zeroed() };
    let mut running = true;
    while running {
        while unsafe { SDL_PollEvent(&mut event).not() } {
            if unsafe { event.r#type == SDL_EVENT_QUIT.into() } {
                running = false;
            }
        }

        // Add rendering here
    }

    // Cleanup
    unsafe { SDL_DestroyWindow(window) };
    unsafe { SDL_Quit() };
}
