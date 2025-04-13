#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

//#[cfg(all(windows, target_pointer_width = "32"))]
//const LIBRARY_NAME: &str = "atiadlxx";
//#[cfg(all(windows, target_pointer_width = "64"))]
//const LIBRARY_NAME: &str = "atiadlxy";
#[cfg(all(windows))]
const LIBRARY_NAME: &str = "atiadlxx";
#[cfg(unix)]
const LIBRARY_NAME: &str = "libatiadlxx.so";

mod bindings;
pub use bindings::*;

use libloading::{Library, Symbol};
use once_cell::unsync::OnceCell;

unsafe extern "C" fn ADL_Main_Memory_Alloc(size: i32) -> *mut std::ffi::c_void {
  std::alloc::alloc(std::alloc::Layout::from_size_align(size as usize, 8).unwrap())
    as *mut std::ffi::c_void
}

thread_local! {
  static LIB_INSTANCE: OnceCell<Library> = OnceCell::new();
}

pub unsafe fn get_adl_fn<T>(
  fn_name: &[u8],
) -> Result<Symbol<'static, T>, libloading::Error>
where
  T: Sized,
{
  LIB_INSTANCE
    .try_with(|cell| {
      let lib: &Library = cell.get_or_try_init(|| {
        let lib = Library::new(LIBRARY_NAME)?;
        let init_fn: Symbol<unsafe extern "C" fn(ADL_MAIN_MALLOC_CALLBACK, i32) -> i32> =
          unsafe { lib.get(b"ADL_Main_Control_Create\0")? };

        if unsafe { init_fn(Some(ADL_Main_Memory_Alloc), 1) } != 0 {
          panic!("Failed to initialize ADL");
        }

        Ok(lib)
      })?;

      let symbol = unsafe {
        std::mem::transmute::<Symbol<T>, Symbol<'static, T>>(lib.get(fn_name)?)
      };
      Ok(symbol)
    })
    .map_err(|_| libloading::Error::DlOpen {
      desc: std::ffi::CString::new("Thread-local access error")
        .unwrap()
        .as_c_str()
        .into(),
    })?
}
