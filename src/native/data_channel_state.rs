use crate::*;

impl From<i32> for RTCDataChannelState {
  fn from(value: i32) -> Self {
    match value {
      0 => Self::Connecting,
      1 => Self::Open,
      2 => Self::Closing,
      3 => Self::Closed,
      _ => panic!("invalid value for RTCDataChannelState"),
    }
  }
}
