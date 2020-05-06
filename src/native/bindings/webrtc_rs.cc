#include <stdlib.h>

#include "common.h"
#include "rtc_base/logging.h"

extern "C" {

namespace webrtc_rs {

WEBRTC_RS_EXPORT void webrtc_rs_free(void *ptr) {
  free(ptr);
}

WEBRTC_RS_EXPORT void webrtc_rs_set_log_level(rtc::LoggingSeverity level) {
  rtc::LogMessage::LogToDebug(level);
}

};

}
