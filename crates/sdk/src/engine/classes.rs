use crate::basic::classes::{FName, FString, TArray};
use std::sync::OnceLock;

use crate::core::classes::{UClass, UFunction, UObject};

//---------------------------------------------------------------------------------------------------------------------
// UE-4 Engine Classes
//---------------------------------------------------------------------------------------------------------------------

/// Class Engine.Actor
/// Size -> 0x0268 (FullSize[0x0290] - InheritedSize[0x0028])
#[repr(C)]
pub struct AActor {
    pub object_: UObject, // 0x00(0x28)
    pad_28: [u8; 0x268],  // 0x28(0x268)
}

static GET_DISTANCE_TO: OnceLock<usize> = OnceLock::new();

impl AActor {
    pub fn get_distance_to(&self, actor: *const AActor) -> f32 {
        GET_DISTANCE_TO.get_or_init(|| {
            UObject::find_object::<usize>("Function Engine.Actor.GetDistanceTo") as usize
        });

        #[repr(C)]
        pub struct Params {
            other_actor: *const AActor,
            return_val: f32,
        }

        let mut params = Params {
            other_actor: actor,
            return_val: 0f32,
        };

        let func = *GET_DISTANCE_TO.get().unwrap() as *const UFunction;
        self.object_
            .process_event(func, &mut params as *mut _ as *mut usize);

        params.return_val
    }
}

/// Class Engine.Canvas
/// Size -> 0x0368 (FullSize[0x0390] - InheritedSize[0x0028])
#[repr(C)]
pub struct UCanvas {
    pub object_: UObject, // 0x00(0x28)
    pad_28: [u8; 0x368],  // 0x28(0x368)
}

/// Class Engine.HUD
/// Size -> 0x00F0 (FullSize[0x0380] - InheritedSize[0x0290])
#[repr(C)]
pub struct AHUD {
    pub actor_: AActor,                             // 0x00(0x290)
    pub player_owner: *const APlayerController,     // 0x0290(0x0008)
    pad_298: [u8; 0x1],                             // 0x0298(0x0001)
    pad_299: [u8; 0x3],                             // 0x0299(0x0003)
    pub current_target_index: i32,                  // 0x029C(0x0004)
    pad_2a0: [u8; 0x1],                             // 0x02A0(0x0001)
    pad_2a1: [u8; 0x7],                             // 0x02A1(0x0007)
    post_rendered_actors: TArray<*const AActor>,    // 0x02A8(0x0010)
    pad_2b8: [u8; 0x8],                             // 0x02B8(0x0008)
    debug_display: TArray<FName>,                   // 0x02C0(0x0010)
    toggled_debug_categories: TArray<FName>,        // 0x02D0(0x0010)
    pub canvas: *const UCanvas,                     // 0x02E0(0x0008)
    debug_canvas: *const UCanvas,                   // 0x02E8(0x0008)
    debug_text_list: [u8; 0x10], // 0x02F0(0x0010) TArray<FDebugTextInfo> debug_text_list
    show_debug_target_desired_class: *const UClass, // 0x300(0x8)
    show_debug_target_actor: *const AActor, // 0x0308(0x0008)
    pad_310: [u8; 0x70],         // 0x310(0x70)
}

/// Class Engine.Player
/// Size -> 0x0020 (FullSize[0x0048] - InheritedSize[0x0028])
#[repr(C)]
pub struct UPlayer {
    pub object_: UObject,                            // 0x00(0x28)
    pad_28: [u8; 0x8],                               // 0x28(0x8)
    pub player_controller: *const APlayerController, // 0x30(0x8)
    current_net_speed: i32,                          // 0x38(0x4)
    configured_internet_speed: i32,                  // 0x3C(0x4)
    configured_lan_speed: i32,                       // 0x40(0x4)
    pad_44: [u8; 0x4],                               // 0x44(0x4)
}

