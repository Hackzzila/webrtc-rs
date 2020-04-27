use libc::{c_char, c_void};
use std::ffi::CString;

use crate::*;

impl<'a> internal::FromWithCleanup<&'a dyn IceCandidateCommon> for internal::RTCIceCandidateInit {
  fn from_with_cleanup<'b>(common: &'b dyn IceCandidateCommon, c_strings: &mut Vec<*mut c_char>) -> Self {
    Self {
      sdp_mline_index: common.sdp_mline_index(),
      candidate: {
        let ptr = CString::new(common.candidate()).expect("error making CString").into_raw();
        c_strings.push(ptr);
        ptr
      },
      sdp_mid: {
        let ptr = CString::new(common.sdp_mid()).expect("error making CString").into_raw();
        c_strings.push(ptr);
        ptr
      },
    }
  }
}

impl From<internal::RTCIceCandidateInit> for RTCIceCandidate {
  fn from(candidate: internal::RTCIceCandidateInit) -> Self {
    let sdp = unsafe {
      let len = libc::strlen(candidate.candidate);
      std::slice::from_raw_parts_mut(candidate.candidate as *mut u8, len)
    };
    let sdp = std::str::from_utf8_mut(sdp).unwrap();

    let sdp_mid = unsafe {
      let len = libc::strlen(candidate.sdp_mid);
      std::slice::from_raw_parts_mut(candidate.sdp_mid as *mut u8, len)
    };
    let sdp_mid = std::str::from_utf8_mut(sdp_mid).unwrap();

    let new_candidate = RTCIceCandidate::new(RTCIceCandidateInit {
      candidate: sdp.to_string(),
      sdp_mid: Some(sdp_mid.to_string()),
      sdp_mline_index: Some(candidate.sdp_mline_index),
    }).unwrap();

    unsafe {
      internal::delete(candidate.candidate as *mut c_void);
      internal::delete(candidate.sdp_mid as *mut c_void);
    }

    new_candidate
  }
}

