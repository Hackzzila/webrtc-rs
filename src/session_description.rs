#[derive(Debug, Clone)]
#[repr(i32)]
pub enum RTCSdpType {
  Offer,
  PrAnswer,
  Answer,
  Rollback,
}

#[derive(Debug, Clone)]
pub struct RTCSessionDescription {
  pub r#type: RTCSdpType,
  pub sdp: String,
}