/// Class Engine.Controller
/// Size -> 0x0098 (FullSize[0x0328] - InheritedSize[0x0290])
#[repr(C)]
pub struct AController {
    pub actor_: AActor,  // 0x00(0x290)
    pad_290: [u8; 0x98], // 0x290(0x98)
}

/// Class Engine.Pawn
/// Size -> 0x0060 (FullSize[0x0280] - InheritedSize[0x0220])
#[repr(C)]
pub struct APawn {
    pub actor_: AActor,  // 0x00(0x220)
    pad_220: [u8; 0x60], // 0x220(0x60)
}

/// Class Engine.PlayerController
/// Size -> 0x0528 (FullSize[0x0850] - InheritedSize[0x0328])
#[repr(C)]
pub struct APlayerController {
    pub controller_: AController,               // 0x00(0x328)
    pad_328: [u8; 0x8],                         // 0x328(0x8)
    pub player: *const UPlayer,                 // 0x330(0x8)
    pub acknowledged_pawn: *const APawn,        // 0x338(0x8)
    pub my_hud: *const AHUD,                    // 0x340(0x8)
    pub player_camera_manager: *const usize, // 0x348(0x8) APlayerCameraManager* player_camera_manager
    player_camera_manager_class: *const UClass, // 0x350(0x8)
    pad_358: [u8; 0x4f8],                    // 0x358(0x4f8)
}

/// Class Engine.LocalPlayer
/// Size -> 0x0250 (FullSize[0x0298] - InheritedSize[0x0048])
#[repr(C)]
pub struct ULocalPlayer {
    pub player_: UPlayer,                                 // 0x00(0x48)
    pad_48: [u8; 0x30],                                   // 0x48(0x30)
    pub viewport_client: *const UGameViewportClient,      // 0x78(0x8)
    pad_80: [u8; 0x38],                                   // 0x80(0x38)
    aspect_ratio_axis_constraint: [u8; 0x1],              // 0xb8(0x1)
    pad_b9: [u8; 0x7],                                    // 0xb9(0x7)
    pending_level_player_controller_class: *const UClass, // 0xc0(0x8)
    sent_split_join: [u8; 0x1],                           // 0xc8(0x1)
    pad_c9: [u8; 0x17],                                   // 0xc9(0x17)
    controller_id: i32,                                   // 0xE0(0x4)
    pad_bc: [u8; 0x1b4],                                  // 0xe4(0x1b4)
}

/// Class Engine.GameInstance
/// Size -> 0x0198 (FullSize[0x01C0] - InheritedSize[0x0028])
#[repr(C)]
pub struct UGameInstance {
    pub object_: UObject,                             // 0x00(0x28)
    pad_28: [u8; 0x10],                               // 0x28(0x10)
    pub local_players: TArray<*const ULocalPlayer>,   // 0x38(0x10)
    online_session: *const usize,                     // 0x48(0x8) UOnlineSession* online_session
    referenced_objects: TArray<*const UObject>,       // 0x50(0x10)
    pad_60: [u8; 0x18],                               // 0x60(0x18)
    on_pawn_controller_changed_delegates: [u8; 0x10], // 0x78(0x10) FScriptMulticastDelegate on_pawn_controller_changed_delegates
    pad_88: [u8; 0x18],                               // 0x88(0x18)
    on_input_device_connection_change: [u8; 0x10], // 0xa0(0x10) FScriptMulticastDelegate on_input_device_connection_change
    on_user_input_device_pairing_change: [u8; 0x10], // 0xb0(0x10) FScriptMulticastDelegate on_user_input_device_pairing_change
    pad_c0: [u8; 0x100],                             // 0xc0(0x100)
}

/// Class Engine.BlueprintFunctionLibrary
/// Size -> 0x0000 (FullSize[0x0028] - InheritedSize[0x0028])
#[repr(C)]
pub struct UBlueprintFunctionLibrary {
    pub object_: UObject, // 0x00(0x28)
}

/// Class Engine.GameplayStatics
/// Size -> 0x0000 (FullSize[0x0028] - InheritedSize[0x0028])
#[repr(C)]
pub struct UGameplayStatics {
    pub blueprint_function_library_: UBlueprintFunctionLibrary, // 0x00(0x28)
}

