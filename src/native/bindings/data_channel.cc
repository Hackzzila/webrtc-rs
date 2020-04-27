#include <iostream>

#include "api/peer_connection_interface.h"

#include "common.h"
#include "internal_rtc_configuration.h"
#include "internal_session_description.h"
#include "create_session_description_observer.h"
#include "set_session_description_observer.h"

extern "C" {

class RTCDataChannelObserver : public webrtc::DataChannelObserver {
 public:
  RTCDataChannelObserver(void *rust_observer, webrtc::DataChannelInterface *data_channel, std::function<void(void *, int)> on_state_change)
    : rust_observer_(rust_observer), data_channel_(data_channel), on_state_change_(on_state_change) { }

  void OnStateChange() {
    if (on_state_change_) {
      on_state_change_(rust_observer_, static_cast<int>(data_channel_->state()));
    }
  }

  void OnMessage(const webrtc::DataBuffer& buffer) {};

  void OnBufferedAmountChange(uint64_t sent_data_size) {}

 private:
  void *rust_observer_;
  webrtc::DataChannelInterface *data_channel_;
  std::function<void(void *, int)> on_state_change_;
};

namespace webrtc_rs {

WEBRTC_RS_EXPORT void webrtc_rs_release_data_channel(void *dc_ptr) {
  reinterpret_cast<webrtc::DataChannelInterface *>(dc_ptr)->Release();
}

WEBRTC_RS_EXPORT void *webrtc_rs_data_channel_register_observer(void *dc_ptr, void *rust_observer, void(*on_state_change)(void *, int)) {
  auto dc = reinterpret_cast<webrtc::DataChannelInterface *>(dc_ptr);
  auto observer = new RTCDataChannelObserver(rust_observer, dc, on_state_change);
  dc->RegisterObserver(observer);
  return observer;
}

WEBRTC_RS_EXPORT void webrtc_rs_data_channel_unregister_observer(void *dc_ptr, void *ob_ptr) {
  auto dc = reinterpret_cast<webrtc::DataChannelInterface *>(dc_ptr);
  dc->UnregisterObserver();
  delete ob_ptr;
}

};  // namespace webrtc_rs

}
