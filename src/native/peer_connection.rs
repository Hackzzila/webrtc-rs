use libc::{c_char, c_void};
use std::ffi::{CStr, CString};

use crate::*;
use crate::internal::FromWithCleanup;

type CreateSessionDescriptionObserverSender = *mut tokio::sync::oneshot::Sender<Result<RTCSessionDescription, String>>;
type SetSessionDescriptionObserverSender = *mut tokio::sync::oneshot::Sender<Result<(), String>>;

unsafe extern fn create_session_description_observer_success(sender: CreateSessionDescriptionObserverSender, desc_internal: internal::RTCSessionDescription) {
  let desc = RTCSessionDescription::from(desc_internal);
  let boxed = Box::from_raw(sender);

  boxed.send(Ok(desc)).expect("oneshot receiver dropped");
}

unsafe extern fn create_session_description_observer_failure(sender: CreateSessionDescriptionObserverSender, err: *const c_char) {
  let boxed = Box::from_raw(sender);
  boxed.send(Err(CStr::from_ptr(err).to_str().unwrap().to_string())).expect("oneshot receiver dropped");
}

unsafe extern fn set_session_description_observer_success(sender: SetSessionDescriptionObserverSender) {
  let boxed = Box::from_raw(sender);
  boxed.send(Ok(())).expect("oneshot receiver dropped");
}

unsafe extern fn set_session_description_observer_failure(sender: SetSessionDescriptionObserverSender, err: *const c_char) {
  let boxed = Box::from_raw(sender);
  boxed.send(Err(CStr::from_ptr(err).to_str().unwrap().to_string())).expect("oneshot receiver dropped");
}

#[link(name = "webrtc-rs")]
extern {
  fn webrtc_rs_release_peer_connection(peer: *mut c_void);

  fn webrtc_rs_peer_connection_create_offer(
    peer: *mut c_void,
    sender: CreateSessionDescriptionObserverSender,
    success: unsafe extern fn(CreateSessionDescriptionObserverSender, internal::RTCSessionDescription),
    error: unsafe extern fn(CreateSessionDescriptionObserverSender, *const c_char)
  );

  fn webrtc_rs_peer_connection_create_answer(
    peer: *mut c_void,
    sender: CreateSessionDescriptionObserverSender,
    success: unsafe extern fn(CreateSessionDescriptionObserverSender, internal::RTCSessionDescription),
    error: unsafe extern fn(CreateSessionDescriptionObserverSender, *const c_char)
  );

  fn webrtc_rs_peer_connection_set_local_description(
    peer: *mut c_void,
    desc: *mut internal::RTCSessionDescription,
    sender: SetSessionDescriptionObserverSender,
    success: unsafe extern fn(SetSessionDescriptionObserverSender),
    error: unsafe extern fn(SetSessionDescriptionObserverSender, *const c_char)
  );

  fn webrtc_rs_peer_connection_set_remote_description(
    peer: *mut c_void,
    desc: *mut internal::RTCSessionDescription,
    sender: SetSessionDescriptionObserverSender,
    success: unsafe extern fn(SetSessionDescriptionObserverSender),
    error: unsafe extern fn(SetSessionDescriptionObserverSender, *const c_char)
  );

  fn webrtc_rs_peer_connection_create_data_channel(
    peer: *mut c_void,
    label: *const c_char,
  ) -> *mut c_void;

  fn webrtc_rs_peer_connection_add_ice_candidate(
    peer: *mut c_void,
    candidate: *mut internal::RTCIceCandidateInit,
  ) -> *mut internal::SdpParseError;
}

pub struct RTCPeerConnection<'a> {
  pub(crate) ptr: *mut c_void,
  pub(crate) observer_ptr: *mut Box<&'a dyn RTCPeerConnectionObserver>,
}

impl<'a> RTCPeerConnection<'a> {
  pub async fn create_offer(&self) -> Result<RTCSessionDescription, String> {
    let (tx, rx) = tokio::sync::oneshot::channel();
    let boxed = Box::new(tx);

    unsafe {
      webrtc_rs_peer_connection_create_offer(self.ptr, Box::into_raw(boxed), create_session_description_observer_success, create_session_description_observer_failure);
    }

    rx.await.map_err(|x| x.to_string())?
  }

