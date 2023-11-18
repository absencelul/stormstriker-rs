pub mod process;
pub mod util;

use std::sync::OnceLock;

use windows::core::PCSTR;
use windows::Win32::System::LibraryLoader::GetModuleHandleA;

static BASE_ADDRESS: OnceLock<u64> = OnceLock::new();

pub fn get_base_address() -> u64 {
    *BASE_ADDRESS
        .get_or_init(|| unsafe { GetModuleHandleA(PCSTR(std::ptr::null())).unwrap().0 as u64 })
}
