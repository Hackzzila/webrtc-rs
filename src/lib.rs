#![allow(improper_ctypes)]

#[cfg_attr(not(target_arch = "wasm32"), path = "native/mod.rs")]
#[cfg_attr(target_arch = "wasm32", path = "webassembly/mod.rs")]
mod backend;

mod configuration;
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

pub use ice_server::RTCIceServer;

pub use observer::Observer;

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
