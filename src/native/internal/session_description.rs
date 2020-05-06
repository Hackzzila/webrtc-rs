use libc::c_char;

use crate::*;

#[derive(Debug, Clone)]
#[repr(C)]
pub struct RTCSessionDescription {
  pub r#type: RTCSdpType,
  pub sdp: *mut c_char,
}
