use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use js_sys::*;
use web_sys;

use crate::*;

pub struct RTCPeerConnectionFactory;

impl RTCPeerConnectionFactory {
  pub fn new() -> Self {
    Self { }
  }

  pub fn create_peer_connection(&self, config: RTCConfiguration, ob: Box<dyn Observer>) -> RTCPeerConnection {
    let peer = RTCPeerConnection {
      internal: Rc::new(web_sys::RtcPeerConnection::new_with_configuration(&web_sys::RtcConfiguration::from(config)).expect("failed to construct RTCPeerConnection")),
    };

    let ob_rc = Rc::new(ob);
    let ob_cloned = Rc::clone(&ob_rc);
    let internal_cloned = Rc::clone(&peer.internal);
    let cb = Closure::wrap(Box::new(move | value: Object | {
      ob_cloned.on_signaling_state_change(RTCSignalingState::from(internal_cloned.signaling_state()));
    }) as Box<FnMut(Object)>);

    peer.internal.set_onsignalingstatechange(Some(cb.as_ref().unchecked_ref()));
    cb.forget();

    peer
  }
}
