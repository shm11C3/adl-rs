use crate::adapter::types;
use adl_sys;
use std::mem;

pub fn get_chipset_info(index: i32) -> Result<types::ADLChipSetInfo, String> {
  unsafe {
    let func = adl_sys::get_adl_fn::<
      unsafe extern "stdcall" fn(i32, *mut adl_sys::ADLChipSetInfo) -> i32,
    >(b"ADL_Adapter_ChipSetInfo_Get\0")
    .map_err(|e| format!("Failed to load ADL_Adapter_ChipSetInfo_Get: {}", e))?;

    let mut raw: adl_sys::ADLChipSetInfo = mem::zeroed();

    let result = func(index, &mut raw);
    if result != 0 {
      return Err(format!(
        "ADL_Adapter_ChipSetInfo_Get failed with code {}",
        result
      ));
    }

    Ok(types::convert_chipset_info(&raw))
  }
}
