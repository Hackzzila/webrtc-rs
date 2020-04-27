use crate::*;

#[allow(unused_variables)]
#[allow(unused_mut)]
pub trait RTCPeerConnectionObserver {
  fn on_signaling_state_change(&self, state: RTCSignalingState) {}
  fn on_data_channel(&self, mut dc: RTCDataChannel) {}
  fn on_ice_candidate(&self, candidate: RTCIceCandidate) {}
}

#[allow(unused_variables)]
pub trait RTCDataChannelObserver {
  fn on_open(&self) {}
  fn on_close(&self) {}
  fn on_closing(&self) {}
  fn on_message(&self) {}
}
