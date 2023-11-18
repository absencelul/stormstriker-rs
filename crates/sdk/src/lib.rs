#![feature(lazy_cell)]

use crate::basic::classes::{FNamePool, TUObjectArray};
use crate::engine::classes::UWorld;
use std::sync::{LazyLock, Mutex};

pub mod basic;
pub mod core;
pub mod engine;

pub mod offsets {
    /*
    inline int32 AppendString      = 0x029A4430;
    inline int32 ProcessEvent      = 0x02B29B20;
    */
    pub const GLOBAL_OBJECTS_OFFSET: u64 = 0x07998720;
    pub const GLOBAL_WORLD_OFFSET: u64 = 0x07B05170;
    pub const GLOBAL_NAMES_OFFSET: u64 = 0x078F9040;
}

static mut G_WORLD: LazyLock<Mutex<Option<*const *const UWorld>>> =
    LazyLock::new(|| Mutex::new(None));
static mut G_NAMES: LazyLock<Mutex<Option<*const FNamePool>>> = LazyLock::new(|| Mutex::new(None));
static mut G_OBJECTS: LazyLock<Mutex<Option<*const TUObjectArray>>> =
    LazyLock::new(|| Mutex::new(None));

pub fn init_sdk() {
    get_g_names();
    get_g_objects();
}

pub fn get_g_world() -> *const UWorld {
    let mut g_world_guard = match unsafe { &*G_WORLD }.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };

    unsafe {
        **g_world_guard.get_or_insert_with(|| {
            std::mem::transmute::<u64, *const *const UWorld>(
                memory::get_base_address() + offsets::GLOBAL_WORLD_OFFSET,
            )
        })
    }
}

pub fn get_g_names() -> *const FNamePool {
    let mut g_names_guard = match unsafe { &*G_NAMES }.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };

    *g_names_guard.get_or_insert_with(|| unsafe {
        std::mem::transmute::<u64, *const FNamePool>(
            memory::get_base_address() + offsets::GLOBAL_NAMES_OFFSET,
        )
    })
}

pub fn get_g_objects() -> *const TUObjectArray {
    let mut g_objects_guard = match unsafe { &*G_OBJECTS }.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };

    *g_objects_guard.get_or_insert_with(|| unsafe {
        std::mem::transmute::<u64, *const TUObjectArray>(
            memory::get_base_address() + offsets::GLOBAL_OBJECTS_OFFSET,
        )
    })
}
