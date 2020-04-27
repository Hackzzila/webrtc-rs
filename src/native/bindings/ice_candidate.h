#ifndef WEBRTC_RS_INTERNAL_ICE_CANDIDATE_H_
#define WEBRTC_RS_INTERNAL_ICE_CANDIDATE_H_

#include "api/jsep.h"

namespace webrtc_rs {

namespace internal {

struct SdpParseError {
  char *line;
  char *description;

  static SdpParseError *From(webrtc::SdpParseError *from) {
    if (!from) return nullptr;

    auto err = new SdpParseError;
    
    err->line = new char[from->line.size() + 1];
    std::strcpy(err->line, from->line.c_str());
    
    err->description = new char[from->description.size() + 1];
    std::strcpy(err->description, from->description.c_str());

    return err;
  }
};

struct RTCIceCandidateInit {
  char *candidate;
  char *sdp_mid;
  uint16_t sdp_mline_index;

  webrtc::IceCandidateInterface *To(SdpParseError *error) {
    webrtc::SdpParseError *err = nullptr;
    auto candidate = webrtc::CreateIceCandidate(std::string(sdp_mid), sdp_mline_index, std::string(this->candidate), err);

    error = SdpParseError::From(err);
    return candidate;
  }

  static RTCIceCandidateInit From(const webrtc::IceCandidateInterface *from) {
    RTCIceCandidateInit candidate;
    candidate.sdp_mline_index = from->sdp_mline_index();

    candidate.sdp_mid = new char[from->sdp_mid().size() + 1];
    std::strcpy(candidate.sdp_mid, from->sdp_mid().c_str());

    std::string out;
    from->ToString(&out);

    auto str = new char[out.size() + 1];
    std::strcpy(str, out.c_str());

    candidate.candidate = str;

    return candidate;
  }
};

};  // namespace internal

}; // namespace webrtc_rs

#endif  // WEBRTC_RS_INTERNAL_ICE_CANDIDATE_H_
