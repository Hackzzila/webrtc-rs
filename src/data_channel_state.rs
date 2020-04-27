#[derive(Debug, Clone)]
#[repr(i32)]
pub enum RTCDataChannelState {
  Connecting,
  Open,
  Closing,
  Closed
}
