#[derive(Debug, Clone)]
#[repr(i32)]
pub enum RTCSignalingState {
  Stable = 0,
  HaveLocalOffer = 1,
  HaveLocalPrAnswer = 2,
  HaveRemoteOffer = 3,
  HaveRemotePrAnswer = 4,
  Closed = 5,
}

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