#[derive(Debug, Clone)]
#[repr(C)]
pub enum RTCSignalingState {
  Stable,
  HaveLocalOffer,
  HaveLocalPrAnswer,
  HaveRemoteOffer,
  HaveRemotePrAnswer,
  Closed,
}