impl UGameplayStatics {
    #[allow(dead_code)]
    pub fn spawn_object(&self, class: *const UClass, outer: *const UObject) -> *const UObject {
        static mut FUNC: *mut UFunction = std::ptr::null_mut();
        struct SpawnObjectParams {
            class: *const UClass,
            outer: *const UObject,
            return_val: *const UObject,
        }

        unsafe {
            if FUNC.is_null() {
                FUNC = UObject::find_object::<UFunction>(
                    "Function Engine.GameplayStatics.SpawnObject",
                ) as *mut UFunction;
                if FUNC.is_null() {
                    println!("[-] Failed to find SpawnObject function");
                    return std::ptr::null();
                }
            }
            let mut params = SpawnObjectParams {
                class,
                outer,
                return_val: std::ptr::null_mut(),
            };
            let flags = (*FUNC).function_flags;
            self.blueprint_function_library_
                .object_
                .process_event(FUNC, &mut params as *mut _ as *mut usize);
            (*FUNC).function_flags = flags;
            params.return_val
        }
    }
}

/// Class Engine.Console
/// Size -> 0x0108 (FullSize[0x0130] - InheritedSize[0x0028])
#[repr(C)]
pub struct UConsole {
    pub object_: UObject,                           // 0x00(0x28)
    pad_28: [u8; 0x10],                             // 0x28(0x10)
    pub console_target_player: *const ULocalPlayer, // 0x38(0x8)
    default_texture_black: *const u64,              // 0x40(0x8) UTexture2D* default_texture_black
    default_texture_white: *const u64,              // 0x48(0x8) UTexture2D* default_texture_white
    pad_50: [u8; 0x18],                             // 0x50(0x18)
    history_buffer: TArray<FString>,                // 0x68(0x10) TArray<FString> history_buffer
    pad_78: [u8; 0xb8],                             // 0x78(0xb8)
}

/// Class Engine.ScriptViewportClient
/// Size -> 0x0010 (FullSize[0x0038] - InheritedSize[0x0028])
#[repr(C)]
pub struct UScriptViewportClient {
    pub object_: UObject,
    // 0x00(0x28)
    pad_28: [u8; 0x10], // 0x28(0x10)
}

/// Class Engine.GameViewportClient
/// Size -> 0x0378 (FullSize[0x03B0] - InheritedSize[0x0038])
#[repr(C)]
pub struct UGameViewportClient {
    pub script_viewport_client_: UScriptViewportClient, // 0x00(0x38)
    pad_38: [u8; 0x8],                                  // 0x38(0x8)
    pub viewport_console: *const UConsole,              // 0x40(0x8)
    debug_properties: [u8; 0x10], // 0x48(0x10) TArray<FDebugDisplayProperty> debug_properties
    pad_58: [u8; 0x10],           // 0x58(0x10)
    max_split_screen_players: i32, // 0x68(0x4)
    pad_6c: [u8; 0xC],            // 0x6c(0xC)
    pub world: *const UWorld,     // 0x78(0x8)
    pub game_instance: *const UGameInstance, // 0x80(0x8)
    pad_88: [u8; 0x328],          // 0x88(0x328)
}

