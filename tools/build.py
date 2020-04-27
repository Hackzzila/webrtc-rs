from __future__ import print_function

import os
import sys
import zipfile
import subprocess
import shutil
import platform
import tempfile
import argparse
import json
import tarfile
import time
import re

try:
    from urllib.request import urlopen, urlretrieve, Request
    from urllib.error import HTTPError
except ImportError:
    from urllib import urlretrieve
    from urllib2 import urlopen, Request

DEPOT_TOOLS_WIN = 'http://storage.googleapis.com/chrome-infra/depot_tools.zip'
DEPOT_TOOLS_REPO = 'https://chromium.googlesource.com/chromium/tools/depot_tools.git'
DEPOT_TOOLS_DEST = 'deps/depot_tools'

WEBRTC_VERSION = 'm79'

VERBOSE = ''
MESSAGE = '\033[94m'
SUCCESS = '\033[92m'
WARNING = '\033[93m'
ERROR = '\033[91m'
ENDC = '\033[0m'

def log(level, message):
  if os.environ.get('CARGO') != None:
    if (level == WARNING or level == ERROR):
      print('cargo:warning=[{} v{}] {}'.format(os.environ.get('CARGO_PKG_NAME'), os.environ.get('CARGO_PKG_VERSION'), message))
    else:
      print(message)
  else:
    print(level + message + ENDC)

def ensure_target(triplet):
  arr = triplet.split('-')
  if arr[0] not in ['x86_64', 'aarch64']:
    log(ERROR, 'unsupported target "{}"'.format(triplet))
    sys.exit(1)

  if arr[2] not in ['windows', 'linux', 'darwin', 'ios', 'android']:
    log(ERROR, 'unsupported target "{}"'.format(triplet))
    sys.exit(1)

  if len(arr) >= 4:
    if arr[0] == 'windows' and arr[3] == 'msvc':
      pass
    elif arr[0] == 'linux' and arr[3] == 'gnu':
      pass
    else:
      log(ERROR, 'unsupported target "{}"'.format(triplet))
      sys.exit(1)

def triplet_to_gn_args(target):
  arr = target.split('-')
  args = []

  if arr[0] == 'x86_64':
    args.append('target_cpu="x64"')
  elif arr[0] == 'aarch64':
    args.append('target_cpu="arm64"')
  else:
    log(ERROR, 'unsupported target "{}"'.format(target))
    sys.exit(1)

  if arr[2] == 'windows':
    args.append('target_os="win"')
  elif arr[2] == 'linux':
    args.append('target_os="unix"')
  elif arr[2] == 'darwin':
    args.append('target_os="mac"')
  elif arr[2] == 'ios':
    args.append('target_os="ios"')
  elif arr[2] == 'android':
    args.append('target_os="android"')
  else:
    log(ERROR, 'unsupported target "{}"'.format(target))
    sys.exit(1)

  return args

def triplet_to_cmake_args(target_str):
  target = target_str.split('-')
  args = [ '-DTARGET_TRIPLET={}'.format(target_str) ]

  if target[2] == 'ios':
    args.extend(['-G', 'Xcode', '-DCMAKE_TOOLCHAIN_FILE=../../toolchains/iOS.cmake', '-DENABLE_VISIBILITY=TRUE', '-DDEPLOYMENT_TARGET=13.0'])
    if target[0] == 'x86_64':
      args.append('-DPLATFORM=SIMULATOR64')
    elif target[0] == 'aarch64':
      args.append('-DPLATFORM=OS64')

  return args

