use std::fmt;
use serde::ser::{Serialize, Serializer};
use serde::de::{Visitor, Deserializer, Deserialize};

#[derive(Debug, Clone)]
#[repr(C)]
pub enum RTCSdpType {
  Offer,
  PrAnswer,
  Answer,
  Rollback,
}

impl From<RTCSdpType> for &str {
  fn from(value: RTCSdpType) -> Self {
    match value {
      RTCSdpType::Offer => "offer",
      RTCSdpType::PrAnswer => "pranswer",
      RTCSdpType::Answer => "answer",
      RTCSdpType::Rollback => "rollback",
    }
  }
}

impl From<&str> for RTCSdpType {
  fn from(value: &str) -> Self {
    match value {
      "offer" => Self::Offer,
      "pranswer" => Self::PrAnswer,
      "answer" => Self::Answer,
      "rollback" => Self::Rollback,
      _ => panic!("invalid value for RTCSdpType"),
    }
  }
}

impl Serialize for RTCSdpType {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where 
    S: Serializer,
  {
    serializer.serialize_str(self.clone().into())
  }
}

struct RTCSdpTypeVisitor;
impl<'de> Visitor<'de> for RTCSdpTypeVisitor {
  type Value = RTCSdpType;

  fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
      formatter.write_str("a enum value of \"offer\", \"pranswer\", \"answer\", or \"rollback\"")
  }

  fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
  where
    E: serde::de::Error,
  {
    Ok(RTCSdpType::from(value))
  }
}

impl<'de> Deserialize<'de> for RTCSdpType {
  fn deserialize<D>(deserializer: D) -> Result<RTCSdpType, D::Error>
  where
      D: Deserializer<'de>,
  {
    deserializer.deserialize_str(RTCSdpTypeVisitor)
  }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RTCSessionDescription {
  pub r#type: RTCSdpType,
  pub sdp: String,
}
