#ifndef WEBRTC_RS_INTERNAL_RTC_CONFIGURATION_H_
#define WEBRTC_RS_INTERNAL_RTC_CONFIGURATION_H_

#include "api/peer_connection_interface.h"

#include "internal_rtc_ice_server.h"

namespace webrtc_rs {

namespace internal {

struct RTCConfiguration {
  RTCIceServer *ice_servers;
  size_t ice_servers_len;
  webrtc::PeerConnectionInterface::IceTransportsType *ice_transport_policy;
  webrtc::PeerConnectionInterface::BundlePolicy *bundle_policy;
  webrtc::PeerConnectionInterface::RtcpMuxPolicy *rtcp_mux_policy;
  uint16_t *ice_candidate_pool_size;

  operator webrtc::PeerConnectionInterface::RTCConfiguration() const {
    auto config = webrtc::PeerConnectionInterface::RTCConfiguration(webrtc::PeerConnectionInterface::RTCConfigurationType::kSafe);
    config.sdp_semantics = webrtc::SdpSemantics::kUnifiedPlan;

    for (int i = 0; i < this->ice_servers_len; i++) {
      config.servers.push_back(this->ice_servers[i]);
    }

    if (this->ice_transport_policy) {
      config.type = *this->ice_transport_policy;
    }

    if (this->bundle_policy) {
      config.bundle_policy = *this->bundle_policy;
    }

    if (this->rtcp_mux_policy) {
      config.rtcp_mux_policy = *this->rtcp_mux_policy;
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
