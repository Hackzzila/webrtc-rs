use crate::*;

pub trait Observer {
  fn on_signaling_state_change(&self, state: RTCSignalingState) {}
}
