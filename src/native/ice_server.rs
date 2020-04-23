use libc::c_char;
use std::ffi::CString;

use crate::*;

impl internal::FromWithCleanup<RTCIceServer<'_>> for internal::RTCIceServer {
  fn from_with_cleanup(server: RTCIceServer, c_strings: &mut Vec<*mut c_char>) -> Self {
    let mut urls = Vec::new();
    for url in server.urls {
      let ptr = CString::new(url).expect("error making CString").into_raw();
      c_strings.push(ptr);
      urls.push(ptr);
    }

    Self {
      urls: urls.as_ptr(),
      urls_len: urls.len(),
      username: match server.username {
        Some(x) => {
          let ptr = CString::new(x).expect("error making CString").into_raw();
          c_strings.push(ptr);
          ptr
        },
        None => std::ptr::null_mut(),
      },
      credential: match server.credential {
        Some(x) => {
          let ptr = CString::new(x).expect("error making CString").into_raw();
          c_strings.push(ptr);
          ptr
        }
        None => std::ptr::null_mut(),
      },
    }
  }
}