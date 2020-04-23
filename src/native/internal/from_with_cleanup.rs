use libc::c_char;

pub trait FromWithCleanup<T> {
  fn from_with_cleanup(config: T, c_strings: &mut Vec<*mut c_char>) -> Self;
}
