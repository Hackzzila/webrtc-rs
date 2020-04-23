#ifndef WEBRTC_RS_INTERNAL_RTC_ICE_SERVER_H_
#define WEBRTC_RS_INTERNAL_RTC_ICE_SERVER_H_

#include "api/peer_connection_interface.h"

namespace webrtc_rs {

namespace internal {

struct RTCIceServer {
  char **urls;
  size_t urls_len;
  char *username;
  char *credential;

  operator webrtc::PeerConnectionInterface::IceServer() const {
    webrtc::PeerConnectionInterface::IceServer ice_server;

    for (int n = 0; n < this->urls_len; n++) {
      ice_server.urls.push_back(std::string(this->urls[n]));
    }
    if (this->username) ice_server.username = std::string(this->username);
    if (this->credential) ice_server.password = std::string(this->credential);

    return ice_server;
  }
};

};  // namespace internal

};  // namespace webrtc_rs

#endif  // WEBRTC_RS_INTERNAL_RTC_ICE_SERVER_H_
