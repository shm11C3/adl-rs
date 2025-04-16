fn main() {
  match adl::adapter::get_adapter_count() {
    Ok(count) => println!("Detected {} adapter(s).", count),
    Err(e) => eprintln!("Failed to get adapter count: {}", e),
  }

  match adl::adapter::get_all_adapter_info() {
    Ok(adapters) => {
      for adapter in adapters {
        println!(
          "Adapter {}: {} ({}), Present: {}, Bus Number: {}, Vendor ID: {}",
          match adl::adapter::get_adapter_id(adapter.index) {
            Ok(id) => id,
            Err(e) => {
              eprintln!("Failed to get adapter ID: {}", e);
              continue;
            }
          },
          adapter.name,
          adapter.display_name,
          adapter.present,
          adapter.bus_number,
          adapter.vendor_id
        );

        let chipset = match adl::adapter::get_chipset_info(adapter.index) {
          Ok(chipset) => chipset,
          Err(e) => {
            eprintln!("Failed to get chipset info: {}", e);
            continue;
          }
        };

        println!(
          "  - Bus: PCIe Gen{} x{} (Max x{}), BusType: {}",
          chipset.bus_speed_type,
          chipset.current_pcie_lane_width,
          chipset.max_pcie_lane_width,
          chipset.bus_type
        );

        let memory = match adl::adapter::get_memory_info(adapter.index) {
          Ok(memory) => memory,
          Err(e) => {
            eprintln!("Failed to get memory info: {}", e);
            continue;
          }
        };

        println!(
          "  - size_bytes: {} B  bit_rate_x2_mbps {} mbps, Memory Type: {}",
          memory.size_bytes, memory.bit_rate_x2_mbps, memory.memory_type
        );

        let vram_usage = match adl::adapter::get_vram_usage(adapter.index) {
          Ok(usage) => usage,
          Err(e) => {
            eprintln!("Failed to get VRAM usage: {}", e);
            continue;
          }
        };
        println!("  - VRAM Usage: {} MB", vram_usage);

        let vbios_info = match adl::adapter::get_vbios_info(adapter.index) {
          Ok(info) => info,
          Err(e) => {
            eprintln!("Failed to get VBIOS info: {}", e);
            continue;
          }
        };
        println!(
          "  - VBIOS Version: {}",
          unsafe { std::ffi::CStr::from_ptr(vbios_info.version.as_ptr() as *const i8) }
            .to_string_lossy()
            .into_owned()
        );
      }
    }
    Err(e) => eprintln!("Failed to get adapter info: {}", e),
  }

  match adl::performance::get_pm_log_data(0) {
    Ok(log_data) => {
      for (name, value) in log_data {
        println!("PM Log Data - {}: {}", name, value);
      }
    }
    Err(e) => eprintln!("Failed to get PM log data: {}", e),
  }
}
