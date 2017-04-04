use std::{env,fs};
use std::path::Path;
use std::process::Command;

fn main() {
  // TODO: Clean this all up. There has to be a prettier way.
  let target = env::var("TARGET").expect("TARGET required");
  let manifest_dir_str = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR required");
  let version = "0.5.0";

  let root = Path::new(manifest_dir_str.as_str());

//   let lib_root_str = env::var("HELIX_LIB_DIR").unwrap_or(manifest_dir_str.clone());
//   let lib_root = Path::new(lib_root_str.as_str());
  let lib_root = root;

  // Best way I could find to tell if we're packaging vs just building
  let is_packaging = root.parent().expect("root has no parent").ends_with("target/package");
  let libname32 = format!("helix-runtime-{}.i386", version.replace(".", "-"));
  let libname64 = format!("helix-runtime-{}.x86_64", version.replace(".", "-"));
  let libname = if target.starts_with("x86_64") { libname64.clone() } else { libname32.clone() };

  // Not required for non-Windows, but it needs to be part of the package
  if is_packaging && (!root.join(format!("{}.lib", libname32)).exists() ||
                      !root.join(format!("{}.lib", libname64)).exists()) {
    panic!("{}.lib and {}.lib must exist when packaging. Please run ./prepackage.sh", libname32, libname64);
  }

  if target.contains("windows") && !lib_root.join(format!("{}.lib", libname)).exists() {
    panic!("{}.lib must exist when running. Set HELIX_LIB_DIR to ruby/windows_build for development.", libname);
  }

  if target.contains("windows") {
    let out_dir_str = env::var("OUT_DIR").expect("OUT_DIR required");

    let out_dir = Path::new(out_dir_str.as_str());

    // Read info about current Ruby install
    let raw_ruby_info = Command::new("ruby")
                                .arg(root.join("ruby_info.rb"))
                                .output()
                                .expect("failed to get Ruby info");
    let raw_ruby_output = String::from_utf8_lossy(&raw_ruby_info.stdout);
    let mut raw_ruby_lines = raw_ruby_output.lines();
    let ruby_libdir = Path::new(raw_ruby_lines.next().expect("Ruby info has no libdir"));
    let libruby = raw_ruby_lines.next().expect("Ruby info has no LIBRUBY");
    let libruby_so = raw_ruby_lines.next().expect("Ruby info has no LIBRUBY_SO");
    if raw_ruby_lines.next() != None {
      panic!("Unexpected information returned in Ruby info");
    }

    let ruby_libname = libruby_so.split('.').next().expect("can't extract Ruby lib name");

    // Copy .dll.a file to .lib since Rust msvc looks for .lib files only
    fs::copy(ruby_libdir.join(libruby), out_dir.join(ruby_libname).with_extension("lib"))
        .expect("unable to copy libruby");

    // Set up linker
    println!("cargo:rustc-flags=-L {libpath} -l dylib={libruby} -L {root} -l helix-runtime:{libname}",
              libpath=out_dir.to_str().expect("can't get str from out_dir"),
              libruby=ruby_libname,
              root=lib_root.to_str().expect("can't get str from root dir"),
              libname=libname);
  }
}

