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
      }
    }
    Err(e) => eprintln!("Failed to get adapter info: {}", e),
  }
}
