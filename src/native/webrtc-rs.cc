#include <iostream>
#include <functional>
#include <chrono>
#include <thread>

#include "api/create_peerconnection_factory.h"
#include "api/audio_codecs/builtin_audio_decoder_factory.h"
#include "api/audio_codecs/builtin_audio_encoder_factory.h"
#include "api/video_codecs/builtin_video_decoder_factory.h"
#include "api/video_codecs/builtin_video_encoder_factory.h"

#ifdef _WIN32
#define WEBRTC_RS_EXPORT __declspec(dllexport)
#else
#define WEBRTC_RS_EXPORT
#endif

class Observer : public webrtc::PeerConnectionObserver {
 public:
  void OnSignalingChange(webrtc::PeerConnectionInterface::SignalingState new_state) {
    std::cout << "OnSignalingChange" << std::this_thread::get_id() << std::endl;
  }

  void OnDataChannel(rtc::scoped_refptr<webrtc::DataChannelInterface> data_channel) {
    std::cout << "OnDataChannel" << std::endl;
  }

  void OnRenegotiationNeeded() {
    std::cout << "OnRenegotiationNeeded" << std::this_thread::get_id() << std::endl;
  }

  void OnIceGatheringChange(webrtc::PeerConnectionInterface::IceGatheringState new_state) {
    std::cout << "OnIceGatheringChange" << std::this_thread::get_id() << std::endl;
  }

  void OnIceCandidate(const webrtc::IceCandidateInterface *canidate) {
    std::cout << "OnIceCanidate" << std::endl;
  }
};

class CreateSessionDescriptionObserver : public webrtc::CreateSessionDescriptionObserver {
 public:
  CreateSessionDescriptionObserver(void *sender, std::function<void(void *, const char *, char *)> success, std::function<void(void *, const char *)> error) : sender_(sender), success_(success), error_(error) { }

  void OnSuccess(webrtc::SessionDescriptionInterface *desc) override {
    std::string out;
    desc->ToString(&out);

    char *str = reinterpret_cast<char *>(malloc(out.size() + 1));
    std::strcpy(str, out.c_str());
    success_(sender_, webrtc::SdpTypeToString(desc->GetType()), str);
  }

  void OnFailure(webrtc::RTCError err) override {
    error_(sender_, err.message());
  }

  void AddRef() const override {
    ref_count_++;
  }

  rtc::RefCountReleaseStatus Release() const override {
    ref_count_--;
    if (ref_count_ == 0) {
      delete this;
      return rtc::RefCountReleaseStatus::kDroppedLastRef;
    }

    return rtc::RefCountReleaseStatus::kOtherRefsRemained;
  }

 private:
  mutable int ref_count_ = 0;
  void *sender_ = nullptr;
  std::function<void(void *, const char *, char *)> success_ = nullptr;
  std::function<void(void *, const char *)> error_ = nullptr;
};

class SetSessionDescriptionObserver : public webrtc::SetSessionDescriptionObserver {
 public:
  SetSessionDescriptionObserver(void *sender, std::function<void(void *)> success, std::function<void(void *, const char *)> error) : sender_(sender), success_(success), error_(error) { }

  void OnSuccess() override {
    success_(sender_);
  }

  void OnFailure(webrtc::RTCError err) override {
    error_(sender_, err.message());
  }

  void AddRef() const override {
    ref_count_++;
  }

  rtc::RefCountReleaseStatus Release() const override {
    ref_count_--;
    if (ref_count_ == 0) {
      delete this;
      return rtc::RefCountReleaseStatus::kDroppedLastRef;
    }

    return rtc::RefCountReleaseStatus::kOtherRefsRemained;
  }

 private:
  mutable int ref_count_ = 0;
  void *sender_ = nullptr;
  std::function<void(void *)> success_ = nullptr;
  std::function<void(void *, const char *)> error_ = nullptr;
};

