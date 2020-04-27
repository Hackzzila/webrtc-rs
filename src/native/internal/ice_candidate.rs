use libc::c_char;

#[derive(Clone, Debug)]
#[repr(C)]
pub struct SdpParseError {
  pub line: *mut c_char,
  pub description: *mut c_char,
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct RTCIceCandidateInit {
  pub candidate: *mut c_char,
  pub sdp_mid: *mut c_char,
  pub sdp_mline_index: u16,
}