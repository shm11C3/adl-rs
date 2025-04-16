use crate::adapter::context::ADLContext;
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
