use libc::{c_char, c_void};
use std::ffi::CString;

use crate::*;

impl From<i32> for RTCSdpType {
  fn from(value: i32) -> Self {
    match value {
      0 => Self::Offer,
      1 => Self::PrAnswer,
      2 => Self::Answer,
      3 => Self::Rollback,
      _ => panic!("invalid value for RTCSdpType"),
    }
  }
}

impl internal::FromWithCleanup<RTCSessionDescription> for internal::RTCSessionDescription {
  fn from_with_cleanup(desc: RTCSessionDescription, c_strings: &mut Vec<*mut c_char>) -> Self {
    let sdp = CString::new(desc.sdp).expect("error making CString").into_raw();
    c_strings.push(sdp);

    Self {
      r#type: (desc.r#type) as i32,
      sdp: sdp,
    }
  }
}

impl From<internal::RTCSessionDescription> for RTCSessionDescription {
  fn from(desc: internal::RTCSessionDescription) -> Self {
    let slice = unsafe {
      let len = libc::strlen(desc.sdp);
      std::slice::from_raw_parts_mut(desc.sdp as *mut u8, len)
    };

    let string = std::str::from_utf8_mut(slice).unwrap();

    let new_desc = Self {
      r#type: RTCSdpType::from(desc.r#type),
      sdp: string.to_string(),
    };

    unsafe { internal::delete(desc.sdp as *mut c_void); }

    new_desc
  }
}

