use libc::c_void;

#[link(name = "webrtc-rs")]
extern {
  pub fn webrtc_rs_free(ptr: *mut c_void);
}
