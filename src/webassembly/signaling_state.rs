use web_sys;

use crate::*;

impl From<web_sys::RtcSignalingState> for RTCSignalingState {
  fn from(value: web_sys::RtcSignalingState) -> Self {
    match value {
      web_sys::RtcSignalingState::Stable => Self::Stable,
      web_sys::RtcSignalingState::HaveLocalOffer => Self::HaveLocalOffer,
      web_sys::RtcSignalingState::HaveRemoteOffer => Self::HaveRemoteOffer,
      web_sys::RtcSignalingState::HaveLocalPranswer => Self::HaveLocalPrAnswer,
      web_sys::RtcSignalingState::HaveRemotePranswer => Self::HaveRemotePrAnswer,
      web_sys::RtcSignalingState::Stable => Self::Closed,
      _ => panic!("invalid value for RTCSignalingState")
    }
  }
}
