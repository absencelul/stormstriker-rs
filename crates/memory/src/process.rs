use log::debug;
use std::ffi::CString;
use std::path::PathBuf;
use windows::{
    core::{
        imp::{GetProcAddress, WaitForSingleObject},
        Result, PCSTR,
    },
    Win32::{
        Foundation::{CloseHandle, HANDLE, INVALID_HANDLE_VALUE, MAX_PATH},
        System::{
            Diagnostics::{
                Debug::WriteProcessMemory,
                ToolHelp::{
                    CreateToolhelp32Snapshot, Module32First, Process32First, Process32Next,
                    MODULEENTRY32, PROCESSENTRY32, TH32CS_SNAPMODULE, TH32CS_SNAPMODULE32,
                    TH32CS_SNAPPROCESS,
                },
            },
            LibraryLoader::GetModuleHandleA,
            Memory::{
                VirtualAllocEx, VirtualFreeEx, MEM_COMMIT, MEM_RELEASE, MEM_RESERVE, PAGE_READWRITE,
            },
            Threading::{
                CreateRemoteThread, GetExitCodeThread, OpenProcess, INFINITE, PROCESS_ALL_ACCESS,
            },
        },
    },
};

use crate::util::read_null_terminated_string;

pub struct Process {
    pub name: String,
    pub id: u32,
    pub handle: HANDLE,
}

impl Process {
    pub fn new(name: String, id: u32, handle: HANDLE) -> Self {
        Self { name, id, handle }
    }

    #[inline]
    pub fn by_name(name: &str) -> Option<Self> {
        if let Some(pid) = get_process_id(name) {
            if let Some(handle) = get_process_handle(pid) {
                return Some(Self::new(name.to_string(), pid, handle));
            }
        }
        None
    }

    #[inline]
    pub fn base_address(&self) -> Option<usize> {
        if self.id > 0 {
            if let Ok(snapshot) = unsafe {
                CreateToolhelp32Snapshot(TH32CS_SNAPMODULE | TH32CS_SNAPMODULE32, self.id)
            } {
                if snapshot != INVALID_HANDLE_VALUE {
                    let mut module_entry = MODULEENTRY32 {
                        dwSize: std::mem::size_of::<MODULEENTRY32>() as u32,
                        ..Default::default()
                    };
                    unsafe {
                        if Module32First(snapshot, &mut module_entry).is_ok() {
                            return Some(module_entry.modBaseAddr as usize);
                        }
                        CloseHandle(snapshot).unwrap();
                    }
                }
            }
        }
        None
    }

    pub fn eject(&self, base_address: *const u8) -> Result<()> {
        let kernel32 = CString::new("Kernel32").unwrap();
        let load_library = CString::new("LoadLibraryW").unwrap();
        let proc_address = unsafe {
            GetProcAddress(
                GetModuleHandleA(PCSTR(kernel32.as_ptr() as _)).unwrap().0,
                PCSTR(load_library.as_ptr() as _).0,
            )
        };
        let thread = unsafe {
            CreateRemoteThread(
                self.handle,
                None,
                0,
                Some(std::mem::transmute(proc_address)),
                Some(base_address as _),
                0,
                None,
            )
        }?;

        unsafe {
            WaitForSingleObject(thread.0, INFINITE);
            CloseHandle(thread)?;

            Ok(())
        }
    }

    pub fn inject(&self, dll_path: PathBuf) -> Result<()> {
        let kernel32 = CString::new("Kernel32").unwrap();
        let load_library = CString::new("LoadLibraryW").unwrap();
        let proc_address = unsafe {
            GetProcAddress(
                GetModuleHandleA(PCSTR(kernel32.as_ptr() as _)).unwrap().0,
                PCSTR(load_library.as_ptr() as _).0,
            )
        };
        let dll_path = CString::new(dll_path.to_str().unwrap()).unwrap();
        let dll_path_buf = unsafe {
            VirtualAllocEx(
                self.handle,
                None,
                (MAX_PATH as usize) * std::mem::size_of::<u16>(),
                MEM_RESERVE | MEM_COMMIT,
                PAGE_READWRITE,
            )
        };
        let mut bytes_written = 0usize;
        let response = unsafe {
            WriteProcessMemory(
                self.handle,
                dll_path_buf,
                dll_path.as_ptr() as _,
                (MAX_PATH as usize) * std::mem::size_of::<u16>(),
                Some((&mut bytes_written) as *mut _),
            )
        };

        debug!(
            "WriteProcessMemory: written {} bytes, returned OK: {}",
            bytes_written,
            response.is_ok()
        );

        let thread = unsafe {
            CreateRemoteThread(
                self.handle,
                None,
                0,
                Some(std::mem::transmute(proc_address)),
                Some(dll_path_buf),
                0,
                None,
            )
        }?;

        unsafe {
            WaitForSingleObject(thread.0, INFINITE);
            let mut exit_code = 0u32;
            GetExitCodeThread(thread, &mut exit_code as *mut u32)?;
            CloseHandle(thread)?;
            VirtualFreeEx(self.handle, dll_path_buf, 0, MEM_RELEASE)?;

            Ok(())
        }
    }
}

pub fn get_process_id(name: &str) -> Option<u32> {
    if cfg!(target_arch = "x86_64") {
        get_process_id_x64(name)
    } else if cfg!(target_arch = "x86") {
        panic!("Unsupported architecture")
    } else {
        panic!("Unsupported architecture")
    }
}

fn get_process_id_x64(name: &str) -> Option<u32> {
    if let Ok(snapshot) = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) } {
        if snapshot == INVALID_HANDLE_VALUE {
            return None;
        }

        let mut process_entry = PROCESSENTRY32 {
            dwSize: std::mem::size_of::<PROCESSENTRY32>() as u32,
            ..Default::default()
        };

        unsafe {
            if let Err(_e) = Process32First(snapshot, &mut process_entry) {
                CloseHandle(snapshot).unwrap();
                return None;
            }

            if Process32First(snapshot, &mut process_entry).is_ok() {
                if let Some(p_name) =
                    read_null_terminated_string(process_entry.szExeFile.as_ptr() as usize)
                {
                    if p_name == name {
                        return Some(process_entry.th32ProcessID);
                    }
                }
                while Process32Next(snapshot, &mut process_entry).is_ok() {
                    if let Some(p_name) =
                        read_null_terminated_string(process_entry.szExeFile.as_ptr() as usize)
                    {
                        if p_name == name {
                            return Some(process_entry.th32ProcessID);
                        }
                    }
                }
            }
            CloseHandle(snapshot).unwrap();
        }
    }

    None
}

fn get_process_id_x86(_name: &str) -> Option<u32> {
    None
}

fn get_process_handle(id: u32) -> Option<HANDLE> {
    match unsafe { OpenProcess(PROCESS_ALL_ACCESS, false, id) } {
        Ok(handle) => Some(handle),
        Err(_) => None,
    }
}
