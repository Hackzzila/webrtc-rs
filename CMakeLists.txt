cmake_minimum_required(VERSION 3.15)

cmake_policy(SET CMP0091 NEW)
set(CMAKE_CXX_STANDARD 11)

project(webrtc-rs)

if(MSVC)
  set(CMAKE_MSVC_RUNTIME_LIBRARY "MultiThreaded$<$<CONFIG:Debug>:Debug>")
endif()

add_library(webrtc-rs SHARED
            src/native/bindings/peer_connection_factory.cc
            src/native/bindings/peer_connection.cc
            src/native/bindings/webrtc_rs.cc)

target_include_directories(webrtc-rs PUBLIC
  ${CMAKE_CURRENT_SOURCE_DIR}/deps/webrtc/src
  ${CMAKE_CURRENT_SOURCE_DIR}/deps/webrtc/src/third_party/abseil-cpp
  ${CMAKE_CURRENT_SOURCE_DIR}/deps/webrtc/src/third_party/libyuv/include)

if(WIN32)
  target_link_libraries(webrtc-rs PUBLIC
    ${CMAKE_CURRENT_SOURCE_DIR}/deps/webrtc/out/${CMAKE_BUILD_TYPE}/obj/webrtc.lib
    ${CMAKE_CURRENT_SOURCE_DIR}/deps/webrtc/out/${CMAKE_BUILD_TYPE}/obj/pc/peerconnection.lib)

  target_link_libraries(webrtc-rs PRIVATE
    dmoguids.lib
    msdmo.lib
    secur32.lib
    winmm.lib
    wmcodecdspuuid.lib
    ws2_32.lib
  )

  if(CMAKE_BUILD_TYPE STREQUAL "Debug")
    target_link_libraries(webrtc-rs PRIVATE MSVCRTD.lib)
  endif()

  target_compile_definitions(webrtc-rs PRIVATE
    -D_WINSOCKAPI_
    -DINCL_EXTRA_HTON_FUNCTIONS
    -DNOGDI
    -DNOMINMAX
    -DWEBRTC_WIN
    -D_ITERATOR_DEBUG_LEVEL=0
  )
