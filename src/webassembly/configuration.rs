use wasm_bindgen::JsValue;
use web_sys;
use js_sys::*;

use crate::*;

impl From<RTCIceTransportPolicy> for web_sys::RtcIceTransportPolicy {
  fn from(policy: RTCIceTransportPolicy) -> Self {
    match policy {
      RTCIceTransportPolicy::Relay => Self::Relay,
      RTCIceTransportPolicy::All => Self::All,
      _ => panic!("invalid RTCIceTransportPolicy for WebAssembly")
    }
  }
}

impl From<RTCBundlePolicy> for web_sys::RtcBundlePolicy {
  fn from(policy: RTCBundlePolicy) -> Self {
    match policy {
      RTCBundlePolicy::Balanced => Self::Balanced,
      RTCBundlePolicy::MaxBundle => Self::MaxBundle,
      RTCBundlePolicy::MaxCompat => Self::MaxCompat,
    }
  }
}

impl From<RTCConfiguration<'_>> for web_sys::RtcConfiguration {
  fn from(value: RTCConfiguration) -> Self {
    let mut config = Self::new();

    let internal_ice_servers = Array::new();
    if let Some(ice_servers) = value.ice_servers {
      for ice_server in ice_servers {
        internal_ice_servers.push(&web_sys::RtcIceServer::from(ice_server));
      }
    }

    config.ice_servers(&internal_ice_servers);

    match value.bundle_policy {
      Some(x) => { config.bundle_policy(web_sys::RtcBundlePolicy::from(x)); },
      None => (),
    };

    match value.ice_transport_policy {
      Some(x) => { config.ice_transport_policy(web_sys::RtcIceTransportPolicy::from(x)); },
      None => (),
    };

    config
  }
}
