#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

type ADL_Adapter_NumberOfAdapters_Get =
  unsafe extern "C" fn(lpNumAdapters: *mut i32) -> i32;

pub fn get_adapter_count() -> Result<i32, libloading::Error> {
  unsafe {
    let get_count: libloading::Symbol<'static, ADL_Adapter_NumberOfAdapters_Get> =
      adl_sys::get_adl_fn(b"ADL_Adapter_NumberOfAdapters_Get\0")?;

    let mut count = 0;
    let result = get_count(&mut count);

    if result == 0 {
      Ok(count)
    } else {
      Err(libloading::Error::DlSym {
        desc: std::ffi::CString::new(format!("ADL call failed with code {}", result))
          .unwrap()
          .as_c_str()
          .into(),
      })
    }
  }
}
