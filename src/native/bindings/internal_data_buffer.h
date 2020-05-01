#ifndef WEBRTC_RS_INTERNAL_DATA_BUFFER_H_
#define WEBRTC_RS_INTERNAL_DATA_BUFFER_H_

#include "api/data_channel_interface.h"

#include "internal_rtc_ice_server.h"

namespace webrtc_rs {

namespace internal {

struct DataBuffer {
  const uint8_t *data;
  size_t len;
  bool binary;

  static DataBuffer From(const webrtc::DataBuffer &value) {
    DataBuffer buf;
    buf.data = value.data.data<uint8_t>();
    buf.len = value.size();
    buf.binary = value.binary;

    return buf;
  }

  operator webrtc::DataBuffer() const {
    return webrtc::DataBuffer(rtc::CopyOnWriteBuffer(this->data, this->len), this->binary);
  }
};

};  // namespace internal

}; // namespace webrtc_rs

#endif  // WEBRTC_RS_INTERNAL_DATA_BUFFER_H_
