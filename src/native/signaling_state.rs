use crate::*;

impl From<i32> for RTCSignalingState {
  fn from(value: i32) -> Self {
    match value {
      0 => Self::Stable,
      1 => Self::HaveLocalOffer,
      2 => Self::HaveLocalPrAnswer,
      3 => Self::HaveRemoteOffer,
      4 => Self::HaveRemotePrAnswer,
      5 => Self::Closed,
      _ => panic!("invalid value for RTCSignalingState"),
    }
  }
}