extern "C" {
  WEBRTC_RS_EXPORT void webrtc_rs_free(void *ptr) {
    free(ptr);
  }

  WEBRTC_RS_EXPORT void *webrtc_rs_create_peer_connection_factory() {
    auto network_thread = rtc::Thread::Create().release();
    network_thread->Start();

    auto worker_thread = rtc::Thread::Create().release();
    worker_thread->Start();

    auto signaling_thread = rtc::Thread::Create().release();
    signaling_thread->Start();

    return webrtc::CreatePeerConnectionFactory(
      network_thread,
      worker_thread,
      signaling_thread,
      nullptr,
      webrtc::CreateBuiltinAudioEncoderFactory(),
      webrtc::CreateBuiltinAudioDecoderFactory(),
      webrtc::CreateBuiltinVideoEncoderFactory(),
      webrtc::CreateBuiltinVideoDecoderFactory(),
      nullptr,
      nullptr).release();
  }

  WEBRTC_RS_EXPORT void webrtc_rs_release_peer_connection_factory(void *factory_ptr) {
    reinterpret_cast<webrtc::PeerConnectionFactoryInterface *>(factory_ptr)->Release();
  }

  WEBRTC_RS_EXPORT void *webrtc_rs_create_rtc_configuration() {
    return new webrtc::PeerConnectionInterface::RTCConfiguration();
  }

  WEBRTC_RS_EXPORT void webrtc_rs_delete_rtc_configuration(void *config_ptr) {
    delete reinterpret_cast<webrtc::PeerConnectionInterface::RTCConfiguration *>(config_ptr);
  }

  WEBRTC_RS_EXPORT void *webrtc_rs_create_peer_connection(void *factory_ptr, void *config_ptr) {
    auto factory = reinterpret_cast<webrtc::PeerConnectionFactoryInterface *>(factory_ptr);
    // auto config = reinterpret_cast<webrtc::PeerConnectionInterface::RTCConfiguration *>(config_ptr);

    webrtc::PeerConnectionInterface::RTCConfiguration config;
    config.sdp_semantics = webrtc::SdpSemantics::kUnifiedPlan;
    config.enable_dtls_srtp = false;
    config.enable_rtp_data_channel = true;
    webrtc::PeerConnectionInterface::IceServer server;
    server.uri = "stun:stun.l.google.com:19302";
    config.servers.push_back(server);

    Observer *observer = new Observer();

    return factory->CreatePeerConnection(config, webrtc::PeerConnectionDependencies(observer)).release();
    // auto peer =  factory->CreatePeerConnection(config, nullptr, nullptr, observer);

    // auto chan = peer->CreateDataChannel("dc", NULL);
    // if (!chan) {
    //   std::cout << "CHAN NULL!" << std::endl;
    // }

    // return peer.release();
  }

  WEBRTC_RS_EXPORT void webrtc_rs_release_peer_connection(void *peer_ptr) {
    reinterpret_cast<webrtc::PeerConnectionInterface *>(peer_ptr)->Release();
  }

  WEBRTC_RS_EXPORT void webrtc_rs_peer_connection_create_offer(void *peer_ptr, void *sender, void(*success)(void *, const char *, char *), void(*error)(void *, const char *)) {
    auto peer = reinterpret_cast<webrtc::PeerConnectionInterface *>(peer_ptr);

    // // const webrtc::DataChannelInit *init = new webrtc::DataChannelInit();
    // auto chan = peer->CreateDataChannel("dc", NULL);
    // if (!chan) {
    //   std::cout << "CHAN NULL" << std::endl;
    //   return;
    // }

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

    auto type = webrtc::SdpTypeFromString(std::string(sdp_str));
    if (!type) {
      error(sender, "invalid type_str passed into peer_connection_set_local_description");
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

    // auto ob = new SetSessionDescriptionObserver()
  }

  // void test() {
  //
  //   Observer observer;

  //   // webrtc::PeerConnectionDependencies dependencies = {};

  //   auto peer = factory->CreatePeerConnection(config, webrtc::PeerConnectionDependencies(&observer));

  //   std::cout << "Foo" << std::endl;
  // }
}

// void main() {
//   auto thread1 = rtc::Thread::Create().release();
//   thread1->Start();

//   auto thread2 = rtc::Thread::Create().release();
//   thread2->Start();

//   auto thread3 = rtc::Thread::Create().release();
//   thread3->Start();

//   auto factory = webrtc::CreatePeerConnectionFactory(
//       thread1,
//       thread2,
//       thread3,
//       nullptr,
//       webrtc::CreateBuiltinAudioEncoderFactory(),
//       webrtc::CreateBuiltinAudioDecoderFactory(),
//       webrtc::CreateBuiltinVideoEncoderFactory(),
//       webrtc::CreateBuiltinVideoDecoderFactory(),
//       nullptr,
//       nullptr);

//   webrtc::PeerConnectionInterface::RTCConfiguration config;
//   config.sdp_semantics = webrtc::SdpSemantics::kUnifiedPlan;
//   config.enable_dtls_srtp = false;
//   config.enable_rtp_data_channel = true;
//   webrtc::PeerConnectionInterface::IceServer server;
//   server.uri = "stun:stun.l.google.com:19302";
//   config.servers.push_back(server);

//   Observer *observer = new Observer();

//   auto peer = factory->CreatePeerConnection(config, webrtc::PeerConnectionDependencies(observer));

//   // auto chan = peer->CreateDataChannel("dc", NULL);
//   // if (!chan) {
//   //   std::cout << "CHAN NULL" << std::endl;
//   //   return;
//   // }

//   peer->Close();

//   webrtc::PeerConnectionInterface::RTCOfferAnswerOptions options;

//   auto observer2 = new CreateSessionDescriptionObserver([](char *foo) {
//     std::cout << "SUCCESS!" << std::endl;
//   }, [](const char *err) {
//     std::cout << "err!" << std::endl;
//   });

//   peer->CreateOffer(observer2, options);

//   std::cout << "MAIN " << std::this_thread::get_id() << std::endl;

//   while (true) {
//     std::this_thread::sleep_for(std::chrono::seconds(5));
//   }

//   // void *factory = create_peer_connection_factory();
//   // void *config = create_rtc_configuration();
//   // void *peer = create_peer(factory, config);

//   // peer_create_offer(peer, [](char *foo) {
//   //   std::cout << "SUCCESS!" << std::endl;
//   // }, [](const char *err) {
//   //   std::cout << "err!" << std::endl;
//   // });

//   // while(true) { }
// }