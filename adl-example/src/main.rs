fn main() {
  match adl::get_adapter_count() {
    Ok(count) => println!("Detected {} adapter(s).", count),
    Err(e) => eprintln!("Failed to get adapter count: {}", e),
  }
}
