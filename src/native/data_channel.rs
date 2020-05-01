use libc::{c_void};

use crate::*;
use crate::internal::FromWithCleanupVec;

unsafe extern fn data_channel_observer_on_state_change(ob: *mut Box<dyn RTCDataChannelObserver>, state_num: i32) {
  let observer = Box::from_raw(ob);
  match RTCDataChannelState::from(state_num) {
    RTCDataChannelState::Open => observer.on_open(),
    RTCDataChannelState::Closing => observer.on_closing(),
    RTCDataChannelState::Closed => observer.on_close(),
    RTCDataChannelState::Connecting => (),
  }
  Box::into_raw(observer);
}

unsafe extern fn data_channel_observer_on_message(ob: *mut Box<dyn RTCDataChannelObserver>, data: internal::DataBuffer) {
  let observer = Box::from_raw(ob);
  observer.on_message(Message::from(data));
  Box::into_raw(observer);
}

#[link(name = "webrtc-rs")]
extern {
  fn webrtc_rs_release_data_channel(dc: *mut c_void);

  fn webrtc_rs_data_channel_register_observer(
    dc: *mut c_void,
    ob: *mut Box<dyn RTCDataChannelObserver>,
    on_state_change: unsafe extern fn(*mut Box<dyn RTCDataChannelObserver>, i32),
    on_message: unsafe extern fn(*mut Box<dyn RTCDataChannelObserver>, internal::DataBuffer),
  ) -> *mut c_void;
  fn webrtc_rs_data_channel_unregister_observer(dc: *mut c_void, cob: *mut c_void);

  fn webrtc_rs_data_channel_send(dc: *mut c_void, msg: internal::DataBuffer);
  fn webrtc_rs_data_channel_close(dc: *mut c_void);

  fn webrtc_rs_data_channel_get_ready_state(dc: *mut c_void) -> i32;
}

pub struct RTCDataChannel {
  pub(crate) ptr: *mut c_void,
  pub(crate) observer_ptr: Option<*mut Box<dyn RTCDataChannelObserver>>,
  pub(crate) c_observer_ptr: *mut c_void,
}

impl RTCDataChannel {
  pub fn register_observer(&mut self, ob: Box<dyn RTCDataChannelObserver>) {
    self.unregister_observer();

    let observer = Box::new(ob);
    let observer_ptr = Box::into_raw(observer);

    self.observer_ptr = Some(observer_ptr);

    unsafe {
      self.c_observer_ptr = webrtc_rs_data_channel_register_observer(self.ptr, observer_ptr, data_channel_observer_on_state_change, data_channel_observer_on_message);
    }
  }

  pub fn unregister_observer(&mut self) {
    if let Some(observer_ptr) = self.observer_ptr {
      unsafe {
        webrtc_rs_data_channel_unregister_observer(self.ptr, self.c_observer_ptr);
        Box::from_raw(observer_ptr);
        self.c_observer_ptr = std::ptr::null_mut();
      }
    }

    self.observer_ptr = None;
  }

  pub fn send(&self, message: Message) {
    let (buffer, vec) = internal::DataBuffer::from_with_cleanup_vec(message);

    unsafe {
      webrtc_rs_data_channel_send(self.ptr, buffer);
    }
  }

  pub fn close(&self) {
    unsafe { webrtc_rs_data_channel_close(self.ptr); }
  }

  pub fn get_ready_state(&self) -> RTCDataChannelState {
    RTCDataChannelState::from(unsafe { webrtc_rs_data_channel_get_ready_state(self.ptr) })
  }
}

impl Drop for RTCDataChannel {
  fn drop(&mut self) {
    self.unregister_observer();
    unsafe {
      webrtc_rs_release_data_channel(self.ptr);
    }
  }
}
