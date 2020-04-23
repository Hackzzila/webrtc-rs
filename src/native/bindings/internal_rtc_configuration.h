#ifndef WEBRTC_RS_INTERNAL_RTC_CONFIGURATION_H_
#define WEBRTC_RS_INTERNAL_RTC_CONFIGURATION_H_

#include "api/peer_connection_interface.h"

#include "internal_rtc_ice_server.h"

namespace webrtc_rs {

namespace internal {

struct RTCConfiguration {
  RTCIceServer *ice_servers;
  size_t ice_servers_len;
  int *ice_transport_policy;
  int *bundle_policy;
  int *rtcp_mux_policy;
  uint16_t *ice_candidate_pool_size;

  operator webrtc::PeerConnectionInterface::RTCConfiguration() const {
    webrtc::PeerConnectionInterface::RTCConfiguration config;
    config.sdp_semantics = webrtc::SdpSemantics::kUnifiedPlan;

    for (int i = 0; i < this->ice_servers_len; i++) {
      config.servers.push_back(this->ice_servers[i]);
    }

    if (this->ice_transport_policy) {
      config.type = static_cast<webrtc::PeerConnectionInterface::IceTransportsType>(*this->ice_transport_policy);
    }

    if (this->bundle_policy) {
      config.bundle_policy = static_cast<webrtc::PeerConnectionInterface::BundlePolicy>(*this->bundle_policy);
    }

    if (this->rtcp_mux_policy) {
      config.rtcp_mux_policy = static_cast<webrtc::PeerConnectionInterface::RtcpMuxPolicy>(*this->rtcp_mux_policy);
    }

    if (this->ice_candidate_pool_size) {
      config.ice_candidate_pool_size = *this->ice_candidate_pool_size;
    }

    return config;
  }
};

};  // namespace internal

}; // namespace webrtc_rs

#endif  // WEBRTC_RS_INTERNAL_RTC_CONFIGURATION_H_
