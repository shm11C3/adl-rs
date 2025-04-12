use std::ffi::c_void;
use std::ptr::null_mut;

pub struct AdlContext {
  pub(crate) handle: *mut c_void,
}

impl AdlContext {
  pub fn new() -> Result<Self, i32> {
    let mut handle: *mut c_void = null_mut();

    let result = unsafe { sys::ADL2_Main_Control_Create(None, 1, &mut handle) };

    if result == 0 {
      Ok(Self { handle })
    } else {
      Err(result)
    }
  }

  pub fn handle(&self) -> *mut c_void {
    self.handle
  }
}

impl Drop for AdlContext {
  fn drop(&mut self) {
    let _ = unsafe { sys::ADL2_Main_Control_Destroy(self.handle) };
  }
}
