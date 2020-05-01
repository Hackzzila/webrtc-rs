#include "api/create_peerconnection_factory.h"
#include "api/audio_codecs/builtin_audio_decoder_factory.h"
#include "api/audio_codecs/builtin_audio_encoder_factory.h"
#include "api/video_codecs/builtin_video_decoder_factory.h"
#include "api/video_codecs/builtin_video_encoder_factory.h"

#include "common.h"

extern "C" {

namespace webrtc_rs {

WEBRTC_RS_EXPORT void *webrtc_rs_create_peer_connection_factory() {
  // auto network_thread = rtc::Thread::CreateWithSocketServer().release();
  // network_thread->Start();

  // auto worker_thread = rtc::Thread::Create().release();
  // worker_thread->Start();

  auto signaling_thread = rtc::Thread::Create().release();
  signaling_thread->Start();

  return webrtc::CreatePeerConnectionFactory(
    nullptr,
    nullptr,
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

};  // namespace webrtc_rs

}
