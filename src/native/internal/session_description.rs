use libc::c_char;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct RTCSessionDescription {
  pub r#type: i32,
  pub sdp: *mut c_char,
}
