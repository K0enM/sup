use std::ffi::c_void;

type HModule = *const c_void;

pub struct Library {
  handle: HModule,
}

