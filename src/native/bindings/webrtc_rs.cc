#include <stdlib.h>

#include "common.h"
#include "rtc_base/logging.h"

extern "C" {

namespace webrtc_rs {

WEBRTC_RS_EXPORT void webrtc_rs_delete(void *ptr) {
  delete ptr;
}

WEBRTC_RS_EXPORT void webrtc_rs_set_log_level(int level) {
  rtc::LogMessage::LogToDebug(static_cast<rtc::LoggingSeverity>(level));
}

};

}
