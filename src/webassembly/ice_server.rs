use wasm_bindgen::JsValue;
use web_sys;
use js_sys::*;

use crate::*;

impl From<RTCIceServer<'_>> for web_sys::RtcIceServer {
  fn from(value: RTCIceServer) -> Self {
    let mut server = Self::new();

    let urls = Array::new();
    for url in value.urls {
      urls.push(&JsValue::from(url));
    }

    server.urls(&urls);

    match value.username {
      Some(x) => {
        server.username(x);
      },
      None => (),
    };

    match value.credential {
      Some(x) => {
        server.credential(x);
      },
      None => (),
    };

    server
  }
}
