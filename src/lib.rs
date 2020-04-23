pub(crate) mod internal;
mod configuration;
mod ice_server;
mod observer;
mod peer_connection;
mod peer_connection_factory;
mod signaling_state;

pub use configuration::{
  RTCIceTransportPolicy,
  RTCBundlePolicy,
  RTCRtcpMuxPolicy,
  RTCConfiguration,
};

pub use ice_server::RTCIceServer;

pub use observer::Observer;

pub use peer_connection_factory::RTCPeerConnectionFactory;

pub use peer_connection::RTCPeerConnection;

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
        ice_canidate_pool_size: None,
        bundle_policy: None,
        rtcp_mux_policy: None,
      }
    }
  };
}
