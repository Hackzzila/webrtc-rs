#[derive(Clone, Debug)]
#[repr(C)]
pub enum LogLevel {
  Verbose,
  Info,
  Warning,
  Error,
  None,
}

#[link(name = "webrtc-rs")]
extern {
  fn webrtc_rs_set_log_level(level: LogLevel);
}

pub fn set_log_level(level: LogLevel) {
  unsafe {
    webrtc_rs_set_log_level(level);
  }
}
