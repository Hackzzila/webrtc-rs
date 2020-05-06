#include <iostream>

#include "api/peer_connection_interface.h"

#include "common.h"
#include "internal_data_buffer.h"
#include "internal_rtc_configuration.h"
#include "internal_session_description.h"
#include "create_session_description_observer.h"
#include "set_session_description_observer.h"

extern "C" {

namespace webrtc_rs {

struct RustRTCDataChannelObserver;

class RTCDataChannelObserver : public webrtc::DataChannelObserver {
 public:
  RTCDataChannelObserver(
    RustRTCDataChannelObserver *rust_observer,
    webrtc::DataChannelInterface *data_channel,
    std::function<void(RustRTCDataChannelObserver *, webrtc::DataChannelInterface::DataState)> on_state_change,
    std::function<void(RustRTCDataChannelObserver *, internal::DataBuffer)> on_message
  ) : rust_observer_(rust_observer),
      data_channel_(data_channel),
      on_state_change_(on_state_change),
      on_message_(on_message) { }

  void OnStateChange() {
    if (on_state_change_) {
      on_state_change_(rust_observer_, data_channel_->state());
    }
  }

  void OnMessage(const webrtc::DataBuffer& buffer) {
    if (on_message_) {
      on_message_(rust_observer_, internal::DataBuffer::From(buffer));
    }
  };

  void OnBufferedAmountChange(uint64_t sent_data_size) {}

 private:
  RustRTCDataChannelObserver *rust_observer_;
  webrtc::DataChannelInterface *data_channel_;
  std::function<void(RustRTCDataChannelObserver *, webrtc::DataChannelInterface::DataState)> on_state_change_;
  std::function<void(RustRTCDataChannelObserver *, internal::DataBuffer)> on_message_;
};

WEBRTC_RS_EXPORT void webrtc_rs_release_data_channel(webrtc::DataChannelInterface *dc) {
  dc->Release();
}

WEBRTC_RS_EXPORT RTCDataChannelObserver *webrtc_rs_data_channel_register_observer(
    webrtc::DataChannelInterface *dc,
    RustRTCDataChannelObserver *rust_observer,
    void(*on_state_change)(RustRTCDataChannelObserver *, webrtc::DataChannelInterface::DataState),
    void(*on_message)(RustRTCDataChannelObserver *, internal::DataBuffer)) {
  auto observer = new RTCDataChannelObserver(rust_observer, dc, on_state_change, on_message);
  dc->RegisterObserver(observer);
  return observer;
}

WEBRTC_RS_EXPORT void webrtc_rs_data_channel_unregister_observer(webrtc::DataChannelInterface *dc, RTCDataChannelObserver *ob_ptr) {
  dc->UnregisterObserver();
  delete ob_ptr;
}

WEBRTC_RS_EXPORT void webrtc_rs_data_channel_send(webrtc::DataChannelInterface *dc, internal::DataBuffer buf) {
  dc->Send(buf);
}

WEBRTC_RS_EXPORT void webrtc_rs_data_channel_close(webrtc::DataChannelInterface *dc) {
  dc->Close();
}

WEBRTC_RS_EXPORT int webrtc_rs_data_channel_get_ready_state(webrtc::DataChannelInterface *dc) {
  return static_cast<int>(dc->state());
}

};  // namespace webrtc_rs

}
