use crate::cache::actors::update_player_cache;
use retour::static_detour;
use sdk::core::classes::UObject;
use sdk::engine::classes::{UCanvas, UGameViewportClient};
use sdk::get_g_world;

type FnPostRender =
    unsafe extern "fastcall" fn(viewport: *const UGameViewportClient, canvas: *const UCanvas);

static_detour! {
    static PostRender: unsafe extern "fastcall" fn(
        *const UGameViewportClient,
        *const UCanvas);
}

fn hk_post_render(viewport: *const UGameViewportClient, canvas: *const UCanvas) {
    unsafe {
        println!("[-] PostRender");

        let g_world = get_g_world();
        if g_world.is_null() {
            println!("[-] GWorld is null");
            return;
        }

        let level = (*g_world).persistent_level;
        if level.is_null() {
            println!("[-] Level is null");
            return;
        }

        let actors = unsafe { &(*level).actors };
        if actors.is_empty() || actors.data.is_null() {
            println!("[-] Actors is empty or data is null");
            return;
        }

        let actors = actors.iter().collect::<Vec<_>>();

        update_player_cache(actors);

        PostRender.call(viewport, canvas);
    }
}

pub const POST_RENDER_INDEX: usize = 0x6d;

pub(crate) fn hook_post_render(object: &UObject) -> bool {
    let vf_table = object.vf_table;
    unsafe {
        let address = *vf_table.add(POST_RENDER_INDEX);

        let fn_post_render: FnPostRender = std::mem::transmute(address as *const usize);
        PostRender
            .initialize(fn_post_render, hk_post_render)
            .unwrap()
            .enable()
            .unwrap();
        fn_post_render as u64 > 0
    }
}

pub(crate) fn unhook_post_render() {
    if PostRender.is_enabled() {
        unsafe {
            PostRender.disable().unwrap();
        }
    }
}
