use libc::{c_char, c_void, size_t};
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

  fn webrtc_rs_create_peer_connection(factory: *mut c_void, config: *const InternalRTCConfiguration) -> *mut c_void;
  fn webrtc_rs_release_peer_connection(peer: *mut c_void);

  fn webrtc_rs_peer_connection_create_offer(
    peer: *mut c_void,
    sender: CreateSessionDescriptionObserverSender,
    success: unsafe extern fn(CreateSessionDescriptionObserverSender, *const c_char, *mut u8),
    error: unsafe extern fn(CreateSessionDescriptionObserverSender, *const c_char)
  );

  fn webrtc_rs_peer_connection_create_answer(
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

  fn webrtc_rs_peer_connection_set_remote_description(
    peer: *mut c_void,
    type_str: *mut c_char,
    sdp: *mut c_char,
    sender: SetSessionDescriptionObserverSender,
    success: unsafe extern fn(SetSessionDescriptionObserverSender),
    error: unsafe extern fn(SetSessionDescriptionObserverSender, *const c_char)
  );
}

#[repr(C)]
struct InternalRTCIceServer {
  urls: *const *mut c_char,
  urls_len: size_t,
  username: *mut c_char,
  credential: *mut c_char,
}

#[derive(Debug, Clone)]
pub struct RTCIceServer<'a> {
  urls: Vec<&'a str>,
  username: Option<&'a str>,
  credential: Option<&'a str>,
}

