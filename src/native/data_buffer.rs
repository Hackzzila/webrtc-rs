use std::iter::FromIterator;

use crate::*;

impl From<internal::DataBuffer> for Message {
  fn from(value: internal::DataBuffer) -> Self {
    let mut vec = Vec::with_capacity(value.len);
    vec.extend_from_slice(unsafe { std::slice::from_raw_parts(value.data, value.len) });

    if value.binary {
      Self::Binary(vec)
    } else {
      Self::String(unsafe { String::from_utf8_unchecked(vec) })
    }
  }
}

impl internal::FromWithCleanupVec<Message> for internal::DataBuffer {
  fn from_with_cleanup_vec(value: Message) -> (Self, Vec<u8>) {
    match value {
      Message::String(string) => {
        let mut vec = Vec::with_capacity(string.len());
        for c in string.as_str().chars() {
          vec.push(c as u8);
        }

        (Self {
          data: vec.as_ptr(),
          len: vec.len(),
          binary: false,
        }, vec)
      },

      Message::Binary(vec) => {
        (Self {
          data: vec.as_ptr(),
          len: vec.len(),
          binary: true,
        }, vec)
      },
    }
  }
}
