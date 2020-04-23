use libc::c_void;
use std::ffi::CString;

use crate::*;

unsafe extern fn observer_on_signaling_state_change(ob: *mut Box<dyn Observer>, state_num: i32) {
  let observer = Box::from_raw(ob);
  let state = RTCSignalingState::from(state_num);
  observer.on_signaling_state_change(state);
  Box::into_raw(observer);
}

#[link(name = "webrtc-rs")]
extern {
  fn webrtc_rs_create_peer_connection_factory() -> *mut c_void;
  fn webrtc_rs_release_peer_connection_factory(factory: *mut c_void);

  fn webrtc_rs_create_peer_connection(
    factory: *mut c_void,
    config: *const internal::RTCConfiguration,
    observer: *mut Box<dyn Observer>,
    on_signaling_change: unsafe extern fn(*mut Box<dyn Observer>, i32)
  ) -> *mut c_void;
}

pub struct RTCPeerConnectionFactory {
  ptr: *mut c_void,
}

impl RTCPeerConnectionFactory {
  pub fn new() -> Self {
    Self { ptr: unsafe { webrtc_rs_create_peer_connection_factory() } }
  }

  pub fn create_peer_connection(&self, config: RTCConfiguration, ob: Box<dyn Observer>) -> RTCPeerConnection {
    let mut c_strings = Vec::new();

    let internal_config = Box::new(config.into_internal(&mut c_strings).unwrap());
    let internal_config_ptr = Box::into_raw(internal_config);

    let observer = Box::new(ob);
    let observer_ptr = Box::into_raw(observer);

    let peer = unsafe {
      RTCPeerConnection {
        ptr: webrtc_rs_create_peer_connection(
          self.ptr,
          internal_config_ptr,
          observer_ptr,
          observer_on_signaling_state_change
        ),
        observer_ptr,
      }
    };

    unsafe { Box::from_raw(internal_config_ptr); }

    for c_string in c_strings {
      unsafe {
        CString::from_raw(c_string);
      }
    }

    peer
  }
}

impl Drop for RTCPeerConnectionFactory {
  fn drop(&mut self) {
    unsafe {
      webrtc_rs_release_peer_connection_factory(self.ptr);
    }
  }
}
