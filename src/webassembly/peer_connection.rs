use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::*;
use web_sys;
use js_sys::*;

use crate::*;

pub struct RTCPeerConnection {
  pub(crate) internal: Rc<web_sys::RtcPeerConnection>,
}

impl RTCPeerConnection {
  pub async fn create_offer(&self) -> Result<RTCSessionDescription, String> {
    match JsFuture::from(self.internal.create_offer()).await {
      Ok(obj) => Ok(RTCSessionDescription::from(web_sys::RtcSessionDescription::from(obj))),
      Err(err) => Err(Object::from(err).to_string().as_string().expect("toString() did not return a string"))
    }
  }

  pub async fn create_answer(&self) -> Result<RTCSessionDescription, String> {
    match JsFuture::from(self.internal.create_answer()).await {
      Ok(obj) => Ok(RTCSessionDescription::from(web_sys::RtcSessionDescription::from(obj))),
      Err(err) => Err(Object::from(err).to_string().as_string().expect("toString() did not return a string"))
    }
  }

  pub async fn set_local_description(&self, desc: RTCSessionDescription) -> Result<(), String> {
    match JsFuture::from(self.internal.set_local_description(&web_sys::RtcSessionDescriptionInit::from(desc))).await {
      Ok(_) => Ok(()),
      Err(err) => Err(Object::from(err).to_string().as_string().expect("toString() did not return a string"))
    }
  }

  pub async fn set_remote_description(&self, desc: RTCSessionDescription) -> Result<(), String> {
    match JsFuture::from(self.internal.set_remote_description(&web_sys::RtcSessionDescriptionInit::from(desc))).await {
      Ok(_) => Ok(()),
      Err(err) => Err(Object::from(err).to_string().as_string().expect("toString() did not return a string"))
    }
  }
}
