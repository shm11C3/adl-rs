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

#[derive(Debug, Clone)]
pub struct ADLMemoryInfo {
  pub size_bytes: i64,
  pub memory_type: String,
  pub bandwidth_mb_s: i64,
  pub hyper_memory_bytes: i64,
  pub invisible_memory_bytes: i64,
  pub visible_memory_bytes: i64,
  pub vram_vendor_rev_id: i64,
  pub bandwidth_x2_mb_s: i64,
  pub bit_rate_x2_mbps: i64,
}

pub fn convert_memory_info_x4(raw: &adl_sys::ADLMemoryInfoX4) -> ADLMemoryInfo {
  ADLMemoryInfo {
    size_bytes: raw.iMemorySize,
    memory_type: unsafe {
      std::ffi::CStr::from_ptr(raw.strMemoryType.as_ptr())
        .to_string_lossy()
        .into_owned()
    },
    bandwidth_mb_s: raw.iMemoryBandwidth,
    hyper_memory_bytes: raw.iHyperMemorySize,
    invisible_memory_bytes: raw.iInvisibleMemorySize,
    visible_memory_bytes: raw.iVisibleMemorySize,
    vram_vendor_rev_id: raw.iVramVendorRevId,
    bandwidth_x2_mb_s: raw.iMemoryBandwidthX2,
    bit_rate_x2_mbps: raw.iMemoryBitRateX2,
  }
}
