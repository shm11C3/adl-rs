use libc;
use std::ffi::c_void;
use std::ptr;

pub type ADLContextHandle = *mut c_void;

unsafe extern "C" fn adl_malloc(size: i32) -> *mut c_void {
  libc::malloc(size as usize)
}

pub struct ADLContext {
  handle: ADLContextHandle,
}

impl ADLContext {
  pub fn new() -> Result<Self, String> {
    unsafe {
      let create_fn = adl_sys::get_adl_fn::<
        unsafe extern "stdcall" fn(
          Option<unsafe extern "C" fn(i32) -> *mut c_void>,
          i32,
          *mut ADLContextHandle,
        ) -> i32,
      >(b"ADL2_Main_Control_Create\0")
      .map_err(|e| format!("Failed to load ADL2_Main_Control_Create: {}", e))?;

      let mut handle: ADLContextHandle = ptr::null_mut();
      let result = create_fn(Some(adl_malloc), 1, &mut handle);

      if result != 0 || handle.is_null() {
        return Err(format!(
          "ADL2_Main_Control_Create failed with code {}",
          result
        ));
      }

      Ok(Self { handle })
    }
  }

  pub fn handle(&self) -> ADLContextHandle {
    self.handle
  }
}

impl Drop for ADLContext {
  fn drop(&mut self) {
    unsafe {
      if let Ok(destroy_fn) = adl_sys::get_adl_fn::<
        unsafe extern "stdcall" fn(ADLContextHandle) -> i32,
      >(b"ADL2_Main_Control_Destroy\0")
      {
        destroy_fn(self.handle);
      }
    }
  }
}
