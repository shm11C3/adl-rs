use libc;
use std::ffi::c_void;
use std::ptr;

use crate::adapter::types;
use adl_sys;

type ADLContextHandle = *mut std::ffi::c_void;

///
/// Get the number of adapters available.
///
/// # Returns
///
/// * `Ok(i32)` - The number of adapters.
/// * `Err(String)` - An error message if the function fails.
///
pub fn get_adapter_count() -> Result<i32, String> {
  unsafe {
    let func = adl_sys::get_adl_fn::<unsafe extern "C" fn(*mut i32) -> i32>(
      b"ADL_Adapter_NumberOfAdapters_Get\0",
    )
    .map_err(|e| format!("Failed to load function: {}", e))?;

    let mut count: i32 = 0;
    let result = func(&mut count);
    if result == 0 {
      Ok(count)
    } else {
      Err(format!(
        "ADL_Adapter_NumberOfAdapters_Get failed with code {}",
        result
      ))
    }
  }
}

unsafe extern "C" fn adl_malloc(size: i32) -> *mut c_void {
  libc::malloc(size as usize)
}

unsafe extern "C" fn adl_free(ptr: *mut c_void) {
  libc::free(ptr)
}

///
/// Get information about all adapters.
///
/// # Returns
///
/// * `Ok(Vec<AdapterInfo>)` - A vector of `AdapterInfo` structs.
/// * `Err(String)` - An error message if the function fails.
///
pub fn get_all_adapter_info() -> Result<Vec<types::AdapterInfo>, String> {
  unsafe {
    let create_fn = adl_sys::get_adl_fn::<
      unsafe extern "stdcall" fn(
        Option<unsafe extern "C" fn(i32) -> *mut c_void>,
        i32,
        *mut ADLContextHandle,
      ) -> i32,
    >(b"ADL2_Main_Control_Create\0")
    .map_err(|e| format!("Failed to load ADL2_Main_Control_Create: {}", e))?;

    let mut context: ADLContextHandle = ptr::null_mut();

    let result = create_fn(Some(adl_malloc), 1, &mut context);
    if result != 0 || context.is_null() {
      return Err(format!(
        "ADL2_Adapter_AdapterInfoX4_Get failed (expected 1, got {})",
        result
      ));
    }

    let mut ptr: *mut types::AdapterInfoX2 = ptr::null_mut();
    let mut count: i32 = 0;

    let get_info_fn = adl_sys::get_adl_fn::<
      unsafe extern "stdcall" fn(
        ADLContextHandle,
        i32,
        *mut i32,
        *mut *mut types::AdapterInfoX2,
      ) -> i32,
    >(b"ADL2_Adapter_AdapterInfoX4_Get\0")
    .map_err(|e| format!("Failed to load AdapterInfoX4_Get: {}", e))?;

    let result = get_info_fn(context, -1, &mut count, &mut ptr);

    // NOTE: ADL2_Adapter_AdapterInfoX4_Get may return 0 even when it provides valid data.
    // See: https://gpuopen-librariesandsdks.github.io/adl/group__ADAPTERAPI.html
    if result != 1 {
      eprintln!(
          "Warning: ADL2_Adapter_AdapterInfoX4_Get returned {}, but data was returned anyway.",
          result
      );
    }

    let slice = std::slice::from_raw_parts(ptr, count as usize);
    let adapters = slice.iter().map(types::convert_adapter_info).collect();

    // Free the buffer
    adl_free(ptr as *mut c_void);

    // Destroy context
    let destroy_fn = adl_sys::get_adl_fn::<
      unsafe extern "stdcall" fn(ADLContextHandle) -> i32,
    >(b"ADL2_Main_Control_Destroy\0")
    .map_err(|e| format!("Failed to load ADL2_Main_Control_Destroy: {}", e))?;

    destroy_fn(context);

    Ok(adapters)
  }
}
