use libc::{c_char, c_void};
use std::ffi::{CStr, CString};

use crate::*;

impl internal::FromWithCleanup<RTCSessionDescription> for internal::RTCSessionDescription {
  fn from_with_cleanup(desc: RTCSessionDescription, c_strings: &mut Vec<*mut c_char>) -> Self {
    let sdp = CString::new(desc.sdp).expect("error making CString").into_raw();
    c_strings.push(sdp);

    Self {
      r#type: desc.r#type,
      sdp: sdp,
    }
  }
}

impl From<internal::RTCSessionDescription> for RTCSessionDescription {
  fn from(desc: internal::RTCSessionDescription) -> Self {
    let new_desc = unsafe {
      Self {
        r#type: desc.r#type,
        sdp: CStr::from_ptr(desc.sdp).to_str().unwrap().to_string(),
      }
    };

    unsafe { internal::free(desc.sdp as *mut c_void); }

    new_desc
  }
}