def copy_library(args):
  out_dir = os.path.abspath(os.path.join('target', args.target if args.target != default_target else '', 'debug' if args.debug else 'release'))
  if (os.environ.get('OUT_DIR') != None):
    out_dir = os.path.normpath(os.path.join(os.environ.get('OUT_DIR'), '..', '..', '..'))

  if not os.path.exists(out_dir):
    os.makedirs(out_dir)

  os_name = platform.system()
  copy_files = []
  if os_name == 'Windows':
    if args.debug:
      copy_files.append(os.path.abspath(os.path.join('out', args.target, 'Debug' if args.debug else 'Release', 'webrtc-rs.pdb')))

    copy_files.append(os.path.abspath(os.path.join('out', args.target, 'Debug' if args.debug else 'Release', 'webrtc-rs.dll')))

  elif os_name == 'Darwin':
    copy_files.append(os.path.abspath(os.path.join('out', args.target, 'Debug' if args.debug else 'Release', 'libwebrtc-rs.dylib')))

  else:
    copy_files.append(os.path.abspath(os.path.join('out', args.target, 'Debug' if args.debug else 'Release', 'libwebrtc-rs.so')))

  for file_name in copy_files:
    shutil.copy(file_name, out_dir)

  if os.environ.get('CARGO') != None:
    print("cargo:rustc-link-search=native=" + os.path.join(os.path.abspath('out'), args.target, 'Debug' if args.debug else 'Release'))

