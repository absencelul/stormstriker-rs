#![feature(lazy_cell)]

mod bones;
mod cache;
mod hooks;

use std::ffi::c_void;
use std::sync::atomic::{AtomicBool, Ordering};
use windows::Win32::UI::Input::KeyboardAndMouse::VK_END;
use windows::Win32::{
    Foundation::{BOOL, HINSTANCE},
    System::SystemServices::{
        DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH, DLL_THREAD_ATTACH, DLL_THREAD_DETACH,
    },
};

static EXITING: AtomicBool = AtomicBool::new(false);

#[no_mangle]
extern "stdcall" fn DllMain(hinstance: HINSTANCE, reason: u32, _reserved: *mut c_void) -> BOOL {
    match reason {
        DLL_PROCESS_ATTACH => {
            std::thread::spawn(move || main_thread(hinstance));
        }
        DLL_THREAD_ATTACH => {}
        DLL_THREAD_DETACH => {}
        DLL_PROCESS_DETACH => {
            hooks::unhook_all();
            memory::util::free_console();
            std::thread::sleep(std::time::Duration::from_millis(250));
        }
        _ => {}
    }

    BOOL::from(true)
}

fn on_loop() {
    if memory::util::key_released(VK_END.0) {
        EXITING.store(true, Ordering::Relaxed);
        memory::util::unload();
    }
}

fn main_thread(_hinstance: HINSTANCE) {
    memory::util::alloc_console();
    sdk::init_sdk();

    if let Err(e) = hooks::initialize_hooks() {
        println!("{}", e);
        return;
    }

    println!("[-] Successfully injected into process..");

    while !EXITING.load(Ordering::SeqCst) {
        on_loop();
    }
}
