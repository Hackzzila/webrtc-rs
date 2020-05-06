#ifndef WEBRTC_RS_CREATE_SESSION_DESCRIPTION_OBSERVER_H_
#define WEBRTC_RS_CREATE_SESSION_DESCRIPTION_OBSERVER_H_

#include <stdlib.h>

#include <atomic>
#include <functional>

#include "api/jsep.h"

#include "internal_session_description.h"

namespace webrtc_rs {

struct RustCreateSessionDescriptionObserver;

class CreateSessionDescriptionObserver : public webrtc::CreateSessionDescriptionObserver {
 public:
  CreateSessionDescriptionObserver(
    RustCreateSessionDescriptionObserver *sender,
    std::function<void(RustCreateSessionDescriptionObserver *, internal::RTCSessionDescription)> success,
    std::function<void(RustCreateSessionDescriptionObserver *, const char *)> error)
    : sender_(sender),
      success_(success),
      error_(error) { }

  void OnSuccess(webrtc::SessionDescriptionInterface *desc) override {
    success_(sender_, internal::RTCSessionDescription::From(desc));
  }

  void OnFailure(webrtc::RTCError err) override {
    error_(sender_, err.message());
  }

  void AddRef() const override {
    ref_count_++;
  }

  rtc::RefCountReleaseStatus Release() const override {
    if (--ref_count_ == 0) {
      delete this;
      return rtc::RefCountReleaseStatus::kDroppedLastRef;
    }

    return rtc::RefCountReleaseStatus::kOtherRefsRemained;
  }

 private:
  mutable std::atomic<int> ref_count_{0};
  RustCreateSessionDescriptionObserver *sender_ = nullptr;
  std::function<void(RustCreateSessionDescriptionObserver *, internal::RTCSessionDescription)> success_ = nullptr;
  std::function<void(RustCreateSessionDescriptionObserver *, const char *)> error_ = nullptr;
};

};  // namespace webrtc_rs

#endif  // WEBRTC_RS_CREATE_SESSION_DESCRIPTION_OBSERVER_H_
