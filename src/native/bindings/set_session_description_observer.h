#ifndef WEBRTC_RS_SET_SESSION_DESCRIPTION_OBSERVER_H_
#define WEBRTC_RS_SET_SESSION_DESCRIPTION_OBSERVER_H_

#include <atomic>
#include <functional>

#include "api/jsep.h"

namespace webrtc_rs {

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
    if (--ref_count_ == 0) {
      delete this;
      return rtc::RefCountReleaseStatus::kDroppedLastRef;
    }

    return rtc::RefCountReleaseStatus::kOtherRefsRemained;
  }

 private:
  mutable std::atomic<int> ref_count_ = 0;
  void *sender_ = nullptr;
  std::function<void(void *)> success_ = nullptr;
  std::function<void(void *, const char *)> error_ = nullptr;
};

};  // namespace webrtc_rs

#endif  // WEBRTC_RS_SET_SESSION_DESCRIPTION_OBSERVER_H_
