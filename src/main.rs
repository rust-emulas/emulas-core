use std::ops::Not;

use std::{
    ffi::{CStr, CString},
    mem::zeroed,
};

use rust_emulas::cpu::Cpu;
use rust_emulas::cpu::instruction::INSTRUCTION_TABLE;
use rust_emulas::memory::Bus;

use sdl3_sys::{
    error::SDL_GetError,
    events::{SDL_EVENT_QUIT, SDL_PollEvent},
    init::{SDL_INIT_VIDEO, SDL_Init, SDL_Quit},
    video::{SDL_CreateWindow, SDL_DestroyWindow, SDL_WINDOW_OPENGL},
};

fn main() {
    let addr: u16 = 0x1FFF;
    let mut b: Bus = Bus::new();
    let mut cpu: Cpu = Cpu::new();

    b.write(addr, 0x00);
    b.write(addr + 1, (0xFFu8).wrapping_add(0x02));

    let brk = b.read(addr);
    println!("Value at address {:#X}", b.read(addr));
    println!("Value at address {:#X}", b.read(addr + 1));
    println!("Value at address {:#X}", b.read(addr + 2));

    let instr = &INSTRUCTION_TABLE[brk as usize];
    instr.kind.execute(instr.mode, &mut cpu, &mut b);

    // if unsafe { SDL_Init(SDL_INIT_VIDEO).not() } {
    //     panic!("SDL_Init failed: {:?}", unsafe {
    //         CStr::from_ptr(SDL_GetError())
    //     });
    // }

    // // Create window
    // let title = CString::new("SDL3 Bindgen Example").unwrap();
    // let window = unsafe { SDL_CreateWindow(title.as_ptr(), 800, 600, SDL_WINDOW_OPENGL) };

    // if window.is_null() {
    //     panic!("SDL_CreateWindow failed: {:?}", unsafe {
    //         CStr::from_ptr(SDL_GetError())
    //     });
    // }

    // // Main loop
    // let mut event = unsafe { zeroed() };
    // let mut running = true;
    // while running {
    //     while unsafe { SDL_PollEvent(&mut event).not() } {
    //         if unsafe { event.r#type == SDL_EVENT_QUIT.into() } {
    //             running = false;
    //         }
    //     }

    //     // Add rendering here
    // }

    // // Cleanup
    // unsafe { SDL_DestroyWindow(window) };
    // unsafe { SDL_Quit() };
}
