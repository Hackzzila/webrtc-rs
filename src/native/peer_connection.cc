#include <iostream>

#include "api/peer_connection_interface.h"

#include "common.h"
#include "internal_rtc_configuration.h"
#include "create_session_description_observer.h"
#include "set_session_description_observer.h"

extern "C" {

class Observer : public webrtc::PeerConnectionObserver {
 public:
  Observer(void *rust_observer, std::function<void(void *, int)> on_signaling_change): rust_observer_(rust_observer), on_signaling_change_(on_signaling_change) { }

  void OnSignalingChange(webrtc::PeerConnectionInterface::SignalingState state) {
    std::cout << "OnSignalingChange" << std::endl;
    if (on_signaling_change_) {
      on_signaling_change_(rust_observer_, static_cast<int>(state));
    }
  }

  void OnDataChannel(rtc::scoped_refptr<webrtc::DataChannelInterface> data_channel) {
    std::cout << "OnDataChannel" << std::endl;
  }

  void OnRenegotiationNeeded() {
    std::cout << "OnRenegotiationNeeded" << std::endl;
  }

  void OnIceGatheringChange(webrtc::PeerConnectionInterface::IceGatheringState new_state) {
    std::cout << "OnIceGatheringChange" << std::endl;
  }

  void OnIceCandidate(const webrtc::IceCandidateInterface *canidate) {
    std::cout << "OnIceCanidate" << std::endl;
  }

 private:
  void *rust_observer_;
  std::function<void(void *, int)> on_signaling_change_;
};

namespace webrtc_rs {

WEBRTC_RS_EXPORT void *webrtc_rs_create_peer_connection(void *factory_ptr, void *config_ptr, void *rust_observer, void(*on_signaling_change)(void *, int)) {
  auto factory = reinterpret_cast<webrtc::PeerConnectionFactoryInterface *>(factory_ptr);
  auto config = reinterpret_cast<internal::RTCConfiguration *>(config_ptr);

  Observer *observer = new Observer(rust_observer, on_signaling_change);

  return factory->CreatePeerConnection(*config, webrtc::PeerConnectionDependencies(observer)).release();
}

WEBRTC_RS_EXPORT void webrtc_rs_release_peer_connection(void *peer_ptr) {
  reinterpret_cast<webrtc::PeerConnectionInterface *>(peer_ptr)->Release();
}

WEBRTC_RS_EXPORT void webrtc_rs_peer_connection_create_offer(void *peer_ptr, void *sender, void(*success)(void *, const char *, char *), void(*error)(void *, const char *)) {
  auto peer = reinterpret_cast<webrtc::PeerConnectionInterface *>(peer_ptr);

  webrtc::PeerConnectionInterface::RTCOfferAnswerOptions options;
  peer->CreateOffer(new CreateSessionDescriptionObserver(sender, success, error), options);
}

WEBRTC_RS_EXPORT void webrtc_rs_peer_connection_create_answer(void *peer_ptr, void *sender, void(*success)(void *, const char *, char *), void(*error)(void *, const char *)) {
  auto peer = reinterpret_cast<webrtc::PeerConnectionInterface *>(peer_ptr);

  webrtc::PeerConnectionInterface::RTCOfferAnswerOptions options;
  peer->CreateAnswer(new CreateSessionDescriptionObserver(sender, success, error), options);
}

WEBRTC_RS_EXPORT void webrtc_rs_peer_connection_set_local_description(void *peer_ptr, char *type_str, char *sdp_str, void *sender, void(*success)(void *), void(*error)(void *, const char *)) {
  auto peer = reinterpret_cast<webrtc::PeerConnectionInterface *>(peer_ptr);

  auto type = webrtc::SdpTypeFromString(std::string(type_str));
  if (!type) {
    error(sender, "invalid type_str passed into webrtc_rs_peer_connection_set_local_description");
    return;
  }

  webrtc::SdpParseError *err = nullptr;
  auto description = webrtc::CreateSessionDescription(type.value(), std::string(sdp_str), err);
  if (err) {
    std::string error_str;
    error_str += "Error parsing SDP at line " + err->line + ": " + err->description;

    char *error_message = reinterpret_cast<char *>(malloc(error_str.size() + 1));
    std::strcpy(error_message, error_str.c_str());

    error(sender, error_message);
    return;
  }

  peer->SetLocalDescription(new SetSessionDescriptionObserver(sender, success, error), description.release());
}

WEBRTC_RS_EXPORT void webrtc_rs_peer_connection_set_remote_description(void *peer_ptr, char *type_str, char *sdp_str, void *sender, void(*success)(void *), void(*error)(void *, const char *)) {
  auto peer = reinterpret_cast<webrtc::PeerConnectionInterface *>(peer_ptr);

  auto type = webrtc::SdpTypeFromString(std::string(type_str));
  if (!type) {
    error(sender, "invalid type_str passed into webrtc_rs_peer_connection_set_remote_description");
    return;
  }

  webrtc::SdpParseError *err = nullptr;
  auto description = webrtc::CreateSessionDescription(type.value(), std::string(sdp_str), err);
  if (err) {
    std::string error_str;
    error_str += "Error parsing SDP at line " + err->line + ": " + err->description;

    char *error_message = reinterpret_cast<char *>(malloc(error_str.size() + 1));
    std::strcpy(error_message, error_str.c_str());

    error(sender, error_message);
    return;
  }

  peer->SetRemoteDescription(new SetSessionDescriptionObserver(sender, success, error), description.release());
}

};  // namespace webrtc_rs

}
