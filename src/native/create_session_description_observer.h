#ifndef WEBRTC_RS_CREATE_SESSION_DESCRIPTION_OBSERVER_H_
#define WEBRTC_RS_CREATE_SESSION_DESCRIPTION_OBSERVER_H_

#include <stdlib.h>

#include <functional>

#include "api/jsep.h"

namespace webrtc_rs {

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

};  // namespace webrtc_rs

#endif  // WEBRTC_RS_CREATE_SESSION_DESCRIPTION_OBSERVER_H_
