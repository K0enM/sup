use std::mem::transmute_copy;
use std::os::raw::c_char;
use std::{
    ffi::{c_void, CString},
    ptr::NonNull,
};

// Types

type HModule = NonNull<*const c_void>;
type FarProc = NonNull<*const c_void>;

// Structs

#[derive(Debug)]
pub struct Library {
    handle: HModule,
}

// Implementations
extern "stdcall" {
    fn LoadLibraryA(name: *const c_char) -> Option<HModule>;
    fn GetProcAddress(module: HModule, name: *const c_char) -> Option<FarProc>;
}

impl Library {
    pub fn new(name: &str) -> Option<Self> {
        let name = CString::new(name).expect("Invalid library name");
        let res = unsafe { LoadLibraryA(name.as_ptr()) };
        res.map(|module| Library { handle: module })
    }

    pub fn get_proc<T>(&self, name: &str) -> Option<T> {
        let name = CString::new(name).expect("Invalid proc name");
        let res = unsafe { GetProcAddress(self.handle, name.as_ptr()) };
        res.map(|proc| unsafe { transmute_copy(&proc) })
    }
}
