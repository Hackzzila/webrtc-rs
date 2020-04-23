use libc::c_char;

use crate::*;

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

#[derive(Clone, Debug, Default)]
pub struct RTCConfiguration<'a> {
  pub ice_servers: Option<Vec<RTCIceServer<'a>>>,
  pub ice_transport_policy: Option<RTCIceTransportPolicy>,
  pub bundle_policy: Option<RTCBundlePolicy>,
  pub rtcp_mux_policy: Option<RTCRtcpMuxPolicy>,
  pub ice_canidate_pool_size: Option<u16>,
}

impl<'a> RTCConfiguration<'a> {
  pub(crate) fn into_internal(self, c_strings: &mut Vec<*mut c_char>) -> Result<internal::RTCConfiguration, std::ffi::NulError> {
    let mut internal_ice_servers = Vec::new();

    if let Some(ice_servers) = self.ice_servers {
      for ice_server in ice_servers {
        internal_ice_servers.push(ice_server.into_internal(c_strings)?);
      }
    }

    Ok(internal::RTCConfiguration {
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
