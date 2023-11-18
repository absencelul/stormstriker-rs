use crate::hooks::post_render::{hook_post_render, unhook_post_render};
use sdk::{get_g_objects, get_g_world};

mod post_render;

pub fn unhook_all() {
    unhook_post_render();
}

pub fn initialize_hooks() -> Result<(), &'static str> {
    let g_world = get_g_world();
    let g_objects = get_g_objects();

    while g_world.is_null() || g_objects.is_null() {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    println!("[-] GWorld: 0x{:X}", g_world as u64);

    let game = unsafe { (*g_world).owning_game_instance };
    if game.is_null() {
        return Err("[-] UGameInstance is null");
    }

    let players = unsafe { &(*game).local_players };
    if players.len() == 0 {
        return Err("[-] TArray<*const ULocalPlayer> is empty");
    }

    let local_player = players.get(0);
    if local_player.is_null() {
        return Err("[-] ULocalPlayer is null");
    }

    let viewport = unsafe { (*local_player).viewport_client };
    if viewport.is_null() {
        return Err("[-] UGameViewportClient is null");
    }

    println!("[-] Hooking PostRender");
    if !hook_post_render(unsafe { &(*viewport).script_viewport_client_.object_ }) {
        return Err("[-] Failed to hook PostRender");
    }

    Ok(())
}
