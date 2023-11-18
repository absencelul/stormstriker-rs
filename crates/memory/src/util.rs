use std::ffi::CString;
use windows::core::PCSTR;
use windows::Win32::System::Console::{AllocConsole, FreeConsole};
use windows::Win32::System::LibraryLoader::{FreeLibraryAndExitThread, GetModuleHandleA};
use windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState;

pub fn read_null_terminated_string(base_address: usize) -> Option<String> {
    let len = (0..500)
        .take_while(|&i| unsafe { *(base_address as *const u8).offset(i) != 0 })
        .count();
    let bytes = unsafe { std::slice::from_raw_parts(base_address as *const u8, len) };
    match String::from_utf8(bytes.to_vec()) {
        Ok(s) => Some(s),
        Err(_) => None,
    }
}

pub fn alloc_console() {
    unsafe {
        AllocConsole().expect("TODO: panic message");
    }
}

pub fn free_console() {
    unsafe {
        FreeConsole().expect("TODO: panic message");
    }
}

pub fn key_released(key: u16) -> bool {
    static mut PRESSED_KEYS: [bool; 255] = [false; 255];

    unsafe {
        let result = GetAsyncKeyState(key as i32);

        let is_currently_pressed = (result >> 15) & 1 != 0;
        let was_previously_pressed = PRESSED_KEYS[key as usize];

        PRESSED_KEYS[key as usize] = is_currently_pressed;

        !is_currently_pressed && was_previously_pressed
    }
}

pub fn unload() {
    let module_name = CString::new("stormstriker_rs.dll").unwrap();
    println!("[-] unloading {:?}", module_name);

    unsafe {
        let module_handle =
            GetModuleHandleA(PCSTR::from_raw(module_name.as_ptr() as *const u8)).unwrap();
        FreeLibraryAndExitThread(module_handle, 0);
    }
}