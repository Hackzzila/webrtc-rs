#include <stdlib.h>

#include "common.h"

extern "C" {

namespace webrtc_rs {

WEBRTC_RS_EXPORT void webrtc_rs_free(void *ptr) {
  free(ptr);
}

};

}
