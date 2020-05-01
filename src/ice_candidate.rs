use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SdpParseError {
  pub line: String,
  pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RTCIceComponent {
  Rtp,
  Rtcp,
}

impl From<u32> for RTCIceComponent {
  fn from(value: u32) -> Self {
    match value {
      0 => Self::Rtp,
      1 => Self::Rtcp,
      _ => panic!("invalid value for RTCIceComponent"),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RTCIceProtocol {
  Udp,
  Tcp,
}

impl From<webrtc_sdp::attribute_type::SdpAttributeCandidateTransport> for RTCIceProtocol {
  fn from(value: webrtc_sdp::attribute_type::SdpAttributeCandidateTransport) -> Self {
    match value {
      webrtc_sdp::attribute_type::SdpAttributeCandidateTransport::Udp => Self::Udp,
      webrtc_sdp::attribute_type::SdpAttributeCandidateTransport::Tcp => Self::Tcp,
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RTCIceTcpCandidateType {
  Active,
  Passive,
  So,
}

impl From<webrtc_sdp::attribute_type::SdpAttributeCandidateTcpType> for RTCIceTcpCandidateType {
  fn from(value: webrtc_sdp::attribute_type::SdpAttributeCandidateTcpType) -> Self {
    match value {
      webrtc_sdp::attribute_type::SdpAttributeCandidateTcpType::Active => Self::Active,
      webrtc_sdp::attribute_type::SdpAttributeCandidateTcpType::Passive => Self::Passive,
      webrtc_sdp::attribute_type::SdpAttributeCandidateTcpType::Simultaneous => Self::So,
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RTCIceCandidateType{
  Host,
  Srflx,
  Prflx,
  Relay,
}

impl From<webrtc_sdp::attribute_type::SdpAttributeCandidateType> for RTCIceCandidateType {
  fn from(value: webrtc_sdp::attribute_type::SdpAttributeCandidateType) -> Self {
    match value {
      webrtc_sdp::attribute_type::SdpAttributeCandidateType::Host => Self::Host,
      webrtc_sdp::attribute_type::SdpAttributeCandidateType::Srflx => Self::Srflx,
      webrtc_sdp::attribute_type::SdpAttributeCandidateType::Prflx => Self::Prflx,
      webrtc_sdp::attribute_type::SdpAttributeCandidateType::Relay => Self::Relay,
    }
  }
}

pub trait IceCandidateCommon {
  fn candidate(&self) -> String;
  fn sdp_mid(&self) -> String;
  fn sdp_mline_index(&self) -> u16;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RTCIceCandidateInit {
  pub candidate: String,
  pub sdp_mid: Option<String>,
  pub sdp_mline_index: Option<u16>,
}

impl IceCandidateCommon for RTCIceCandidateInit {
  fn candidate(&self) -> String {
    self.candidate.clone()
  }

  fn sdp_mid(&self) -> String {
    self.sdp_mid.clone().unwrap_or_default()
  }

  fn sdp_mline_index(&self) -> u16 {
    self.sdp_mline_index.unwrap_or_default()
  }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RTCIceCandidate {
  pub candidate: String,
  pub sdp_mid: Option<String>,
  pub sdp_mline_index: Option<u16>,

  #[serde(skip)]
  pub component: Option<RTCIceComponent>,

  #[serde(skip)]
  pub foundation: Option<String>,

  #[serde(skip)]
  pub address: Option<String>,

  #[serde(skip)]
  pub port: Option<u16>,

  #[serde(skip)]
  pub priority: Option<u32>,

  #[serde(skip)]
  pub protocol: Option<RTCIceProtocol>,

  #[serde(skip)]
  pub related_address: Option<String>,

  #[serde(skip)]
  pub related_port: Option<u16>,
  
  #[serde(skip)]
  pub tcp_type: Option<RTCIceTcpCandidateType>,

  #[serde(skip)]
  pub r#type: Option<RTCIceCandidateType>,
}

impl RTCIceCandidate {
  pub fn new(init: RTCIceCandidateInit) -> Result<Self, String> {
    match webrtc_sdp::attribute_type::parse_attribute(&init.candidate) {
      Ok(sdp) => {
        if let webrtc_sdp::SdpType::Attribute(webrtc_sdp::attribute_type::SdpAttribute::Candidate(candidate)) = sdp {
          Ok(Self {
            candidate: init.candidate,
            sdp_mid: init.sdp_mid,
            sdp_mline_index: init.sdp_mline_index,
            component: Some(RTCIceComponent::from(candidate.component)),
            foundation: Some(candidate.foundation),
            address: Some(candidate.address.to_string()),
            port: Some(candidate.port as u16),
            priority: Some(candidate.priority as u32),
            protocol: Some(RTCIceProtocol::from(candidate.transport)),
            related_address: candidate.raddr.map_or(None, |x| Some(x.to_string())),
            related_port: candidate.rport.map_or(None, |x| Some(x as u16)),
            tcp_type: candidate.tcp_type.map_or(None, |x| Some(RTCIceTcpCandidateType::from(x))),
            r#type: Some(RTCIceCandidateType::from(candidate.c_type)),
          })
        } else {
          Err("did not find candidate-attribute".to_string())
        }
      }
      Err(err) => Err(format!("{:?}", err)),
    }
  }
}

impl IceCandidateCommon for RTCIceCandidate {
  fn candidate(&self) -> String {
    self.candidate.clone()
  }

  fn sdp_mid(&self) -> String {
    self.sdp_mid.clone().unwrap_or_default()
  }

  fn sdp_mline_index(&self) -> u16 {
    self.sdp_mline_index.unwrap_or_default()
  }
}
