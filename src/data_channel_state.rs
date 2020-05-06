#[derive(Debug, Clone)]
#[repr(C)]
pub enum RTCDataChannelState {
  Connecting,
  Open,
  Closing,
  Closed
}
