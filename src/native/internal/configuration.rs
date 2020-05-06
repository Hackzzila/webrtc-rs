use libc::size_t;

use crate::*;
use crate::internal;

#[repr(C)]
pub struct RTCConfiguration {
  pub ice_servers: *const internal::RTCIceServer,
  pub ice_servers_len: size_t,
  pub ice_transport_policy: *const RTCIceTransportPolicy,
  pub bundle_policy: *const RTCBundlePolicy,
  pub rtcp_mux_policy: *const RTCRtcpMuxPolicy,
  pub ice_candidate_pool_size: *const u16,
}
