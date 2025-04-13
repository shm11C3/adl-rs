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
      }
    }
    Err(e) => eprintln!("Failed to get adapter info: {}", e),
  }
}
