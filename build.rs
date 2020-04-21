use std::process::Command;

#[cfg(unix)]
fn set_link_path() {
  let mut build_dir = std::env::current_dir().unwrap();
  build_dir.push("build");

  println!("cargo:rustc-link-search=native={}", build_dir.display());
}

#[cfg(not(unix))]
fn set_link_path() { }

fn download() -> Result<(), String> {
  Err("no downloads available".to_string())
}

#[cfg(not(windows))]
fn build() {
  let mut build_path = std::env::current_dir().unwrap();
  build_path.push("build");

  if !build_path.is_dir() {
    std::fs::create_dir(build_path.clone());
  }

  let mut build_type = "Release";
  if cfg!(debug_assertions) {
    build_type = "Debug"
  }

  Command::new("cmake")
          .arg(format!("-DCMAKE_BUILD_TYPE={}", build_type))
          .arg("..")
          .current_dir(build_path.clone())
          .status()
          .expect("failed to execute 'cmake ..'");

  Command::new("make")
          .current_dir(build_path.clone())
          .status()
          .expect("failed to execute 'make'");
}

#[cfg(windows)]
fn build() {
  let mut build_path = std::env::current_dir().unwrap();
  build_path.push("build");

  if !build_path.is_dir() {
    std::fs::create_dir(build_path.clone());
  }

  let mut build_type = "Release";
  if cfg!(debug_assertions) {
    build_type = "Debug"
  }

  Command::new("cmake")
          .arg(format!("-DCMAKE_BUILD_TYPE={}", build_type))
          .arg("..")
          .current_dir(build_path.clone())
          .status()
          .expect("failed to execute 'cmake ..'");

  Command::new("cmake")
          .arg("--build")
          .arg(".")
          .arg("--config")
          .arg(build_type)
          .current_dir(build_path.clone())
          .status()
          .expect("failed to execute 'make'");
}

fn download_or_build() {
  let res = download();
  if res.is_err() {
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    println!("cargo:warning=[webrtc {}] no downloads available - building from source, rerun with -vv to view build output", VERSION);
    build();
  }
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
fn copy_library() {
  let mut lib_file = std::env::current_dir().unwrap();
  lib_file.push("build");
  lib_file.push("libwebrtc-rs.dylib");

  let mut out_file_debug = std::env::current_dir().unwrap();
  out_file_debug.push("target");
  out_file_debug.push("debug");

  if out_file_debug.is_dir() {
    out_file_debug.push("libwebrtc-rs.dylib");

    std::fs::copy(lib_file.clone(), out_file_debug);
  }

  let mut out_file_release = std::env::current_dir().unwrap();
  out_file_release.push("target");
  out_file_release.push("release");

  if out_file_release.is_dir() {
    out_file_release.push("libwebrtc-rs.dylib");

    std::fs::copy(lib_file, out_file_release);
  }
}

#[cfg(all(not(windows), not(any(target_os = "macos", target_os = "ios"))))]
fn copy_library() {
  let mut lib_file = std::env::current_dir().unwrap();
  lib_file.push("build");
  lib_file.push("libwebrtc-rs.so");

  let mut out_file_debug = std::env::current_dir().unwrap();
  out_file_debug.push("target");
  out_file_debug.push("debug");

  if out_file_debug.is_dir() {
    out_file_debug.push("libwebrtc-rs.so");

    std::fs::copy(lib_file.clone(), out_file_debug);
  }

  let mut out_file_release = std::env::current_dir().unwrap();
  out_file_release.push("target");
  out_file_release.push("release");

  if out_file_release.is_dir() {
    out_file_release.push("libwebrtc-rs.so");

    std::fs::copy(lib_file, out_file_release);
  }
}

fn main() {
  // println!("cargo:rerun-if-changed=build.rs");

  set_link_path();

  download_or_build();

  copy_library();
}
