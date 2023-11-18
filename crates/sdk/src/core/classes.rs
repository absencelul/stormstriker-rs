use crate::core::constants::PROCESS_EVENTS_INDEX;
use crate::basic::classes::FName;
use crate::get_g_objects;

//---------------------------------------------------------------------------------------------------------------------
// UE-4 CoreUObject Classes
//---------------------------------------------------------------------------------------------------------------------

/// Class CoreUObject.Field
/// Size -> 0x0008 (FullSize[0x0030] - InheritedSize[0x0028])
#[repr(C)]
pub struct UField {
    pub object_: UObject,    // 0x00(0x28)
    pub next: *const UField, // 0x28(0x08)
}

/// Class CoreUObject.Struct
/// Size -> 0x0080 (FullSize[0x00B0] - InheritedSize[0x0030])
#[repr(C)]
pub struct UStruct {
    pub field_: UField, // 0x00(0x30)
    pad_30: [u8; 0x10],
    // 0x30(0x10)
    pub super_field: *const UStruct,
    // 0x40(0x08)
    pad_48: [u8; 0x68], // 0x48(0x68)
}

/// Class CoreUObject.Class
/// Size -> 0x0180 (FullSize[0x0230] - InheritedSize[0x00B0])
#[repr(C)]
pub struct UClass {
    pub struct_: UStruct,
    // 0x00(0xB0)
    pad_b0: [u8; 0x180], // 0xb0(0x180)
}

/// Class CoreUObject.Function
/// Size -> 0x0030 (FullSize[0x00E0] - InheritedSize[0x00B0])
#[repr(C)]
pub struct UFunction {
    pub struct_: UStruct,                 // 0x00(0xB0)
    pub function_flags: i32,              // 0xB0(0x04)
    pub num_params: u8,                   // 0xB4(0x01)
    pub params_size: u16,                 // 0xB5(0x02)
    pad_b7: [u8; 0x1],                    // 0xB7(0x01)
    pub return_value_offset: u16,         // 0xB8(0x02)
    pub rpc_id: u16,                      // 0xBA(0x02)
    pub rpc_response_id: u16,             // 0xBC(0x02)
    pub pad_be: [u8; 0x02],               // 0xBE(0x02)
    first_property_to_init: *const usize, // 0xC0(0x08)
    event_graphic_function: *const usize, // 0xC8(0x08)
    event_graph_call_offset: i32,         // 0xD0(0x4)
    pad_d4: [u8; 0x4],                    // 0xD4(0x04)
    pub func: *const std::ffi::c_void,    // 0xD8(0x08)
}

/// Class CoreUObject.Object
/// Size -> 0x0028
#[repr(C)]
pub struct UObject {
    pub vf_table: *const *const u64,
    // 0x00(0x08)
    pub flags: i32,
    // 0x08(0x04)
    pub internal_index: i32,
    // 0x0C(0x04)
    pub class: *const UClass,
    // 0x10(0x08)
    pub name: FName,
    // 0x18(0x08)
    pub outer: *const UObject, // 0x20(0x08)
}

impl UObject {
    pub fn get_name(&self) -> String {
        self.name.get_name()
    }

    pub fn get_full_name(&self) -> String {
        let mut name = String::new();
        if !self.class.is_null() {
            let mut outer = self.outer;
            unsafe {
                while !outer.is_null() {
                    name = format!("{}.{}", (*outer).get_name(), name);
                    outer = (*outer).outer;
                }
            }

            name = format!(
                "{} {}",
                unsafe { (*self.class).struct_.field_.object_.get_name() },
                name
            );
            name.push_str(&self.get_name());
        }
        name
    }

    pub fn find_object<T: 'static>(name: &str) -> *const T {
        let g_objects = get_g_objects();
        if !g_objects.is_null() {
            unsafe {
                for i in 0..(*g_objects).len() {
                    let object = (*g_objects).get_by_index(i as i32);
                    if object.is_null() {
                        continue;
                    }

                    if (*object).get_full_name() == name {
                        return object as *const T;
                    }
                }
            }
        }
        std::ptr::null()
    }

    #[allow(dead_code)]
    pub fn find_objects<T: 'static>(name: &str) -> Vec<*const T> {
        let mut objects: Vec<*const T> = Vec::new();
        let g_objects = get_g_objects();
        if !g_objects.is_null() {
            unsafe {
                for i in 0..(*g_objects).len() as u32 {
                    let object = (*g_objects).get_by_index(i as i32);
                    if object.is_null() {
                        continue;
                    }

                    if (*object).get_full_name() == name {
                        objects.push(object as *const T);
                    }
                }
            }
        }

        objects
    }

    #[allow(dead_code)]
    pub fn find_class(name: &str) -> *const UClass {
        UObject::find_object::<UClass>(name)
    }

    #[allow(dead_code)]
    pub fn is_a(&self, cmp: &mut UClass) -> bool {
        let mut s = self.class as *mut UClass;
        while !s.is_null() {
            if s == cmp {
                return true;
            }
            s = unsafe { (*s).struct_.super_field as *mut UClass };
        }
        false
    }

    pub fn process_event(&self, function: *const UFunction, params: *mut usize) {
        type VTableFn = extern "C" fn(*const UObject, *const UFunction, *const usize);
        let self_ptr = self as *const _ as *const *const VTableFn;
        let vtable = unsafe { *self_ptr };
        let fn_call = unsafe { *vtable.add(PROCESS_EVENTS_INDEX) };
        fn_call(self, function, params);
    }
}