  pub async fn create_answer(&self) -> Result<RTCSessionDescription, String> {
    let (tx, rx) = tokio::sync::oneshot::channel();
    let boxed = Box::new(tx);

    unsafe {
      webrtc_rs_peer_connection_create_answer(self.ptr, Box::into_raw(boxed), create_session_description_observer_success, create_session_description_observer_failure);
    }

    rx.await.map_err(|x| x.to_string())?
  }

  pub async fn set_local_description(&self, desc: RTCSessionDescription) -> Result<(), String> {
    let (tx, rx) = tokio::sync::oneshot::channel();
    let boxed = Box::new(tx);

    let mut c_strings = Vec::new();

    let internal_desc = Box::new(internal::RTCSessionDescription::from_with_cleanup(desc, &mut c_strings));
    let internal_desc_ptr = Box::into_raw(internal_desc);

    unsafe {
      webrtc_rs_peer_connection_set_local_description(
        self.ptr,
        internal_desc_ptr,
        Box::into_raw(boxed),
        set_session_description_observer_success,
        set_session_description_observer_failure,
      );
    }

    unsafe { Box::from_raw(internal_desc_ptr ); }

    for c_string in c_strings {
      unsafe {
        CString::from_raw(c_string);
      }
    }

    rx.await.map_err(|x| x.to_string())?
  }

  pub async fn set_remote_description(&self, desc: RTCSessionDescription) -> Result<(), String> {
    let (tx, rx) = tokio::sync::oneshot::channel();
    let boxed = Box::new(tx);

    let mut c_strings = Vec::new();

    let internal_desc = Box::new(internal::RTCSessionDescription::from_with_cleanup(desc, &mut c_strings));
    let internal_desc_ptr = Box::into_raw(internal_desc);

    unsafe {
      webrtc_rs_peer_connection_set_remote_description(
        self.ptr,
        internal_desc_ptr,
        Box::into_raw(boxed),
        set_session_description_observer_success,
        set_session_description_observer_failure,
      );
    }

    unsafe { Box::from_raw(internal_desc_ptr ); }

    for c_string in c_strings {
      unsafe {
        CString::from_raw(c_string);
      }
    }

    rx.await.map_err(|x| x.to_string())?
  }

  pub fn create_data_channel(&self, label: &str) -> RTCDataChannel {
    let c_string = CString::new(label).unwrap();
    let c_ptr = c_string.as_ptr();

    RTCDataChannel {
      ptr: unsafe {
        webrtc_rs_peer_connection_create_data_channel(self.ptr, c_ptr)
      },
      observer_ptr: None,
      c_observer_ptr: std::ptr::null_mut(),
    }
  }

  pub fn add_ice_candidate<'b>(&self, candidate: &'b dyn IceCandidateCommon) -> Result<(), SdpParseError> {
    let mut c_strings = Vec::new();

    let internal_candidate = Box::new(internal::RTCIceCandidateInit::from_with_cleanup(candidate, &mut c_strings));
    let internal_candidate_ptr = Box::into_raw(internal_candidate);

    let err = unsafe {
      webrtc_rs_peer_connection_add_ice_candidate(
        self.ptr,
        internal_candidate_ptr,
      )
    };

    unsafe { Box::from_raw(internal_candidate_ptr ); }

    for c_string in c_strings {
      unsafe {
        CString::from_raw(c_string);
      }
    }

    if err != std::ptr::null_mut() {
      let line = unsafe {
        let len = libc::strlen((*err).line);
        std::slice::from_raw_parts_mut((*err).line as *mut u8, len)
      };
      let line = std::str::from_utf8_mut(line).unwrap();

      let description = unsafe {
        let len = libc::strlen((*err).description);
        std::slice::from_raw_parts_mut((*err).description as *mut u8, len)
      };
      let description = std::str::from_utf8_mut(description).unwrap();

      unsafe {
        internal::delete((*err).line as *mut c_void);
        internal::delete((*err).description as *mut c_void);
      }

      Err(SdpParseError { line: line.to_string(), description: description.to_string() })
    } else {
      Ok(())
    }
  }
}

impl<'a> Drop for RTCPeerConnection<'a> {
  fn drop(&mut self) {
    unsafe {
      webrtc_rs_release_peer_connection(self.ptr);
      Box::from_raw(self.observer_ptr);
    }
  }
}
