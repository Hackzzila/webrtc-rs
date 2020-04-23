use libc::size_t;

use crate::internal;

#[repr(C)]
pub struct RTCConfiguration {
  pub ice_servers: *const internal::RTCIceServer,
  pub ice_servers_len: size_t,
  pub ice_transport_policy: *const i32,
  pub bundle_policy: *const i32,
  pub rtcp_mux_policy: *const i32,
  pub ice_candidate_pool_size: *const u16,
}
