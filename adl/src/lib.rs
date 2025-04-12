use std::ffi::c_void;
use std::ptr::null_mut;

pub fn initialize() -> Result<*mut c_void, i32> {
  let mut context: *mut c_void = null_mut();

  let result = unsafe { sys::ADL2_Main_Control_Create(None, 1, &mut context) };

  if result == 0 {
    Ok(context)
  } else {
    Err(result)
  }
}
