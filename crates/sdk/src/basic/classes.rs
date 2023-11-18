//---------------------------------------------------------------------------------------------------------------------
// UE-4 BasicTypes Classes
//---------------------------------------------------------------------------------------------------------------------

use crate::basic::constants::NUM_ELEMENTS_PER_CHUNK;
use crate::core::classes::UObject;
use crate::get_g_names;

/// PredefinedClass BasicTypes.TArray
/// Size -> 0x0010 (FullSize[0x0010] - InheritedSize[0x0000])
#[repr(C)]
pub struct TArray<T> {
    pub data: *const T,
    // 0x00(0x08)
    pub count: u32,
    // 0x08(0x04)
    pub max: u32, // 0x0C(0x04)
}

impl<T: Clone> TArray<T> {
    pub fn new() -> Self {
        Self {
            data: std::ptr::null(),
            count: 0u32,
            max: 0u32,
        }
    }

    pub fn get(&self, index: u32) -> T {
        assert!(index < self.count, "Index out of bounds");
        unsafe {
            let result = self.data.add(index as usize);
            (*result).clone()
        }
    }

    pub fn len(&self) -> usize {
        self.count as usize
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    pub fn iter(&self) -> TArrayIterator<T> {
        TArrayIterator {
            array: self,
            index: 0,
        }
    }
}

pub struct TArrayIterator<'a, T: Clone> {
    array: &'a TArray<T>,
    // 0x00(0x08)
    index: u32, // 0x08(0x04)
}

impl<'a, T: Clone> Iterator for TArrayIterator<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.array.count {
            return None;
        }

        let result = self.array.get(self.index);
        self.index += 1;
        Some(result)
    }
}

/// PredefinedClass BasicTypes.FString
/// Size -> 0x0010 (FullSize[0x0010] - InheritedSize[0x0000])
#[repr(C)]
pub struct FString(TArray<u16>);

impl FString {
    pub fn new(string: &str) -> Self {
        let mut utf16: Vec<u16> = string.encode_utf16().collect();
        utf16.push(0); // null terminator
        Self(TArray {
            data: utf16.as_ptr(),
            count: utf16.len() as u32,
            max: utf16.len() as u32,
        })
    }
}

/// PredefinedClass BasicTypes.FUObjectItem
/// Size -> 0x0018 (FullSize[0x0018] - InheritedSize[0x0000])
#[repr(C)]
pub struct FUObjectItem {
    pub object: *const UObject, // 0x00(0x08)
    pub flags: i32, // 0x08(0x04)
    pub cluster_index: i32, // 0x0C(0x04)
    pub serial_number: i32, // 0x10(0x04)
    pad_14: [u8; 0x4], // 0x14(0x04)
}

/// PredefinedClass BasicTypes.TUObjectArray
/// Size -> 0x0020 (FullSize[0x0020] - InheritedSize[0x0000])
#[repr(C)]
pub struct TUObjectArray {
    objects: *const *const FUObjectItem, // 0x00(0x08)
    pre_allocated_objects: *const FUObjectItem, // 0x08(0x08)
    pub max_elements: i32, // 0x10(0x04)
    pub num_elements: i32, // 0x14(0x04)
    pub max_chunks: i32, // 0x18(0x04)
    pub num_chunks: i32, // 0x1C(0x04)
}

impl TUObjectArray {
    pub fn len(&self) -> usize {
        self.num_elements as usize
    }

    #[allow(dead_code)]
    pub fn max(&self) -> usize {
        self.max_elements as usize
    }

    pub fn is_valid_index(&self, index: i32) -> bool {
        index < self.num_elements && index < self.max_elements
    }

    pub fn get_by_index(&self, index: i32) -> *const UObject {
        if self.is_valid_index(index) {
            let chunk_index = index / NUM_ELEMENTS_PER_CHUNK;
            if chunk_index > self.num_chunks {
                return std::ptr::null();
            }

            let within_chunk_index = index % NUM_ELEMENTS_PER_CHUNK;
            unsafe {
                let chunk = *self.objects.add(chunk_index as usize);
                if !chunk.is_null() {
                    return *(chunk.add(within_chunk_index as usize) as *const *const UObject);
                }
            }
        }

        std::ptr::null()
    }
}

/// PredefinedClass BasicTypes.FName
/// Size -> 0x0000
#[repr(C)]
pub struct FName {
    pub comparison_index: i32, // 0x00(0x04)
    pub number: i32,           // 0x04(0x04)
}

impl FName {
    pub fn get_name(&self) -> String {
        let g_names = get_g_names();
        if !g_names.is_null() {
            let handle = FNameEntryHandle::index_to_handle(self.comparison_index as u32);
            let entry = unsafe { (*g_names).get_entry(handle) };
            let mut name = unsafe { (*entry).string() };
            if self.number > 0 {
                name.push_str(format!("_{}", self.number.to_string()).as_str());
            }
            if let Some(pos) = name.rfind('/') {
                name = name[pos + 1..].to_string();
            }

            return name;
        }
        String::new()
    }
}

/// PredefinedClass BasicTypes.FNamePool
/// Size -> 0x0000
#[repr(C)]
pub struct FNamePool {
    pub lock: [u8; 0x8],
    // 0x00(0x08)
    pub current_block: i32,
    // 0x08(0x04)
    pub current_byte_cursor: i32,
    // 0x0C(0x04)
    pub blocks: [*const u8; 8192], // 0x10(0x2000)
}

impl FNamePool {
    pub fn get_entry(&self, handle: FNameEntryHandle) -> *const FNameEntry {
        let block_ptr = self.blocks[handle.block as usize];
        let offset = block_ptr as u64 + (2 * handle.offset as u64);
        let entry = offset as *const FNameEntry;
        entry
    }
}

#[repr(C)]
union NameUnion {
    ansi_name: [u8; 1024],  // 0x00(0x400)
    wide_name: [u16; 1024], // 0x00(0x400)
}

/// PredefinedClass BasicTypes.FNameEntry
/// Size -> 0x0000
#[repr(C)]
pub struct FNameEntry {
    flags: u16,      // 0x00(0x02)
    name: NameUnion, // 0x02(0x400)
}

impl FNameEntry {
    fn is_wide(&self) -> bool {
        (self.flags & 0x1) != 0x0
    }

    fn len(&self) -> u16 {
        (self.flags >> 6) & 0x3FF
    }

    pub fn string(&self) -> String {
        if self.is_wide() {
            return String::new();
        }

        let name_bytes = unsafe { &self.name.ansi_name[..self.len() as usize] };
        String::from_utf8_lossy(name_bytes).to_string()
    }
}

#[repr(C)]
pub struct FNameEntryHandle {
    pub block: u32,  // 0x00(0x04)
    pub offset: u32, // 0x04(0x04)
}

impl FNameEntryHandle {
    fn new(block: u32, offset: u32) -> Self {
        Self { block, offset }
    }

    fn index_to_handle(index: u32) -> Self {
        let block = index >> 16;
        let offset = index & 65535;
        Self::new(block, offset)
    }
}
