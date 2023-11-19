use crate::cache::GenericCache;
use sdk::engine::classes::AActor;
use std::sync::{LazyLock, Mutex};

pub static mut PLAYER_CACHE: LazyLock<Mutex<GenericCache<*const AActor>>> =
    LazyLock::new(|| Mutex::new(GenericCache::new()));

pub fn update_player_cache(actors: Vec<*const AActor>) {
    let mut cache = unsafe { &*PLAYER_CACHE }.lock().unwrap();
    cache.update(actors, validate_player);
}

pub fn validate_player(actor: *const AActor) -> bool {
    if actor.is_null() {
        println!("actor is null");
        return false;
    }

    // need to check if actor is a player
    // and other checks
    true
}
