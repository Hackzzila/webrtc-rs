#[derive(Debug, Clone)]
#[repr(i32)]
pub enum RTCSignalingState {
  Stable,
  HaveLocalOffer,
  HaveLocalPrAnswer,
  HaveRemoteOffer,
  HaveRemotePrAnswer,
  Closed,
}
