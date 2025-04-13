use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
  println!("cargo:rerun-if-changed=adl_sdk/adl_sdk.h");

  let builder = bindgen::Builder::default()
    .header("include/adl_sdk.h")
    .clang_arg("-Iinclude")
    .clang_arg("-IC:/Program Files/Microsoft Visual Studio/2022/Community/VC/Tools/MSVC/14.40.33807/include")
    .clang_arg("-IC:/Program Files (x86)/Windows Kits/10/Include/10.0.22621.0/ucrt")
    .clang_arg("-IC:/Program Files (x86)/Windows Kits/10/Include/10.0.22621.0/shared")
    .clang_arg("-IC:/Program Files (x86)/Windows Kits/10/Include/10.0.22621.0/um")
    .clang_arg("-include")
    .clang_arg("include/stdbool_shim.h")
    .clang_arg("-fms-compatibility")
    .clang_arg("-fms-extensions")
    .clang_arg("-fdeclspec")
    .clang_arg("-include")
    .clang_arg("include/wchar_shim.h")
    .clang_arg("-DADL_ENABLE_THREAD_SAFE")
    .clang_arg("-DADL_EXPOSED")
    .parse_callbacks(Box::new(bindgen::CargoCallbacks::default()));

  match builder.generate() {
    Ok(bindings) => {
      let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
      let out_file = out_path.join("bindings.rs");

      println!("Generating bindings to: {}", out_file.display());
      bindings
        .write_to_file(&out_file)
        .expect("Couldn't write bindings!");

      bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

      // Copy the generated bindings to src/bindings.rs for development
      let dev_bindings_path = PathBuf::from("src").join("bindings.rs");
      fs::copy(&out_path.join("bindings.rs"), &dev_bindings_path)
        .expect("Failed to copy bindings to src/bindings.rs");
    }
    Err(e) => {
      eprintln!("Unable to generate bindings: {e:?}");
      std::process::exit(1);
    }
  }
}
