#ifndef WEBRTC_RS_INTERNAL_SESSION_DESCRIPTION_H_
#define WEBRTC_RS_INTERNAL_SESSION_DESCRIPTION_H_

#include "api/jsep.h"

namespace webrtc_rs {

namespace internal {

struct RTCSessionDescription {
  int type;
  char *sdp;

  static RTCSessionDescription From(webrtc::SessionDescriptionInterface *from) {
    RTCSessionDescription desc;
    desc.type = static_cast<int>(from->GetType());

    std::string out;
    from->ToString(&out);

    auto str = new char[out.size() + 1];
    std::strcpy(str, out.c_str());

    desc.sdp = str;

    return desc;
  }

  operator webrtc::SessionDescriptionInterface *() const {
    return webrtc::CreateSessionDescription(static_cast<webrtc::SdpType>(type), std::string(sdp)).release();
  }
};

};  // namespace internal

}; // namespace webrtc_rs

#endif  // WEBRTC_RS_INTERNAL_SESSION_DESCRIPTION_H_
