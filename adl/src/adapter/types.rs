use std::ffi::CStr;

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct AdapterInfoX2 {
  pub iSize: i32,
  pub iAdapterIndex: i32,
  pub strUDID: [i8; 256],
  pub iBusNumber: i32,
  pub iDeviceNumber: i32,
  pub iFunctionNumber: i32,
  pub iVendorID: i32,
  pub strAdapterName: [i8; 256],
  pub strDisplayName: [i8; 256],
  pub iPresent: i32,
  pub iExist: i32,
  pub strDriverPath: [i8; 256],
  pub strDriverPathExt: [i8; 256],
  pub strPNPString: [i8; 256],
  pub iOSDisplayIndex: i32,
  pub iInfoMask: i32,
  pub iInfoValue: i32,
}

#[derive(Debug, Clone)]
pub struct AdapterInfo {
  pub index: i32,
  pub name: String,
  pub display_name: String,
  pub present: bool,
  pub bus_number: i32,
  pub vendor_id: i32,
}

pub fn convert_adapter_info(raw: &AdapterInfoX2) -> AdapterInfo {
  AdapterInfo {
    index: raw.iAdapterIndex,
    name: unsafe { CStr::from_ptr(raw.strAdapterName.as_ptr()) }
      .to_string_lossy()
      .into_owned(),
    display_name: unsafe { CStr::from_ptr(raw.strDisplayName.as_ptr()) }
      .to_string_lossy()
      .into_owned(),
    present: raw.iPresent != 0,
    bus_number: raw.iBusNumber,
    vendor_id: raw.iVendorID,
  }
}
