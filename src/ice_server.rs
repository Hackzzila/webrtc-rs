#[derive(Debug, Clone)]
pub struct RTCIceServer<'a> {
  pub urls: Vec<&'a str>,
  pub username: Option<&'a str>,
  pub credential: Option<&'a str>,
}