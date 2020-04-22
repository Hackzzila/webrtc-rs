use libc::{c_char, c_void};
use std::ffi::{CStr, CString};

type CreateSessionDescriptionObserverSender = *mut tokio::sync::oneshot::Sender<Result<(String, String), String>>;
type SetSessionDescriptionObserverSender = *mut tokio::sync::oneshot::Sender<Result<(), String>>;


unsafe extern fn create_session_description_observer_success(sender: CreateSessionDescriptionObserverSender, type_str: *const c_char, sdp: *mut u8) {
  let len = libc::strlen(sdp as *mut i8);
  let slice = std::slice::from_raw_parts_mut(sdp, len);

  let string = std::str::from_utf8_mut(slice).unwrap();

  println!("{:?}", string);

  let boxed = Box::from_raw(sender);
  boxed.send(Ok((CStr::from_ptr(type_str).to_str().unwrap().to_string(), string.to_string().clone())));

  webrtc_rs_free(sdp as *mut c_void);
}

unsafe extern fn create_session_description_observer_failure(sender: CreateSessionDescriptionObserverSender, err: *const c_char) {
  let boxed = Box::from_raw(sender);
  boxed.send(Err(CStr::from_ptr(err).to_str().unwrap().to_string()));
}

unsafe extern fn set_session_description_observer_success(sender: SetSessionDescriptionObserverSender) {
  let boxed = Box::from_raw(sender);
  boxed.send(Ok(()));
}

unsafe extern fn set_session_description_observer_failure(sender: SetSessionDescriptionObserverSender, err: *const c_char) {
  let boxed = Box::from_raw(sender);
  boxed.send(Err(CStr::from_ptr(err).to_str().unwrap().to_string()));
}

#[link(name = "webrtc-rs")]
extern {
  fn webrtc_rs_free(ptr: *mut c_void);

  fn webrtc_rs_create_peer_connection_factory() -> *mut c_void;
  fn webrtc_rs_release_peer_connection_factory(factory: *mut c_void);

  fn webrtc_rs_create_rtc_configuration() -> *mut c_void;
  fn webrtc_rs_delete_rtc_configuration(config: *mut c_void);

  fn webrtc_rs_create_peer_connection(factory: *mut c_void, config: *mut c_void) -> *mut c_void;
  fn webrtc_rs_release_peer_connection(peer: *mut c_void);

  fn webrtc_rs_peer_connection_create_offer(
    peer: *mut c_void,
    sender: CreateSessionDescriptionObserverSender,
    success: unsafe extern fn(CreateSessionDescriptionObserverSender, *const c_char, *mut u8),
    error: unsafe extern fn(CreateSessionDescriptionObserverSender, *const c_char)
  );

  fn webrtc_rs_peer_connection_set_local_description(
    peer: *mut c_void,
    type_str: *mut c_char,
    sdp: *mut c_char,
    sender: SetSessionDescriptionObserverSender,
    success: unsafe extern fn(SetSessionDescriptionObserverSender),
    error: unsafe extern fn(SetSessionDescriptionObserverSender, *const c_char)
  );
}

pub struct PeerConnection {
  ptr: *mut c_void,
}

impl PeerConnection {
  pub fn from(ptr: *mut c_void) -> Self {
    Self { ptr }
  }

  pub async fn create_offer(&self) -> Result<(String, String), String> {
    let (tx, rx) = tokio::sync::oneshot::channel();
    let boxed = Box::new(tx);

    unsafe {
      webrtc_rs_peer_connection_create_offer(self.ptr, Box::into_raw(boxed), create_session_description_observer_success, create_session_description_observer_failure);
    }

    match rx.await {
      Ok(result) => return result,
      Err(err) => Err(err.to_string()),
    }
  }

  pub async fn set_local_description(&self, type_str: String, sdp: String) -> Result<(), String> {
    let (tx, rx) = tokio::sync::oneshot::channel();
    let boxed = Box::new(tx);

    unsafe {
      let type_cstr = CString::new(type_str).unwrap();
      let sdp_cstr = CString::new(sdp).unwrap();

      webrtc_rs_peer_connection_set_local_description(self.ptr, type_cstr.into_raw(), sdp_cstr.into_raw(), Box::into_raw(boxed), set_session_description_observer_success, set_session_description_observer_failure);
    }

    match rx.await {
      Ok(result) => return result,
      Err(err) => Err(err.to_string()),
    }
  }
}

impl Drop for PeerConnection {
  fn drop(&mut self) {
    unsafe {
      webrtc_rs_release_peer_connection(self.ptr);
    }
  }
}

pub struct PeerConnectionFactory {
  ptr: *mut c_void,
}

impl PeerConnectionFactory {
  pub fn new() -> Self {
    Self { ptr: unsafe { webrtc_rs_create_peer_connection_factory() } }
  }

  pub fn create_peer_connection(&self, config: *mut c_void) -> PeerConnection {
    PeerConnection::from(unsafe { webrtc_rs_create_peer_connection(self.ptr, config) })
  }
}

impl Drop for PeerConnectionFactory {
  fn drop(&mut self) {
    unsafe {
      webrtc_rs_release_peer_connection_factory(self.ptr);
    }
  }
}

#[tokio::main]
async fn main() {
  let factory = PeerConnectionFactory::new();
  let peer = factory.create_peer_connection(unsafe { webrtc_rs_create_rtc_configuration() });

  let res = peer.create_offer().await;
  println!("{:?}", res);

  let tuple = res.unwrap();

  let res2 = peer.set_local_description(tuple.0, tuple.1).await;
  println!("{:?}", res2);

  // unsafe {
  //   // let factory = ;
  //   // release_peer_connection_factory(create_peer_connection_factory());

  //   // let factory = create_peer_connection_factory();

  //   // let config = create_rtc_configuration();

  //   // let peer = create_peer(factory, config);

  //   // println!("{:?}", create_offer(peer).await);

  //   // // loop {}

  //   // release_peer(peer);
  //   // delete_rtc_configuration(config);
  //   // release_peer_connection_factory(factory);
  // }

  println!(":):");
}