/// Class Engine.Level
/// Size -> 0x02F0 (FullSize[0x0318] - InheritedSize[0x0028])
#[repr(C)]
pub struct ULevel {
    pub object_: UObject,                                      // 0x00(0x28)
    pad_28: [u8; 0x70],                                        // 0x28(0x70)
    pub actors: TArray<*const AActor>,                         // 0x98(0x10)
    pub garbage_actors: TArray<*const AActor>,                 // 0xa8(0x10)
    pub owning_world: *const UWorld,                           // 0xb8(0x8)
    model: *const usize,                                       // 0xc0(0x8) UModel* model
    model_components: TArray<*const usize>, // 0xc8(0x10) TArray<UModelComponent> model_components
    actor_cluster: *const usize,            // 0xd8(0x8) ULevelActorContainer* actor_cluster
    num_texture_streaming_un_built_components: i32, // 0xe0(0x4)
    num_texture_streaming_dirty_resources: i32, // 0xe4(0x4)
    level_script_actor: *const usize,       // 0xe8(0x8) ALevelScriptActor* level_script_actor
    nav_list_start: *const usize,           // 0xf0(0x8) ANavigationObjectBase* nav_list_start
    nav_list_end: *const usize,             // 0xf8(0x8) ANavigationObjectBase* nav_list_end
    nav_data_chunks: TArray<*const usize>, // 0x100(0x10) TArray<UNavigationDataChunk*> nav_data_chunks
    light_map_total_size: f32,             // 0x110(0x4)
    shadow_map_total_size: f32,            // 0x114(0x4)
    static_navigable_geometry: [u8; 0x10], // 0x118(0x10) TArray<FVector> static_navigable_geometry
    streaming_texture_guids: [u8; 0x10],   // 0x128(0x10) TArray<FGuid> streaming_texture_guids
    streaming_textures: TArray<FName>,     // 0x138(0x98)
    packed_texture_streaming_quality_level_feature_level: u32, // 0x148(0x4)
    pad_14c: [u8; 0xc4],                   // 0x14c(0xc4)
    level_build_data_id: [u8; 0x10],       // 0x210(0x10) FGuid level_build_data_id
    map_build_data: *const usize,          // 0x220(0x8) UMapBuildDataRegistry* map_build_data
    light_build_level_offset: [u8; 0xC],   // 0x228(0xC) FIntVector light_build_level_offset
    pad_234: [u8; 0x1],                    // 0x234(0x1)
    pad_235: [u8; 0x1],                    // 0x235(0x1)
    pad_236: [u8; 0x1],                    // 0x236(0x1)
    pad_237: [u8; 0x61],                   // 0x237(0x63)
    world_settings: *const usize,          // 0x298(0x8) AWorldSettings* world_settings
    world_data_layers: *const usize,       // 0x2a0(0x8)
    world_partition_runtime_cell: [u8; 0x30], // 0x2a8(0x30) TArray<FWorldPartitionRuntimeCell> world_partition_runtime_cell
    pad_2d8: [u8; 0x10],                      // 0x2d8(0x10)
    asset_user_data: TArray<*const usize>,    // 0x2e0(0x10) TArray<UAssetUserData*> asset_user_data
    pad_2f0: [u8; 0x10],                      // 0x2f0(0x10)
    destroyed_replicated_static_actors: [u8; 0x10], // 0x300(0x10) TArray<FReplicatedStaticActorDestructionInfo> destroyed_replicated_static_actors
    pad_310: [u8; 0x8],                             // 0x310(0x8)
}

