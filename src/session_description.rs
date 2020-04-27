use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[repr(i32)]
pub enum RTCSdpType {
  Offer,
  PrAnswer,
  Answer,
  Rollback,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RTCSessionDescription {
  pub r#type: RTCSdpType,
  pub sdp: String,
}
