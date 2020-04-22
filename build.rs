use std::process::Command;
// use serde::{Deserialize};
// use flate2::read::GzDecoder;
// use tar::Archive;

// #[cfg(not(windows))]
// fn set_link_path() {
//   let mut build_dir = std::env::current_dir().unwrap();
//   build_dir.push("build");

//   println!("cargo:rustc-link-search=native={}", build_dir.display());
// }

// #[cfg(windows)]
// fn set_link_path() {
//   let mut build_dir = std::env::current_dir().unwrap();
//   build_dir.push("build");

//   let mut profile = std::env::var("PROFILE").unwrap();
//   profile.make_ascii_lowercase();

//   let mut build_type = "Release";
//   if profile == "debug" {
//     build_type = "Debug";
//   }

//   build_dir.push(build_type);

//   println!("cargo:rustc-link-search=native={}", build_dir.display());
// }

// #[derive(Deserialize, Debug)]
// struct GitHubAsset {
//   name: String,
//   browser_download_url: String,
// }

// #[derive(Deserialize, Debug)]
// struct GitHubRelease {
//   tag_name: String,
//   assets: Vec<GitHubAsset>,
// }

// fn download_and_unpack(client: reqwest::blocking::Client, url: String) -> Result<(), String> {
//   let res = client.get(&url)
//     .header(reqwest::header::USER_AGENT, "webrtc-rs builder")
//     .send().unwrap();

//   let tar = GzDecoder::new(res);
//   let mut archive = Archive::new(tar);
//   match archive.unpack("build") {
//     Ok(_) => Ok(()),
//     Err(_) => Err("error unpacking .tar.gz file".to_string()),
//   }
// }

// fn download() -> Result<(), String> {
//   let client = reqwest::blocking::Client::new();
//   let res = client.get("https://api.github.com/repos/Hackzzila/rust-webrtc/releases")
//     .header(reqwest::header::USER_AGENT, "webrtc-rs builder")
//     .send().unwrap();
//   let result = res.json::<Vec<GitHubRelease>>();

//   let version_str = format!("v{}", env!("CARGO_PKG_VERSION"));
//   let target = std::env::var("TARGET").unwrap();
//   let asset_str = format!("{}.tar.gz", target);

//   for release in result.unwrap() {
//     if release.tag_name == version_str {
//       for asset in release.assets {
//         if asset.name == asset_str {
//           return download_and_unpack(client, asset.browser_download_url);
//         }
//       }
//     }
//   }

//   Err("no download available".to_string())
// }

// #[cfg(not(windows))]
// fn build() {
//   let mut build_path = std::env::current_dir().unwrap();
//   build_path.push("build");

//   if !build_path.is_dir() {
//     std::fs::create_dir(build_path.clone());
//   }

//   let mut profile = std::env::var("PROFILE").unwrap();
//   profile.make_ascii_lowercase();

//   let mut build_type = "Release";
//   if profile == "debug" {
//     build_type = "Debug";
//   }

//   Command::new("cmake")
//           .arg(format!("-DCMAKE_BUILD_TYPE={}", build_type))
//           .arg("..")
//           .current_dir(build_path.clone())
//           .status()
//           .expect("failed to execute 'cmake ..'");

//   Command::new("make")
//           .current_dir(build_path.clone())
//           .status()
//           .expect("failed to execute 'make'");
// }

// #[cfg(windows)]
// fn build() {
//   let mut build_path = std::env::current_dir().unwrap();
//   build_path.push("build");

//   if !build_path.is_dir() {
//     std::fs::create_dir(build_path.clone());
//   }

//   let mut profile = std::env::var("PROFILE").unwrap();
//   profile.make_ascii_lowercase();

//   let mut build_type = "Release";
//   if profile == "debug" {
//     build_type = "Debug";
//   }

//   Command::new("python")
//           .arg("tools/build.py")
//           .current_dir(build_path.clone())
//           .status()
//           .expect("failed to execute 'python tools/build.py'");

//   Command::new("cmake")
//           .arg(format!("-DCMAKE_BUILD_TYPE={}", build_type))
//           .arg("..")
//           .current_dir(build_path.clone())
//           .status()
//           .expect("failed to execute 'cmake ..'");

//   Command::new("cmake")
//           .arg("--build")
//           .arg(".")
//           .arg("--config")
//           .arg(build_type)
//           .current_dir(build_path.clone())
//           .status()
//           .expect("failed to execute 'cmake --build .'");
// }

// #[cfg(any(target_os = "macos", target_os = "ios"))]
// fn copy_library() {
//   let mut lib_file = std::env::current_dir().unwrap();
//   lib_file.push("build");
//   lib_file.push("libwebrtc-rs.dylib");

//   let mut out_file = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
//   out_file.pop();
//   out_file.pop();
//   out_file.pop();

//   if out_file.is_dir() {
//     out_file.push("libwebrtc-rs.dylib");

//     std::fs::copy(lib_file.clone(), out_file);
//   }
// }

// #[cfg(all(not(windows), not(any(target_os = "macos", target_os = "ios"))))]
// fn copy_library() {
//   let mut lib_file = std::env::current_dir().unwrap();
//   lib_file.push("build");
//   lib_file.push("libwebrtc-rs.so");

//   let mut out_file = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
//   out_file.pop();
//   out_file.pop();
//   out_file.pop();

//   if out_file.is_dir() {
//     out_file.push("libwebrtc-rs.so");

//     std::fs::copy(lib_file.clone(), out_file);
//   }
// }

// #[cfg(windows)]
// fn copy_library() {
//   let mut dll_file = std::env::current_dir().unwrap();
//   dll_file.push("build");

//   let mut profile = std::env::var("PROFILE").unwrap();
//   profile.make_ascii_lowercase();

//   let mut build_type = "Release";
//   if profile == "debug" {
//     build_type = "Debug";
//   }

//   dll_file.push(build_type);

//   let mut pdb_file = dll_file.clone();

//   dll_file.push("webrtc-rs.dll");
//   pdb_file.push("webrtc-rs.pdb");

//   let mut out_dll_file = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
//   out_dll_file.pop();
//   out_dll_file.pop();
//   out_dll_file.pop();

//   if out_dll_file.is_dir() {
//     if build_type == "Debug" {
//       let mut out_pdb_file = out_dll_file.clone();
//       out_pdb_file.push("webrtc-rs.pdb");

//       std::fs::copy(pdb_file.clone(), out_pdb_file);
//     }

//     out_dll_file.push("webrtc-rs.dll");

//     std::fs::copy(dll_file.clone(), out_dll_file);
//   }
// }

fn main() {
  Command::new("python")
          .arg("tools/build.py")
          .arg("downloadOrBuild")
          .status()
          .expect("failed to execute build script");

  // println!("cargo:rerun-if-changed=build.rs");

  // set_link_path();

  // let res = download();
  // if res.is_err() {
  //   println!("cargo:warning=[webrtc {}] no downloads available - building from source, rerun with -vv to view build output", env!("CARGO_PKG_VERSION"));
  //   build();
  // }

  // copy_library();
}
