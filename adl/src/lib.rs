pub fn initialize() -> Result<(), i32> {
  let result = unsafe { sys::ADL_Main_Control_Create(None, 1) };

  if result == 0 { Ok(()) } else { Err(result) }
}
