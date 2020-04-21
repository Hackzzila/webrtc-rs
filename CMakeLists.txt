cmake_minimum_required(VERSION 3.15)

cmake_policy(SET CMP0091 NEW)
set(CMAKE_CXX_STANDARD 11)

project(test)

if(MSVC)
  set(CMAKE_MSVC_RUNTIME_LIBRARY "MultiThreadedDebug")
endif()

# set(CMAKE_CXX_FLAGS_RELEASE "${CMAKE_CXX_FLAGS_RELEASE} /MT /GR-")
# set(CMAKE_CXX_FLAGS_DEBUG "${CMAKE_CXX_FLAGS_DEBUG} /MTd /GR-")

message(WARNING ${CMAKE_MSVC_RUNTIME_LIBRARY})

add_library(test STATIC test.cc)
# add_executable(test test.cc)
target_link_libraries(test PUBLIC
  ${CMAKE_CURRENT_SOURCE_DIR}/deps/webrtc/out/Default/obj/webrtc.lib
  ${CMAKE_CURRENT_SOURCE_DIR}/deps/webrtc/out/Default/obj/pc/peerconnection.lib)


  target_include_directories(test PUBLIC
  ${CMAKE_CURRENT_SOURCE_DIR}/deps/webrtc/src
  ${CMAKE_CURRENT_SOURCE_DIR}/deps/webrtc/src/third_party/abseil-cpp
  ${CMAKE_CURRENT_SOURCE_DIR}/deps/webrtc/src/third_party/libyuv/include)

target_link_libraries(test PRIVATE
  dmoguids.lib
  msdmo.lib
  secur32.lib
  winmm.lib
  wmcodecdspuuid.lib
  ws2_32.lib
)

target_compile_definitions(test PRIVATE
  -D_WINSOCKAPI_
  -DINCL_EXTRA_HTON_FUNCTIONS
  -DNOGDI
  -DNOMINMAX
  -DWEBRTC_WIN
)