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

struct RustRTCPeerConnectionObserver;

class RTCPeerConnectionObserver : public webrtc::PeerConnectionObserver {
 public:
  RTCPeerConnectionObserver(
    RustRTCPeerConnectionObserver *rust_observer,
    std::function<void(RustRTCPeerConnectionObserver *, webrtc::PeerConnectionInterface::SignalingState)> on_signaling_change,
    std::function<void(RustRTCPeerConnectionObserver *, webrtc::DataChannelInterface *)> on_data_channel,
    std::function<void(RustRTCPeerConnectionObserver *, internal::RTCIceCandidateInit)> on_ice_candidate)
    : rust_observer_(rust_observer),
      on_signaling_change_(on_signaling_change),
      on_data_channel_(on_data_channel),
      on_ice_candidate_(on_ice_candidate) { }

  void OnSignalingChange(webrtc::PeerConnectionInterface::SignalingState state) {
    if (on_signaling_change_) {
      on_signaling_change_(rust_observer_, state);
    }
  }

  void OnDataChannel(rtc::scoped_refptr<webrtc::DataChannelInterface> data_channel) {
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
    if (on_ice_candidate_) {
      on_ice_candidate_(rust_observer_, internal::RTCIceCandidateInit::From(candidate));
    }
  }

 private:
  RustRTCPeerConnectionObserver *rust_observer_;
  std::function<void(RustRTCPeerConnectionObserver *, webrtc::PeerConnectionInterface::SignalingState)> on_signaling_change_;
  std::function<void(RustRTCPeerConnectionObserver *, webrtc::DataChannelInterface *)> on_data_channel_;
  std::function<void(RustRTCPeerConnectionObserver *, internal::RTCIceCandidateInit)> on_ice_candidate_;
};

WEBRTC_RS_EXPORT webrtc::PeerConnectionInterface *webrtc_rs_create_peer_connection(
    webrtc::PeerConnectionFactoryInterface *factory,
    internal::RTCConfiguration *config,
    RustRTCPeerConnectionObserver *rust_observer,
    void(*on_signaling_change)(RustRTCPeerConnectionObserver *, webrtc::PeerConnectionInterface::SignalingState),
    void(*on_data_channel)(RustRTCPeerConnectionObserver *, webrtc::DataChannelInterface *),
    void(*on_ice_candidate)(RustRTCPeerConnectionObserver *, internal::RTCIceCandidateInit)) {
  auto observer = new RTCPeerConnectionObserver(rust_observer, on_signaling_change, on_data_channel, on_ice_candidate);

  return factory->CreatePeerConnection(*config, webrtc::PeerConnectionDependencies(observer)).release();
}

WEBRTC_RS_EXPORT void webrtc_rs_release_peer_connection(webrtc::PeerConnectionFactoryInterface *peer) {
  peer->Release();
}

WEBRTC_RS_EXPORT void webrtc_rs_peer_connection_create_offer(
    webrtc::PeerConnectionInterface *peer,
    RustCreateSessionDescriptionObserver *sender,
    void(*success)(RustCreateSessionDescriptionObserver *, internal::RTCSessionDescription),
    void(*error)(RustCreateSessionDescriptionObserver *, const char *)) {
  webrtc::PeerConnectionInterface::RTCOfferAnswerOptions options;
  peer->CreateOffer(new CreateSessionDescriptionObserver(sender, success, error), options);
}

WEBRTC_RS_EXPORT void webrtc_rs_peer_connection_create_answer(
    webrtc::PeerConnectionInterface *peer,
    RustCreateSessionDescriptionObserver *sender,
    void(*success)(RustCreateSessionDescriptionObserver *, internal::RTCSessionDescription),
    void(*error)(RustCreateSessionDescriptionObserver *, const char *)) {
  webrtc::PeerConnectionInterface::RTCOfferAnswerOptions options;
  peer->CreateAnswer(new CreateSessionDescriptionObserver(sender, success, error), options);
}

WEBRTC_RS_EXPORT void webrtc_rs_peer_connection_set_local_description(
    webrtc::PeerConnectionInterface *peer,
    internal::RTCSessionDescription *desc,
    RustSetSessionDescriptionObserver *sender,
    void(*success)(RustSetSessionDescriptionObserver *),
    void(*error)(RustSetSessionDescriptionObserver *, const char *)) {
  peer->SetLocalDescription(new SetSessionDescriptionObserver(sender, success, error), *desc);
}

WEBRTC_RS_EXPORT void webrtc_rs_peer_connection_set_remote_description(
    webrtc::PeerConnectionInterface *peer,
    internal::RTCSessionDescription *desc,
    RustSetSessionDescriptionObserver *sender,
    void(*success)(RustSetSessionDescriptionObserver *),
    void(*error)(RustSetSessionDescriptionObserver *, const char *)) {
  peer->SetRemoteDescription(new SetSessionDescriptionObserver(sender, success, error), *desc);
}

WEBRTC_RS_EXPORT webrtc::DataChannelInterface *webrtc_rs_peer_connection_create_data_channel(
    webrtc::PeerConnectionInterface *peer,
    char *label) {
  return peer->CreateDataChannel(std::string(label), new webrtc::DataChannelInit()).release();
}

WEBRTC_RS_EXPORT internal::SdpParseError *webrtc_rs_peer_connection_add_ice_candidate(
    webrtc::PeerConnectionInterface *peer,
    internal::RTCIceCandidateInit *candidate) {
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