/// Class Engine.World
/// Size -> 0x0870 (FullSize[0x0898] - InheritedSize[0x0028])
#[repr(C)]
pub struct UWorld {
    pub object_: UObject,                                           // 0x00(0x28)
    pad_28: [u8; 0x8],                                              // 0x28(0x8)
    pub persistent_level: *const ULevel,                            // 0x30(0x8)
    net_driver: *const usize,   // 0x38(0x8) UNetDriver* net_driver
    line_batcher: *const usize, // 0x40(0x8) ULineBatchComponent* line_batcher
    persistent_line_batcher: *const usize, // 0x48(0x8) ULineBatchComponent* persistent_line_batcher
    foreground_line_batcher: *const usize, // 0x50(0x8) ULineBatchComponent* foreground_line_batcher
    network_manager: *const usize,         // 0x58(0x8) AGameNetworkManager* network_manager
    physics_collision_handler: *const usize, // 0x60(0x8) UPhysicsCollisionHandler* physics_collision_handler
    extra_referenced_objects: TArray<*const UObject>, // 0x68(0x10)
    per_module_data_objects: TArray<*const UObject>, // 0x78(0x10)
    streaming_levels: TArray<*const usize>,  // 0x88(0x10) TArray<ULevelStreaming> streaming_levels
    streaming_levels_to_consider: [u8; 0x28], // 0x98(0x28) FStreamingLevelsToConsider streaming_levels_to_consider
    streaming_levels_visibility: *const usize, // 0xC0(0x08) AServerStreamingLevelsVisibility* streaming_levels_visibility
    streaming_levels_prefix: FString,         // 0xC8(0x10)
    pad_d8: [u8; 0x8],                        // 0xD8(0x8)
    current_level_pending_visibility: *const ULevel, // 0xE0(0x8)
    current_level_pending_invisibility: *const ULevel, // 0xE8(0x8)
    demo_net_driver: *const usize,            // 0xF0(0x8) UDemoNetDriver* demo_net_driver
    my_particle_event_manager: *const usize, // 0xF8(0x8) AParticleEventManager* my_particle_event_manager
    default_physics_volume: *const usize,    // 0x100(0x8) APhysicsVolume* default_physics_volume
    pad_108: [u8; 0x36],                     // 0x108(0x36)
    pad_13e: [u8; 0x1],                      // 0x13e(0x1)
    pad_13f: [u8; 0x9],                      // 0x13f(0x9)
    navigation_system: *const usize,         // 0x148(0x8) UNavigationSystemBase* navigation_system
    authority_game_mode: *const usize,       // 0x150(0x8) AGameModeBase* authority_game_mode
    game_state: *const usize,                // 0x158(0x8) AGameStateBase* game_state
    ai_system: *const usize,                 // 0x160(0x8) UAISystemBase* ai_system
    avoidance_manager: *const usize,         // 0x168(0x8) UAvoidanceManager* avoidance_manager
    pub levels: TArray<*const ULevel>,       // 0x170(0x10)
    level_collections: [u8; 10], // 0x180(0x10) TArray<FLevelCollection> level_collections
    pad_190: [u8; 0x28],         // 0x190(0x28)
    pub owning_game_instance: *const UGameInstance, // 0x1b8(0x8)
    parameter_collection_instances: [u8; 0x10], // 0x1c0(0x10)
    canvas_for_rendering_to_target: *const UCanvas, // 0x1d0(0x8)
    canvas_for_draw_material_to_render_target: *const UCanvas, // 0x1d8(0x8)
    pad_1a8: [u8; 0x70],         // 0x1e0(0x70)
    physics_field: *const usize, // 0x250(0x8) UPhysicsFieldComponent* physics_field
    lwi_fast_assigned_uid: u32,  // 0x258(0x4)
    pad_25c: [u8; 0x4],          // 0x25c(0x4)
    components_that_need_pre_end_of_frame_sync: [u8; 0x50], // 0x260(0x50)
    components_that_need_end_of_frame_update: TArray<*const usize>, // 0x2b0(0x10)
    components_that_need_end_of_frame_update_on_game_thread: TArray<*const usize>, // 0x2c0(0x10)
    pad_2d0: [u8; 0x3f8],        // 0x2d0(0x3f8)
    world_composition: *const usize, // 0x6c8(0x8) UWorldComposition* world_composition
    content_bundle_manager: *const usize, // 0x6d0(0x8) UContentBundleManager* content_bundle_manager
    pad_6d8: [u8; 0xa8],                  // 0x6d8(0x90)
    psc_pool: [u8; 0x58],                 // 0x780(0x58) FWorldPSCPool
    pad_7d8: [u8; 0xc0],                  // 0x7d0(0xc0)
}

/// Class Engine.Font
/// Size -> 0x01A8 (FullSize[0x01D0] - InheritedSize[0x0028])
#[repr(C)]
pub struct UFont {
    pub object_: UObject,
    // 0x00(0x28)
    pad_28: [u8; 0x180], // 0x28(0x180)
}

impl UFont {
    pub fn get_font() -> *const UFont {
        UObject::find_object::<UFont>("Font Roboto.Roboto")
    }
}
