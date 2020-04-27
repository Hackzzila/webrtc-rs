use libc::c_char;

use crate::*;

impl internal::FromWithCleanup<RTCConfiguration<'_>> for internal::RTCConfiguration {
  fn from_with_cleanup(config: RTCConfiguration, c_strings: &mut Vec<*mut c_char>) -> Self {
    let mut internal_ice_servers = Vec::new();

    if let Some(ice_servers) = config.ice_servers {
      for ice_server in ice_servers {
        internal_ice_servers.push(internal::RTCIceServer::from_with_cleanup(ice_server, c_strings));
      }
    }

    Self {
      ice_servers: internal_ice_servers.as_ptr(),
      ice_servers_len: internal_ice_servers.len(),
      ice_transport_policy: match config.ice_transport_policy {
        Some(x) => &(x as i32) as *const i32,
        None => std::ptr::null(),
      },
      bundle_policy: match config.bundle_policy {
        Some(x) => &(x as i32) as *const i32,
        None => std::ptr::null(),
      },
      rtcp_mux_policy: match config.rtcp_mux_policy {
        Some(x) => &(x as i32) as *const i32,
        None => std::ptr::null(),
      },
      ice_candidate_pool_size: match config.ice_candidate_pool_size {
        Some(x) => &x as *const u16,
        None => std::ptr::null(),
      },
    }
  }
}
