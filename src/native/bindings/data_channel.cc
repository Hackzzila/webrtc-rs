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

class RTCDataChannelObserver : public webrtc::DataChannelObserver {
 public:
  RTCDataChannelObserver(
    void *rust_observer,
    webrtc::DataChannelInterface *data_channel,
    std::function<void(void *, int)> on_state_change,
    std::function<void(void *, internal::DataBuffer)> on_message
  ) : rust_observer_(rust_observer),
      data_channel_(data_channel),
      on_state_change_(on_state_change),
      on_message_(on_message) { }

  void OnStateChange() {
    if (on_state_change_) {
      on_state_change_(rust_observer_, static_cast<int>(data_channel_->state()));
    }
  }

  void OnMessage(const webrtc::DataBuffer& buffer) {
    if (on_message_) {
      on_message_(rust_observer_, internal::DataBuffer::From(buffer));
    }
  };

  void OnBufferedAmountChange(uint64_t sent_data_size) {}

 private:
  void *rust_observer_;
  webrtc::DataChannelInterface *data_channel_;
  std::function<void(void *, int)> on_state_change_;
  std::function<void(void *, internal::DataBuffer)> on_message_;
};

WEBRTC_RS_EXPORT void webrtc_rs_release_data_channel(void *dc_ptr) {
  reinterpret_cast<webrtc::DataChannelInterface *>(dc_ptr)->Release();
}

WEBRTC_RS_EXPORT void *webrtc_rs_data_channel_register_observer(void *dc_ptr, void *rust_observer, void(*on_state_change)(void *, int), void(*on_message)(void *, internal::DataBuffer)) {
  auto dc = reinterpret_cast<webrtc::DataChannelInterface *>(dc_ptr);
  auto observer = new RTCDataChannelObserver(rust_observer, dc, on_state_change, on_message);
  dc->RegisterObserver(observer);
  return observer;
}

WEBRTC_RS_EXPORT void webrtc_rs_data_channel_unregister_observer(void *dc_ptr, void *ob_ptr) {
  auto dc = reinterpret_cast<webrtc::DataChannelInterface *>(dc_ptr);
  dc->UnregisterObserver();
  delete ob_ptr;
}

WEBRTC_RS_EXPORT void webrtc_rs_data_channel_send(void *dc_ptr, internal::DataBuffer buf) {
  auto dc = reinterpret_cast<webrtc::DataChannelInterface *>(dc_ptr);
  dc->Send(buf);
}

WEBRTC_RS_EXPORT void webrtc_rs_data_channel_close(void *dc_ptr) {
  auto dc = reinterpret_cast<webrtc::DataChannelInterface *>(dc_ptr);
  dc->Close();
}

WEBRTC_RS_EXPORT int webrtc_rs_data_channel_get_ready_state(void *dc_ptr) {
  auto dc = reinterpret_cast<webrtc::DataChannelInterface *>(dc_ptr);
  return static_cast<int>(dc->state());
}

};  // namespace webrtc_rs

}