elseif(APPLE)
  set(libwebrtc_source_dir ${CMAKE_CURRENT_SOURCE_DIR}/deps/webrtc)

  set(libc++_include_dir "${libwebrtc_source_dir}/src/buildtools/third_party/libc++/trunk/include")
  set(libc++abi_include_dir "${libwebrtc_source_dir}/src/buildtools/third_party/libc++abi/trunk/include")

  # libc++
  # -----------------------------------------------------------------------------

  set(libwebrtc_binary_dir ${CMAKE_CURRENT_SOURCE_DIR}/deps/webrtc/out/${CMAKE_BUILD_TYPE})
  set(libwebrtc_src_dir ${CMAKE_CURRENT_SOURCE_DIR}/deps/webrtc)

  add_library(libc++ OBJECT IMPORTED)
  add_dependencies(libc++ libwebrtc)

  set(libc++_objects
    algorithm.o
    any.o
    bind.o
    chrono.o
    condition_variable.o
    debug.o
    exception.o
    functional.o
    future.o
    hash.o
    ios.o
    iostream.o
    locale.o
    memory.o
    mutex.o
    new.o
    optional.o
    random.o
    regex.o
    shared_mutex.o
    stdexcept.o
    string.o
    strstream.o
    system_error.o
    thread.o
    typeinfo.o
    utility.o
    valarray.o
    variant.o
    vector.o
  )
  list(TRANSFORM libc++_objects PREPEND ${libwebrtc_binary_dir}/obj/buildtools/third_party/libc++/libc++/)

  set_property(TARGET libc++ APPEND PROPERTY IMPORTED_CONFIGURATIONS DEBUG)
  set_target_properties(libc++ PROPERTIES IMPORTED_OBJECTS_DEBUG "${libc++_objects}" IMPORTED_OBJECTS "${libc++_objects}")

  # NOTE(mroberts): I would like this to be INTERFACE.
  #
  #   https://gitlab.kitware.com/cmake/cmake/issues/15052
  #
  # target_include_directories(libc++ SYSTEM INTERFACE "${libc++_include_dir}")

  # libc++abi
  # -----------------------------------------------------------------------------

  add_library(libc++abi OBJECT IMPORTED)
  add_dependencies(libc++abi libwebrtc)

  set(libc++abi_objects
    abort_message.o
    cxa_aux_runtime.o
    cxa_default_handlers.o
    cxa_demangle.o
    cxa_exception.o
    cxa_exception_storage.o
    cxa_guard.o
    cxa_handlers.o
    cxa_personality.o
    cxa_unexpected.o
    cxa_vector.o
    cxa_virtual.o
    fallback_malloc.o
    private_typeinfo.o
    stdlib_exception.o
    stdlib_stdexcept.o
    stdlib_typeinfo.o
  )
  list(TRANSFORM libc++abi_objects PREPEND ${libwebrtc_binary_dir}/obj/buildtools/third_party/libc++abi/libc++abi/)

  set_property(TARGET libc++abi APPEND PROPERTY IMPORTED_CONFIGURATIONS DEBUG)
  set_target_properties(libc++abi PROPERTIES IMPORTED_OBJECTS_DEBUG "${libc++abi_objects}" IMPORTED_OBJECTS "${libc++abi_objects}")

  # NOTE(mroberts): Workaround for
  #
  #   https://gitlab.kitware.com/cmake/cmake/issues/15052
  #
  # is to include all the header files here.
  target_include_directories(webrtc-rs SYSTEM PRIVATE
    ${libc++_include_dir}
    ${libc++abi_include_dir}
  )

  # NOTE(mroberts): On Linux and macOS, we statically link the same libc++ and
  # libc++abi libraries that libwebrtc builds against.
  target_link_libraries(webrtc-rs PRIVATE
    -lc
    -lm
    libc++
    libc++abi
  )

  target_compile_options(webrtc-rs PRIVATE
    -fno-rtti
    # -fvisibility=hidden
    -nostdinc++
    -nodefaultlibs
    -Wall
    -Wextra
    -Wno-missing-field-initializers
    -Wno-uninitialized
    -Wno-unused-local-typedefs
    -Wno-unused-variable
    -Wpedantic
  )

  target_compile_definitions(webrtc-rs PRIVATE
    # -D_LIBCPP_ABI_UNSTABLE
    -DWEBRTC_POSIX=1
  )

  if(CMAKE_BUILD_TYPE STREQUAL "Debug")
    target_compile_options(webrtc-rs PRIVATE
      -g
    )
  endif()

  target_link_libraries(webrtc-rs PRIVATE
    # "-framework AppKit"
    # "-framework AVFoundation
    "-framework Cocoa"
    "-framework CoreAudio"
    "-framework AudioToolbox"
  )

  target_compile_options(webrtc-rs PRIVATE
    -Weverything
    -Wno-c++98-compat
    -Wno-c++98-compat-pedantic
    -Wno-documentation
    -Wno-exit-time-destructors
    -Wno-float-conversion
    -Wno-global-constructors
    -Wno-padded
    -Wno-shadow
    -Wno-shadow-field-in-constructor
    -Wno-shorten-64-to-32
    -Wno-sign-conversion
    -Wno-thread-safety-negative
    -Wno-unused-template
    -Wno-weak-vtables
  )

  target_compile_definitions(webrtc-rs PRIVATE
    -D_LIBCPP_ABI_UNSTABLE
    -DWEBRTC_MAC
    -DWEBRTC_IOS
  )

  target_link_libraries(webrtc-rs PUBLIC
    ${CMAKE_CURRENT_SOURCE_DIR}/deps/webrtc/out/${CMAKE_BUILD_TYPE}/obj/libwebrtc.a
    ${CMAKE_CURRENT_SOURCE_DIR}/deps/webrtc/out/${CMAKE_BUILD_TYPE}/obj/pc/libpeerconnection.a)
endif()