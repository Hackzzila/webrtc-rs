use crate::*;

#[derive(Debug, Clone)]
#[repr(C)]
pub enum RTCIceTransportPolicy {
  None,
  Relay,
  NoHost,
  All,
}

#[derive(Debug, Clone)]
#[repr(C)]
pub enum RTCBundlePolicy {
  Balanced,
  MaxBundle,
  MaxCompat,
}

#[derive(Debug, Clone)]
#[repr(C)]
pub enum RTCRtcpMuxPolicy {
  Negotiate,
  Require,
}

#[derive(Clone, Debug, Default)]
pub struct RTCConfiguration<'a> {
  pub ice_servers: Option<Vec<RTCIceServer<'a>>>,
  pub ice_transport_policy: Option<RTCIceTransportPolicy>,
  pub bundle_policy: Option<RTCBundlePolicy>,
  pub rtcp_mux_policy: Option<RTCRtcpMuxPolicy>,
  pub ice_candidate_pool_size: Option<u16>,
}
