use libc::{c_char, size_t};

#[repr(C)]
pub struct RTCIceServer {
  pub urls: *const *mut c_char,
  pub urls_len: size_t,
  pub username: *mut c_char,
  pub credential: *mut c_char,
}
