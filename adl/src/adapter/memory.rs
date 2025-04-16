use crate::adapter::types::{convert_memory_info_x4, ADLMemoryInfo};
use crate::context::ADLContext;

///
/// Get memory information for a specific adapter.
///
/// # Arguments
///
/// * `context` - The ADL context.
/// * `index` - The index of the adapter.
///
/// # Returns
///
/// * `Ok(MemoryInfo)` - The memory information.
/// * `Err(String)` - An error message if the function fails.
///
pub fn get_memory_info(index: i32) -> Result<ADLMemoryInfo, String> {
  unsafe {
    let context = ADLContext::new()?;

    let func = adl_sys::get_adl_fn::<
      unsafe extern "stdcall" fn(
        *mut std::ffi::c_void,
        i32,
        *mut adl_sys::ADLMemoryInfoX4,
      ) -> i32,
    >(b"ADL2_Adapter_MemoryInfoX4_Get\0")
    .map_err(|e| format!("Failed to load ADL2_Adapter_MemoryInfoX4_Get: {}", e))?;

    let mut raw: adl_sys::ADLMemoryInfoX4 = std::mem::zeroed();
    let result = func(context.handle(), index, &mut raw);

    if result != 0 {
      return Err(format!(
        "ADL2_Adapter_MemoryInfoX4_Get failed with code {}",
        result
      ));
    }

    Ok(convert_memory_info_x4(&raw))
  }
}

///
/// Get the VRAM usage for a specific adapter.
///
/// # Arguments
///
/// * `index` - The index of the adapter.
///
/// # Returns
///
/// * `Ok(i32)` - The VRAM usage in MB.
/// * `Err(String)` - An error message if the function fails.
///
pub fn get_vram_usage(index: i32) -> Result<i32, String> {
  let context = ADLContext::new()?;

  unsafe {
    let func = adl_sys::get_adl_fn::<
      unsafe extern "stdcall" fn(*mut std::ffi::c_void, i32, *mut i32) -> i32,
    >(b"ADL2_Adapter_VRAMUsage_Get\0")
    .map_err(|e| format!("Failed to load ADL2_Adapter_VRAMUsage_Get: {}", e))?;

    let mut usage_mb = 0i32;
    let result = func(context.handle(), index, &mut usage_mb);

    if result != 0 {
      return Err(format!(
        "ADL2_Adapter_VRAMUsage_Get failed with code {}",
        result
      ));
    }

    Ok(usage_mb)
  }
}
