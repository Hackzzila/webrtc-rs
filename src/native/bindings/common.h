#ifndef WEBRTC_RS_COMMON_H_
#define WEBRTC_RS_COMMON_H_

#ifdef _WIN32
  #define WEBRTC_RS_EXPORT __declspec(dllexport)
#else
  #define WEBRTC_RS_EXPORT
#endif


#endif  // WEBRTC_RS_COMMON_H
