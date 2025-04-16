use crate::context::ADLContext;
use std::mem::zeroed;

pub fn get_pm_log_data(index: i32) -> Result<Vec<(u32, u32)>, String> {
  let context = ADLContext::new()?;

  unsafe {
    let func = adl_sys::get_adl_fn::<
      unsafe extern "stdcall" fn(
        *mut std::ffi::c_void,
        i32,
        *mut adl_sys::ADLPMLogData,
      ) -> i32,
    >(b"ADL2_New_QueryPMLogData_Get\0")
    .map_err(|e| format!("Failed to load ADL2_New_QueryPMLogData_Get: {}", e))?;

    let mut raw: adl_sys::ADLPMLogData = zeroed();
    raw.ulVersion = size_of::<adl_sys::ADLPMLogData>() as u32;

    let result = func(context.handle(), index, &mut raw);
    if result != 0 {
      return Err(format!(
        "ADL2_New_QueryPMLogData_Get failed with code {}",
        result
      ));
    }

    let mut values = Vec::new();
    for i in 0..adl_sys::ADL_PM_LOG_MAX {
      let [sensor_id, value] = raw.ulValues[i];
      if sensor_id != 0 {
        values.push((sensor_id, value));
      }
    }

    Ok(values)
  }
}

pub fn get_gpu_activity(index: i32) -> Result<adl_sys::ADLPMActivity, String> {
  unsafe {
    let func = adl_sys::get_adl_fn::<
      unsafe extern "stdcall" fn(i32, *mut adl_sys::ADLPMActivity) -> i32,
    >(b"ADL_PM_CurrentActivity_Get\0")
    .map_err(|e| format!("Failed to load ADL_PM_CurrentActivity_Get: {}", e))?;

    let mut activity: adl_sys::ADLPMActivity = zeroed();
    activity.iSize = size_of::<adl_sys::ADLPMActivity>() as i32;

    let result = func(index, &mut activity);
    if result != 0 {
      return Err(format!(
        "ADL_PM_CurrentActivity_Get failed with code {}",
        result
      ));
    }

    Ok(activity)
  }
}