def build(args):
  os_name = platform.system()
  use_shell = os_name == 'Windows'

  if not os.path.exists('deps/webrtc'):
    os.makedirs('deps/webrtc')

  if os_name == 'Windows':
    os.environ['PATH'] = os.path.abspath(DEPOT_TOOLS_DEST) + ';' + os.environ.get('PATH')
    os.environ['DEPOT_TOOLS_WIN_TOOLCHAIN'] = '0'
  else:
    os.environ['PATH'] = os.environ.get('PATH') + ':' + os.path.abspath(DEPOT_TOOLS_DEST)

  if not os.path.exists(DEPOT_TOOLS_DEST):
    if os_name == 'Windows':
      os.makedirs(DEPOT_TOOLS_DEST)

      log(MESSAGE, 'Downloading depot_tools for Windows')
      temp_path = os.path.join(tempfile.mkdtemp(), 'depot_tools.zip')
      urlretrieve(DEPOT_TOOLS_WIN, temp_path)

      log(MESSAGE, 'Extracting depot_tools for Windows')
      with zipfile.ZipFile(temp_path, 'r') as zf:
        zf.extractall(DEPOT_TOOLS_DEST)

      log(MESSAGE, 'Bootstrapping depot_tools')
      process = subprocess.Popen(['gclient'], shell=use_shell)
      process.wait()
      if process.returncode != 0:
        log(ERROR, 'Failed to bootstrap depot_tools')
        return 1

      log(MESSAGE, 'Installing pywin32')
      process = subprocess.Popen(['python', '-m', 'pip', 'install', 'pywin32'], shell=use_shell)
      process.wait()
      if process.returncode != 0:
        log(ERROR, 'Failed to install pywin32')
        return 1
    else:
      log(MESSAGE, 'Cloning depot_tools')
      process = subprocess.Popen(['git', 'clone', DEPOT_TOOLS_REPO], cwd='deps', shell=use_shell)
      process.wait()
      if process.returncode != 0:
        log(ERROR, 'Failed to clone depot_tools')
        return 1

  if not os.path.exists('deps/webrtc/src'):
    webrtc_repo = "webrtc"
    if os_name == "Darwin":
      webrtc_repo = "webrtc_ios"
    elif os_name == "Linux":
      webrtc_repo = "webrtc_android"

    log(MESSAGE, 'Fetching WebRTC')
    process = subprocess.Popen(['fetch', '--nohooks', webrtc_repo], cwd='deps/webrtc', shell=use_shell)
    process.wait()
    if process.returncode != 0:
      log(ERROR, 'Failed to fetch WebRTC')
      return 1

    log(MESSAGE, 'Syncing WebRTC')

    process = subprocess.Popen(['git', 'checkout', '-f', 'branch-heads/' + WEBRTC_VERSION], cwd='deps/webrtc/src', shell=use_shell)
    process.wait()
    if process.returncode != 0:
      log(ERROR, 'Failed to checkout branch')
      return 1

    process = subprocess.Popen(['gclient', 'sync', '-D'], cwd='deps/webrtc/src', shell=use_shell)
    process.wait()
    if process.returncode != 0:
      log(ERROR, 'Failed to sync WebRTC')
      return 1

    if os_name == 'Linux':
      process = subprocess.Popen(['./build/install-build-deps.sh'], cwd='deps/webrtc/src', shell=use_shell)
      process.wait()
      if process.returncode != 0:
        log(ERROR, 'Failed to install build deps')
        return 1

    log(MESSAGE, 'WebRTC up to date')

  out_dir = "out/{}/{}".format(args.target, "Debug" if args.debug else "Release")
  gn_args = [
    'rtc_build_examples=false',
    'rtc_include_pulse_audio=false',
    'rtc_include_tests=false',
    'rtc_build_tools=false',
  ]
  if args.debug:
    gn_args.append('is_debug=true')
  else:
    gn_args.append('is_debug=false')

  if os_name == "Windows":
    gn_args.append('is_clang=false')

  gn_args.extend(triplet_to_gn_args(args.target))

  if not os.path.exists('deps/webrtc/' + out_dir):
    log(MESSAGE, 'Running GN')
    process = subprocess.Popen(['gn', 'gen', '../' + out_dir, '--args=' + ' '.join(gn_args)], cwd='deps/webrtc/src', shell=use_shell)
    process.wait()
    if process.returncode != 0:
      log(ERROR, 'GN failed')
      return 1

  log(MESSAGE, 'Running Ninja')
  process = subprocess.Popen(['ninja', '-C', '../' + out_dir], cwd='deps/webrtc/src', shell=use_shell)
  process.wait()
  if process.returncode != 0:
    log(ERROR, 'Ninja failed')
    return 1

  build_dir = 'build/' + args.target
  if not os.path.exists(build_dir):
    os.makedirs(build_dir)

  build_type = "Debug" if args.debug else "Release"
  cmake_args = ['cmake', '-DCMAKE_BUILD_TYPE={}'.format(build_type), '../..']
  cmake_args.extend(triplet_to_cmake_args(args.target))

  log(MESSAGE, 'Running CMake')
  process = subprocess.Popen(cmake_args, cwd=build_dir, shell=use_shell)
  process.wait()
  if process.returncode != 0:
    log(ERROR, 'CMake failed')
    return 1

  # if os_name == 'Windows':
  log(MESSAGE, 'Running CMake build')
  process = subprocess.Popen(['cmake', '--build', '.', '--config', build_type], cwd=build_dir, shell=use_shell)
  process.wait()
  if process.returncode != 0:
    log(ERROR, 'CMake build failed')
    return 1
  # else:
  #   log(MESSAGE, 'Running make')
  #   process = subprocess.Popen(['cmake'], cwd='build', shell=use_shell)
  #   process.wait()
  #   if process.returncode != 0:
  #     log(ERROR, 'make failed')
  #     return 1

  out_dir = os.path.abspath(os.path.join('out', args.target, 'Debug' if args.debug else 'Release'))
  if not os.path.exists(out_dir):
    os.makedirs(out_dir)

  os_name = platform.system()
  copy_files = []
  if os_name == 'Windows':
    if args.debug:
      copy_files.append(os.path.abspath(os.path.join('build', args.target, 'Debug' if args.debug else 'Release', 'webrtc-rs.pdb')))

    copy_files.append(os.path.abspath(os.path.join('build', args.target, 'Debug' if args.debug else 'Release', 'webrtc-rs.lib')))
    copy_files.append(os.path.abspath(os.path.join('build', args.target, 'Debug' if args.debug else 'Release', 'webrtc-rs.dll')))

  elif os_name == 'Darwin':
    copy_files.append(os.path.abspath(os.path.join('build', args.target, 'Debug' if args.debug else 'Release', 'libwebrtc-rs.dylib')))

  else:
    copy_files.append(os.path.abspath(os.path.join('build', args.target, 'Debug' if args.debug else 'Release', 'libwebrtc-rs.so')))

  for file_name in copy_files:
    shutil.copy(file_name, out_dir)

  copy_library(args)

  log(SUCCESS, 'Successfully built')

  return 0

