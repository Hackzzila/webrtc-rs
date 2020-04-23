use libc::c_char;
use std::ffi::CString;

use crate::*;

#[derive(Debug, Clone)]
pub struct RTCIceServer<'a> {
  pub urls: Vec<&'a str>,
  pub username: Option<&'a str>,
  pub credential: Option<&'a str>,
}

impl<'a> RTCIceServer<'a> {
  pub(crate) fn into_internal(self, c_strings: &mut Vec<*mut c_char>) -> Result<internal::RTCIceServer, std::ffi::NulError> {
    let mut urls = Vec::new();
    for url in self.urls {
      let ptr = CString::new(url)?.into_raw();
      c_strings.push(ptr);
      urls.push(ptr);
    }

    Ok(internal::RTCIceServer {
      urls: urls.as_ptr(),
      urls_len: urls.len(),
      username: match self.username {
        Some(x) => {
          let ptr = CString::new(x)?.into_raw();
          c_strings.push(ptr);
          ptr
        },
        None => std::ptr::null_mut(),
      },
      credential: match self.credential {
        Some(x) => {
          let ptr = CString::new(x)?.into_raw();
          c_strings.push(ptr);
          ptr
        }
        None => std::ptr::null_mut(),
      },
    })
  }
}