impl<'a> RTCIceServer<'a> {
  fn into_internal(self, c_strings: &mut Vec<*mut c_char>) -> Result<InternalRTCIceServer, std::ffi::NulError> {
    let mut urls = Vec::new();
    for url in self.urls {
      let ptr = CString::new(url)?.into_raw();
      c_strings.push(ptr);
      urls.push(ptr);
    }

    Ok(InternalRTCIceServer {
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

#[derive(Debug, Clone)]
#[repr(i32)]
pub enum RTCIceTransportPolicy {
  None,
  Relay,
  NoHost,
  All,
}

#[derive(Debug, Clone)]
#[repr(i32)]
pub enum RTCBundlePolicy {
  Balanced,
  MaxBundle,
  MaxCompat,
}
#[derive(Debug, Clone)]
#[repr(i32)]
pub enum RTCRtcpMuxPolicy {
  Negotiate,
  Require,
}

#[repr(C)]
struct InternalRTCConfiguration {
  ice_servers: *const InternalRTCIceServer,
  ice_servers_len: size_t,
  ice_transport_policy: *const i32,
  bundle_policy: *const i32,
  rtcp_mux_policy: *const i32,
  ice_canidate_pool_size: *const u16,
}

#[derive(Clone, Debug, Default)]
pub struct RTCConfiguration<'a> {
  ice_servers: Option<Vec<RTCIceServer<'a>>>,
  ice_transport_policy: Option<RTCIceTransportPolicy>,
  bundle_policy: Option<RTCBundlePolicy>,
  rtcp_mux_policy: Option<RTCRtcpMuxPolicy>,
  ice_canidate_pool_size: Option<u16>,
}

impl<'a> RTCConfiguration<'a> {
  fn into_internal(self, c_strings: &mut Vec<*mut c_char>) -> Result<InternalRTCConfiguration, std::ffi::NulError> {
    let mut internal_ice_servers = Vec::new();

    if self.ice_servers.is_some() {
      for ice_server in self.ice_servers.expect("rust is broken") {
        internal_ice_servers.push(ice_server.into_internal(c_strings)?);
      }
    }

    Ok(InternalRTCConfiguration {
      ice_servers: internal_ice_servers.as_ptr(),
      ice_servers_len: internal_ice_servers.len(),
      ice_transport_policy: match self.ice_transport_policy {
        Some(x) => &(x as i32) as *const i32,
        None => std::ptr::null(),
      },
      bundle_policy: match self.bundle_policy {
        Some(x) => &(x as i32) as *const i32,
        None => std::ptr::null(),
      },
      rtcp_mux_policy: match self.rtcp_mux_policy {
        Some(x) => &(x as i32) as *const i32,
        None => std::ptr::null(),
      },
      ice_canidate_pool_size: match self.ice_canidate_pool_size {
        Some(x) => &x as *const u16,
        None => std::ptr::null(),
      },
    })
  }
}

pub struct RTCPeerConnection {
  ptr: *mut c_void,
}

impl RTCPeerConnection {
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

  pub async fn create_answer(&self) -> Result<(String, String), String> {
    let (tx, rx) = tokio::sync::oneshot::channel();
    let boxed = Box::new(tx);

    unsafe {
      webrtc_rs_peer_connection_create_answer(self.ptr, Box::into_raw(boxed), create_session_description_observer_success, create_session_description_observer_failure);
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

      webrtc_rs_peer_connection_set_local_description(
        self.ptr,
        type_cstr.into_raw(),
        sdp_cstr.into_raw(),
        Box::into_raw(boxed),
        set_session_description_observer_success,
        set_session_description_observer_failure,
      );
    }

    match rx.await {
      Ok(result) => return result,
      Err(err) => Err(err.to_string()),
    }
  }

  pub async fn set_remote_description(&self, type_str: String, sdp: String) -> Result<(), String> {
    let (tx, rx) = tokio::sync::oneshot::channel();
    let boxed = Box::new(tx);

    unsafe {
      let type_cstr = CString::new(type_str).unwrap();
      let sdp_cstr = CString::new(sdp).unwrap();

      webrtc_rs_peer_connection_set_remote_description(
        self.ptr,
        type_cstr.into_raw(),
        sdp_cstr.into_raw(),
        Box::into_raw(boxed),
        set_session_description_observer_success,
        set_session_description_observer_failure,
      );
    }

    match rx.await {
      Ok(result) => return result,
      Err(err) => Err(err.to_string()),
    }
  }
}

impl Drop for RTCPeerConnection {
  fn drop(&mut self) {
    unsafe {
      webrtc_rs_release_peer_connection(self.ptr);
    }
  }
}

pub struct RTCPeerConnectionFactory {
  ptr: *mut c_void,
}

impl RTCPeerConnectionFactory {
  pub fn new() -> Self {
    Self { ptr: unsafe { webrtc_rs_create_peer_connection_factory() } }
  }

  pub fn create_peer_connection(&self, config: RTCConfiguration) -> RTCPeerConnection {
    let mut c_strings = Vec::new();

    let internal_config = Box::new(config.into_internal(&mut c_strings).unwrap());

    let peer = RTCPeerConnection::from(unsafe { webrtc_rs_create_peer_connection(self.ptr, & *internal_config) });

    for c_string in c_strings {
      unsafe {
        let _ = CString::from_raw(c_string);
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

macro_rules! ice_servers {
  ( $( $x:expr ),* ) => {
    {
      let mut temp_ice_servers = Vec::new();

      $(
        temp_ice_servers.push(RTCIceServer {
          urls: vec![$x],
          username: None,
          credential: None,
        });
      )*

      RTCConfiguration {
        ice_servers: Some(temp_ice_servers),
        ice_transport_policy: None,
        ice_canidate_pool_size: None,
        bundle_policy: None,
        rtcp_mux_policy: None,
      }
    }
  };
}

#[tokio::main]
async fn main() {
  // let config = RTCConfiguration {
  //   ice_servers: None, /* Some(vec![
  //     RTCIceServer {
  //       urls: vec!["stun:stun.l.google.com:19302"],
  //       username: None,
  //       credential: None,
  //     }
  //   ]),*/
  //   ice_transport_policy: None,
  //   ice_canidate_pool_size: None,
  //   bundle_policy: None,
  //   rtcp_mux_policy: None,
  // };

    let cfg: RTCConfiguration = Default::default();
    println!("{:?}", cfg);

    // println!("{:?}", ice_servers!["stun:stun.l.google.com:19302"]);

  let factory = RTCPeerConnectionFactory::new();
  let peer1 = factory.create_peer_connection(ice_servers!["stun:stun.l.google.com:19302"]);
  let peer2 = factory.create_peer_connection(ice_servers!["stun:stun.l.google.com:19302"]);

  let res1 = peer1.create_offer().await;
  println!("{:?}", res1);

  let offer = res1.unwrap();

  println!("{:?}", peer1.set_local_description(offer.0.clone(), offer.1.clone()).await);
  println!("{:?}", peer2.set_remote_description(offer.0, offer.1).await);

  let res3 = peer2.create_answer().await;
  println!("{:?}", res3);
  let answer = res3.unwrap();

  println!("{:?}", peer2.set_local_description(answer.0.clone(), answer.1.clone()).await);
  println!("{:?}", peer1.set_remote_description(answer.0, answer.1).await);

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

  // while(true) { std::thread::sleep_ms(1000); }

  println!(":):");
}
