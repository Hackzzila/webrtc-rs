use libc::{size_t};

#[repr(C)]
pub struct DataBuffer {
  pub data: *const u8,
  pub len: size_t,
  pub binary: bool,
}
