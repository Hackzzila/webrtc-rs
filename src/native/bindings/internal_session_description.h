#ifndef WEBRTC_RS_INTERNAL_SESSION_DESCRIPTION_H_
#define WEBRTC_RS_INTERNAL_SESSION_DESCRIPTION_H_

#include "api/jsep.h"

namespace webrtc_rs {

namespace internal {

struct RTCSessionDescription {
  webrtc::SdpType type;
  char *sdp;

  static RTCSessionDescription From(webrtc::SessionDescriptionInterface *from) {
    RTCSessionDescription desc;
    desc.type = from->GetType();

    std::string out;
    from->ToString(&out);

    auto str = reinterpret_cast<char *>(malloc(out.size() + 1));
    std::strcpy(str, out.c_str());

    desc.sdp = str;

    return desc;
  }

  operator webrtc::SessionDescriptionInterface *() const {
    return webrtc::CreateSessionDescription(type, std::string(sdp)).release();
  }
};

};  // namespace internal

}; // namespace webrtc_rs

#endif  // WEBRTC_RS_INTERNAL_SESSION_DESCRIPTION_H_
