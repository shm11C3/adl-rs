use crate::adapter::{context::ADLContext, types};
use adl_sys;
use libc;
use std::ffi::c_void;
use std::ptr;

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
    let context = ADLContext::new()?;

    let get_fn = adl_sys::get_adl_fn::<
      unsafe extern "stdcall" fn(
        *mut c_void,
        i32,
        *mut i32,
        *mut *mut types::AdapterInfoX2,
      ) -> i32,
    >(b"ADL2_Adapter_AdapterInfoX4_Get\0")
    .map_err(|e| format!("Failed to load AdapterInfoX4_Get: {}", e))?;

    let mut ptr: *mut types::AdapterInfoX2 = ptr::null_mut();
    let mut count: i32 = 0;
    let result = get_fn(context.handle(), -1, &mut count, &mut ptr);

    if ptr.is_null() || count <= 0 {
      return Err(format!(
        "ADL2_Adapter_AdapterInfoX4_Get failed: result = {}, count = {}, ptr = {:?}",
        result, count, ptr
      ));
    }

    // NOTE: ADL2_Adapter_AdapterInfoX4_Get may return 0 even when it provides valid data.
    // See: https://gpuopen-librariesandsdks.github.io/adl/group__ADAPTERAPI.html
    if result != 1 {
      eprintln!(
        "Warning: ADL2_Adapter_AdapterInfoX4_Get returned {}, but data seems usable.",
        result
      );
    }

    let slice = std::slice::from_raw_parts(ptr, count as usize);
    let adapters = slice.iter().map(types::convert_adapter_info).collect();

    adl_free(ptr as *mut c_void);

    Ok(adapters)
  }
}

/// Get the ADL adapter ID for a given adapter index.
///
/// # Arguments
/// * `index` - The adapter index (0-based)
///
/// # Returns
/// * `Ok(i32)` - The adapter ID
/// * `Err(String)` - If the call fails
pub fn get_adapter_id(index: i32) -> Result<i32, String> {
  unsafe {
    let func = adl_sys::get_adl_fn::<unsafe extern "stdcall" fn(i32, *mut i32) -> i32>(
      b"ADL_Adapter_ID_Get\0",
    )
    .map_err(|e| format!("Failed to load ADL_Adapter_ID_Get: {}", e))?;

    let mut id: i32 = 0;
    let result = func(index, &mut id);
    if result == 0 {
      Ok(id)
    } else {
      Err(format!("ADL_Adapter_ID_Get failed with code {}", result))
    }
  }
}

///
/// Get chipset information for a specific adapter.
///
/// # Arguments
///
/// * `index` - The index of the adapter.
///
/// # Returns
///
/// * `Ok(ChipSetInfo)` - The chipset information.
/// * `Err(String)` - An error message if the function fails.
///
pub fn get_vbios_info(index: i32) -> Result<types::ADLBiosInfo, String> {
  let context = ADLContext::new()?;

  unsafe {
    let func = adl_sys::get_adl_fn::<
      unsafe extern "stdcall" fn(
        *mut std::ffi::c_void,
        i32,
        *mut adl_sys::ADLBiosInfo,
      ) -> i32,
    >(b"ADL2_Adapter_VideoBiosInfo_Get\0")
    .map_err(|e| format!("Failed to load ADL2_Adapter_VideoBiosInfo_Get: {}", e))?;

    let mut raw: adl_sys::ADLBiosInfo = std::mem::zeroed();

    let result = func(context.handle(), index, &mut raw);
    if result != 0 {
      return Err(format!(
        "ADL2_Adapter_VideoBiosInfo_Get failed with code {}",
        result
      ));
    }

    Ok(types::convert_vbios_info(&raw))
  }
}
