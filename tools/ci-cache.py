import os
import sys
import shutil

if not os.path.exists('deps/CACHE_VERSION'):
  if os.path.exists('deps/depot_tools'):
    shutil.rmtree('deps/depot_tools')
  
  if os.path.exists('deps/webrtc'):
    shutil.rmtree('deps/webrtc')

  f = open('deps/CACHE_VERSION', 'w')
  f.write(sys.argv[1])
  f.close()

  exit(0)

f = open('deps/CACHE_VERSION', 'r')
if f.read() != sys.argv[1]:
  if os.path.exists('deps/depot_tools'):
    shutil.rmtree('deps/depot_tools')
  
  if os.path.exists('deps/webrtc'):
    shutil.rmtree('deps/webrtc')

  f = open('deps/CACHE_VERSION', 'w')
  f.write(sys.argv[1])
  f.close()
