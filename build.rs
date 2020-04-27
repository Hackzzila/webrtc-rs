use std::process::Command;

fn main() {
  Command::new("python")
          .arg("tools/build.py")
          .arg("downloadOrBuild")
          .status()
          .expect("failed to execute build script");
}
