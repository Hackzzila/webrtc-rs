use std::ffi::CString;

use crate::*;
use crate::internal::FromWithCleanup;

unsafe extern fn peer_connection_observer_on_signaling_state_change<'a>(ob: *mut Box<&'a dyn RTCPeerConnectionObserver>, state: RTCSignalingState) {
  let observer = Box::from_raw(ob);
  observer.on_signaling_state_change(state);
  Box::into_raw(observer);
}

unsafe extern fn peer_connection_observer_on_data_channel<'a>(ob: *mut Box<&'a dyn RTCPeerConnectionObserver>, ptr: *mut RTCDataChannelInterfaceC) {
  let observer = Box::from_raw(ob);
  observer.on_data_channel(RTCDataChannel { ptr, observer_ptr: None, c_observer_ptr: std::ptr::null_mut() });
  Box::into_raw(observer);
}

unsafe extern fn peer_connection_observer_on_ice_candidate<'a>(ob: *mut Box<&'a dyn RTCPeerConnectionObserver>, candidate: internal::RTCIceCandidateInit) {
  let observer = Box::from_raw(ob);
  observer.on_ice_candidate(RTCIceCandidate::from(candidate));
  Box::into_raw(observer);
}

#[repr(C)] pub(crate) struct RTCPeerConnectionFactoryInterfaceC { _private: [u8; 0] }

#[link(name = "webrtc-rs")]
extern {
  fn webrtc_rs_create_peer_connection_factory() -> *mut RTCPeerConnectionFactoryInterfaceC;
  fn webrtc_rs_release_peer_connection_factory(factory: *mut RTCPeerConnectionFactoryInterfaceC);

  fn webrtc_rs_create_peer_connection<'a>(
    factory: *mut RTCPeerConnectionFactoryInterfaceC,
    config: *const internal::RTCConfiguration,
    observer: *mut Box<&'a dyn RTCPeerConnectionObserver>,
    on_signaling_change: unsafe extern fn(*mut Box<&'a dyn RTCPeerConnectionObserver>, RTCSignalingState),
    on_data_channel: unsafe extern fn(*mut Box<&'a dyn RTCPeerConnectionObserver>, *mut RTCDataChannelInterfaceC),
    on_ice_candidate: unsafe extern fn(*mut Box<&'a dyn RTCPeerConnectionObserver>, internal::RTCIceCandidateInit),
  ) -> *mut RTCPeerConnectionInterfaceC;
}

pub struct RTCPeerConnectionFactory {
  ptr: *mut RTCPeerConnectionFactoryInterfaceC,
}

impl RTCPeerConnectionFactory {
  pub fn new() -> Self {
    Self { ptr: unsafe { webrtc_rs_create_peer_connection_factory() } }
  }

  pub fn create_peer_connection<'a>(&self, config: RTCConfiguration, ob: &'a dyn RTCPeerConnectionObserver) -> RTCPeerConnection<'a> {
    let mut c_strings = Vec::new();

    let internal_config = Box::new(internal::RTCConfiguration::from_with_cleanup(config, &mut c_strings));
    let internal_config_ptr = Box::into_raw(internal_config);

    let observer = Box::new(Box::new(ob));
    let observer_ptr = Box::into_raw(observer);

    let peer = unsafe {
      RTCPeerConnection {
        ptr: webrtc_rs_create_peer_connection(
          self.ptr,
          internal_config_ptr,
          observer_ptr,
          peer_connection_observer_on_signaling_state_change,
          peer_connection_observer_on_data_channel,
          peer_connection_observer_on_ice_candidate,
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
