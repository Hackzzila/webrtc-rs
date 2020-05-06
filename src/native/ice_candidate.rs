use libc::{c_char, c_void};
use std::ffi::{CStr, CString};

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
    let new_candidate = unsafe {
      RTCIceCandidate::new(RTCIceCandidateInit {
        candidate: CStr::from_ptr(candidate.candidate).to_str().unwrap().to_string(),
        sdp_mid: Some(CStr::from_ptr(candidate.sdp_mid).to_str().unwrap().to_string()),
        sdp_mline_index: Some(candidate.sdp_mline_index),
      }).unwrap()
    };

    unsafe {
      internal::free(candidate.candidate as *mut c_void);
      internal::free(candidate.sdp_mid as *mut c_void);
    }

    new_candidate
  }
}

