#include <iostream>

#include "api/peer_connection_interface.h"

#include "common.h"
#include "ice_candidate.h"
#include "internal_rtc_configuration.h"
#include "internal_session_description.h"
#include "create_session_description_observer.h"
#include "set_session_description_observer.h"

extern "C" {

namespace webrtc_rs {

class RTCPeerConnectionObserver : public webrtc::PeerConnectionObserver {
 public:
  RTCPeerConnectionObserver(
    void *rust_observer,
    std::function<void(void *, int)> on_signaling_change,
    std::function<void(void *, void *)> on_data_channel,
    std::function<void(void *, internal::RTCIceCandidateInit)> on_ice_candidate
  ) : rust_observer_(rust_observer),
      on_signaling_change_(on_signaling_change),
      on_data_channel_(on_data_channel),
      on_ice_candidate_(on_ice_candidate) { }

  void OnSignalingChange(webrtc::PeerConnectionInterface::SignalingState state) {
    std::cout << "OnSignalingChange" << std::endl;
    if (on_signaling_change_) {
      on_signaling_change_(rust_observer_, static_cast<int>(state));
    }
  }

  void OnDataChannel(rtc::scoped_refptr<webrtc::DataChannelInterface> data_channel) {
    std::cout << "OnDataChannel" << std::endl;
    if (on_data_channel_) {
      on_data_channel_(rust_observer_, data_channel.release());
    }
  }

  void OnRenegotiationNeeded() {
    std::cout << "OnRenegotiationNeeded" << std::endl;
  }

  void OnIceGatheringChange(webrtc::PeerConnectionInterface::IceGatheringState new_state) {
    std::cout << "OnIceGatheringChange " << new_state << std::endl;
  }

  void OnIceCandidate(const webrtc::IceCandidateInterface *candidate) {
    // std::string str;
    // candidate->ToString(&str);

    std::cout << "OnIceCanidate" << std::endl;
    // std::cout << candidate->sdp_mid() << std::endl;
    // std::cout << candidate->sdp_mline_index() << std::endl;
    // std::cout << str << std::endl;

    // exit(1);

    if (on_ice_candidate_) {
      on_ice_candidate_(rust_observer_, internal::RTCIceCandidateInit::From(candidate));
    }
  }

 private:
  void *rust_observer_;
  std::function<void(void *, int)> on_signaling_change_;
  std::function<void(void *, void *)> on_data_channel_;
  std::function<void(void *, internal::RTCIceCandidateInit)> on_ice_candidate_;
};

WEBRTC_RS_EXPORT void *webrtc_rs_create_peer_connection(void *factory_ptr, void *config_ptr, void *rust_observer, void(*on_signaling_change)(void *, int), void(*on_data_channel)(void *, void *), void(*on_ice_candidate)(void *, internal::RTCIceCandidateInit)) {
  auto factory = reinterpret_cast<webrtc::PeerConnectionFactoryInterface *>(factory_ptr);
  auto config = reinterpret_cast<internal::RTCConfiguration *>(config_ptr);
  auto observer = new RTCPeerConnectionObserver(rust_observer, on_signaling_change, on_data_channel, on_ice_candidate);

  return factory->CreatePeerConnection(*config, webrtc::PeerConnectionDependencies(observer)).release();
}

WEBRTC_RS_EXPORT void webrtc_rs_release_peer_connection(void *peer_ptr) {
  reinterpret_cast<webrtc::PeerConnectionInterface *>(peer_ptr)->Release();
}

WEBRTC_RS_EXPORT void webrtc_rs_peer_connection_create_offer(void *peer_ptr, void *sender, void(*success)(void *, internal::RTCSessionDescription), void(*error)(void *, const char *)) {
  auto peer = reinterpret_cast<webrtc::PeerConnectionInterface *>(peer_ptr);

  std::cout << "sender " << (intptr_t)sender << std::endl;

  webrtc::PeerConnectionInterface::RTCOfferAnswerOptions options;
  peer->CreateOffer(new CreateSessionDescriptionObserver(sender, success, error), options);
}

WEBRTC_RS_EXPORT void webrtc_rs_peer_connection_create_answer(void *peer_ptr, void *sender, void(*success)(void *, internal::RTCSessionDescription), void(*error)(void *, const char *)) {
  auto peer = reinterpret_cast<webrtc::PeerConnectionInterface *>(peer_ptr);

  webrtc::PeerConnectionInterface::RTCOfferAnswerOptions options;
  peer->CreateAnswer(new CreateSessionDescriptionObserver(sender, success, error), options);
}

WEBRTC_RS_EXPORT void webrtc_rs_peer_connection_set_local_description(void *peer_ptr, internal::RTCSessionDescription *desc, void *sender, void(*success)(void *), void(*error)(void *, const char *)) {
  auto peer = reinterpret_cast<webrtc::PeerConnectionInterface *>(peer_ptr);

  peer->SetLocalDescription(new SetSessionDescriptionObserver(sender, success, error), *desc);
}

WEBRTC_RS_EXPORT void webrtc_rs_peer_connection_set_remote_description(void *peer_ptr, internal::RTCSessionDescription *desc, void *sender, void(*success)(void *), void(*error)(void *, const char *)) {
  auto peer = reinterpret_cast<webrtc::PeerConnectionInterface *>(peer_ptr);

  peer->SetRemoteDescription(new SetSessionDescriptionObserver(sender, success, error), *desc);
}

WEBRTC_RS_EXPORT void *webrtc_rs_peer_connection_create_data_channel(void *peer_ptr, char *label) {
  auto peer = reinterpret_cast<webrtc::PeerConnectionInterface *>(peer_ptr);
  
  auto config = new webrtc::DataChannelInit;
  return peer->CreateDataChannel(std::string(label), config).release();
}

WEBRTC_RS_EXPORT internal::SdpParseError *webrtc_rs_peer_connection_add_ice_candidate(void *peer_ptr, internal::RTCIceCandidateInit *candidate) {
  auto peer = reinterpret_cast<webrtc::PeerConnectionInterface *>(peer_ptr);
  
  internal::SdpParseError *error = nullptr;
  auto rtc_candidate = candidate->To(error);

  if (error) {
    return error;
  }

  peer->AddIceCandidate(rtc_candidate);

  return nullptr;
}

};  // namespace webrtc_rs

}
