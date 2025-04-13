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

fn describe_bus_type(bus_type: i32) -> &'static str {
  match bus_type {
    1 => "PCIe",
    2 => "AGP",
    3 => "PCI",
    _ => "Unknown",
  }
}

#[derive(Debug, Clone)]
pub struct ADLChipSetInfo {
  pub bus_type: String,
  pub bus_speed_type: i32,
  pub max_pcie_lane_width: i32,
  pub current_pcie_lane_width: i32,
  pub supported_agp_speeds: i32,
  pub current_agp_speed: i32,
}

pub fn convert_chipset_info(raw: &adl_sys::ADLChipSetInfo) -> ADLChipSetInfo {
  ADLChipSetInfo {
    bus_type: describe_bus_type(raw.iBusType).to_string(),
    bus_speed_type: raw.iBusSpeedType,
    max_pcie_lane_width: raw.iMaxPCIELaneWidth,
    current_pcie_lane_width: raw.iCurrentPCIELaneWidth,
    supported_agp_speeds: raw.iSupportedAGPSpeeds,
    current_agp_speed: raw.iCurrentAGPSpeed,
  }
}
