#![allow(improper_ctypes)]

#[cfg_attr(not(target_arch = "wasm32"), path = "native/mod.rs")]
#[cfg_attr(target_arch = "wasm32", path = "webassembly/mod.rs")]
mod backend;

mod configuration;
mod data_channel_state;
mod ice_candidate;
mod ice_server;
mod observer;
mod session_description;
mod signaling_state;

pub use backend::*;

pub use configuration::{
  RTCIceTransportPolicy,
  RTCBundlePolicy,
  RTCRtcpMuxPolicy,
  RTCConfiguration,
};

pub use data_channel_state::RTCDataChannelState;

pub use ice_candidate::{
  SdpParseError,
  IceCandidateCommon,
  RTCIceCandidateInit,
  RTCIceComponent,
  RTCIceProtocol,
  RTCIceTcpCandidateType,
  RTCIceCandidateType,
  RTCIceCandidate,
};

pub use ice_server::RTCIceServer;

pub use observer::{RTCPeerConnectionObserver, RTCDataChannelObserver};

pub use session_description::{RTCSdpType, RTCSessionDescription};

pub use signaling_state::RTCSignalingState;

#[macro_export]
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
        ice_candidate_pool_size: None,
        bundle_policy: None,
        rtcp_mux_policy: None,
      }
    }
  };
}

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::*;

#[cfg(target_arch = "wasm32")]
struct MyObserver;
#[cfg(target_arch = "wasm32")]
impl RTCPeerConnectionObserver for MyObserver {
  fn on_signaling_state_change(&self, state: RTCSignalingState) {
    console_log!("Signaling change :) {:?}", state);
  }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub async fn foo() -> JsValue {
  console_error_panic_hook::set_once();

  let factory = RTCPeerConnectionFactory::new();

  let ob1 = Box::new(MyObserver { });
  let ob2 = Box::new(MyObserver { });

  let factory = RTCPeerConnectionFactory::new();
  let peer1 = factory.create_peer_connection(ice_servers!["stun:stun.l.google.com:19302"], ob1);
  let peer2 = factory.create_peer_connection(ice_servers!["stun:stun.l.google.com:19302"], ob2);

  let res1 = peer1.create_offer().await;
  println!("{:?}", res1);

  let offer = res1.unwrap();

  println!("{:?}", peer1.set_local_description(offer.clone()).await);
  println!("{:?}", peer2.set_remote_description(offer).await);

  let res3 = peer2.create_answer().await;
  println!("{:?}", res3);
  let answer = res3.clone().unwrap();

  println!("{:?}", peer2.set_local_description(answer.clone()).await);
  println!("{:?}", peer1.set_remote_description(answer).await);

  // let js: &JsValue = &peer;

  // let res = peer.create_answer().await;
  JsValue::from(format!("{:?}", res3))

  // JsValue::from(future_to_promise(peer.create_offer()));

  // let config = RTCConfiguration {
  //     ice_servers: Some(vec![
  //       RTCIceServer {
  //         urls: vec!["stun:stun.l.google.com:19302"],
  //         username: Some("foo"),
  //         credential: Some("bar"),
  //       },
  //       RTCIceServer {
  //         urls: vec!["stun:stun.l.google.com:19302"],
  //         username: None,
  //         credential: None,
  //       }
  //     ]),
  //     ice_transport_policy: Some(RTCIceTransportPolicy::All),
  //     // ice_candidate_pool_size: None,
  //     bundle_policy: Some(RTCBundlePolicy::MaxBundle),
  //     // rtcp_mux_policy: None,
  //     ..Default::default()
  //   };

    // JsValue::from(peer.internal)

  // println!("{:?}",ice_servers!["foo"].into_internal());

  // JsValue::from(peer.internal)
  // JsValue::from(ice_servers!["foo"].into_internal())

  // JsValue::from("foo")
  // "bar".to_string()
  // JsValue::NULL
  // JsValue::from()
}
