use crate::context::ADLContext;

pub fn get_gpu_activity_percent(
  index: i32,
) -> Result<adl_sys::ADLODNPerformanceStatus, String> {
  let context = ADLContext::new()?;

  unsafe {
    let func = adl_sys::get_adl_fn::<
      unsafe extern "stdcall" fn(
        *mut std::ffi::c_void,
        i32,
        *mut adl_sys::ADLODNPerformanceStatus,
      ) -> i32,
    >(b"ADL2_OverdriveN_PerformanceStatus_Get\0")
    .map_err(|e| {
      format!(
        "Failed to load ADL2_OverdriveN_PerformanceStatus_Get: {}",
        e
      )
    })?;

    let mut status: adl_sys::ADLODNPerformanceStatus = std::mem::zeroed();

    let result = func(context.handle(), index, &mut status);
    if result != 0 {
      return Err(format!(
        "ADL2_OverdriveN_PerformanceStatus_Get failed with code {}",
        result
      ));
    }

    Ok(status)
  }
}