def download(args):
  try:
    repo = args.repo.split('/')
    req = Request('https://api.github.com/repos/{}/{}/releases'.format(repo[len(repo)-2], repo[len(repo)-1]))
    req.add_header('User-Agent', 'webrtc-rs builder')

    res = urlopen(req)
    releases = json.loads(res.read())

    for release in releases:
      if release[u'tag_name'] == 'v{}'.format(args.version):
        for asset in release[u'assets']:
          if asset[u'name'] == '{}.tar.gz'.format(args.target):
            log(MESSAGE, 'Downloading {} {}'.format(asset[u'name'], release[u'tag_name']))
            temp_path = os.path.join(tempfile.mkdtemp(), asset[u'name'])
            urlretrieve(asset[u'browser_download_url'], temp_path)

            log(MESSAGE, 'Extracting {} {}'.format(asset[u'name'], release[u'tag_name']))
            with tarfile.open(temp_path, 'r') as tf:
              tf.extractall('out')

            copy_library(args)
            log(SUCCESS, 'Successfully downloaded')
            return 0
  except HTTPError as err:
    log(WARNING, "download failed with code {}: {}".format(err.code, err.reason))
  except:
    log(WARNING, "download failed")

  return 1

def clean(args):
  if os.path.exists('out'):
    shutil.rmtree('out')

  if os.path.exists('build'):
    shutil.rmtree('build')

  if os.path.exists('deps/webrtc/out'):
    shutil.rmtree('deps/webrtc/out')

  if args.clean_src and os.path.exists(DEPOT_TOOLS_DEST):
    shutil.rmtree(DEPOT_TOOLS_DEST)

  if args.clean_src and os.path.exists('deps/webrtc'):
    shutil.rmtree('deps/webrtc')

  return 0

def main():
  if os.environ.get('CARGO_CFG_TARGET_ARCH') == 'wasm32': return 0

  parser = argparse.ArgumentParser('build tool for webrtc-rs')
  parser.add_argument('action', choices=['build', 'download', 'downloadOrBuild', 'clean'])
  parser.add_argument('--debug', action='store_true', default=os.environ.get('PROFILE') == 'debug', help="enables debug build")
  parser.add_argument('--clean-src', action='store_true', help="removes the source code directories for dependencies")
  parser.add_argument('--version', type=str, default=os.environ.get('CARGO_PKG_VERSION'), help="sets the version to download")
  parser.add_argument('--repo', type=str, default=os.environ.get('CARGO_PKG_REPOSITORY'), help="repo url to download from")
  parser.add_argument('--target', type=str, default=os.environ.get('TARGET'), help="rust target to compile for")

  args = parser.parse_args()

  global default_target
  process = subprocess.Popen(['rustc', '--version', '--verbose'], stdout=subprocess.PIPE, shell=True if platform.system() == "Windows" else False)
  out, err = process.communicate()
  default_target = re.compile(r"host:\s*(.+)").search(out).group(1)
  if not args.target:
    args.target = default_target

  ensure_target(args.target)

  if (args.action == 'build'):
    return build(args)
  elif (args.action == 'download'):
    if download(args) != 0:
      log(WARNING, "couldn't find prebuilt for {} v{}".format(args.target, args.version))
  elif (args.action == 'downloadOrBuild'):
    if not os.path.exists('build/' + args.target):
      if download(args) != 0:
        log(WARNING, "no prebuilts available for {} v{} - building from source".format(args.target, args.version))
        return build(args)
    else:
      log(WARNING, "local build found, skipping download")
      return build(args)

    return 0
  elif (args.action == 'clean'):
    return clean(args)

if __name__ == '__main__':
  sys.exit(main())
