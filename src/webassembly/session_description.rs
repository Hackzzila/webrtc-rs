use web_sys;

use crate::*;

impl From<web_sys::RtcSdpType> for RTCSdpType {
  fn from(value: web_sys::RtcSdpType) -> Self {
    match value {
      web_sys::RtcSdpType::Offer => Self::Offer,
      web_sys::RtcSdpType::Pranswer => Self::PrAnswer,
      web_sys::RtcSdpType::Answer => Self::Answer,
      web_sys::RtcSdpType::Rollback => Self::Rollback,
      _ => panic!("invalid value for RTCSdpType"),
    }
  }
}

impl From<RTCSdpType> for web_sys::RtcSdpType {
  fn from(value: RTCSdpType) -> Self {
    match value {
      RTCSdpType::Offer => Self::Offer,
      RTCSdpType::PrAnswer => Self::Pranswer,
      RTCSdpType::Answer => Self::Answer,
      RTCSdpType::Rollback => Self::Rollback,
    }
  }
}

impl From<web_sys::RtcSessionDescription> for RTCSessionDescription {
  fn from(value: web_sys::RtcSessionDescription) -> Self {
    Self {
      r#type: RTCSdpType::from(value.type_()),
      sdp: value.sdp(),
    }
  }
}

impl From<RTCSessionDescription> for web_sys::RtcSessionDescription {
  fn from(value: RTCSessionDescription) -> Self {
    let desc = Self::new().expect("failed to create RtcSessionDescription");

    desc.set_type(web_sys::RtcSdpType::from(value.r#type));
    desc.set_sdp(&value.sdp);

    desc
  }
}

impl From<RTCSessionDescription> for web_sys::RtcSessionDescriptionInit {
  fn from(value: RTCSessionDescription) -> Self {
    let mut desc = Self::new(web_sys::RtcSdpType::from(value.r#type));
    desc.sdp(&value.sdp);

    desc
  }
}

