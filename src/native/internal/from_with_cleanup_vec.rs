pub trait FromWithCleanupVec<T> {
  fn from_with_cleanup_vec(value: T) -> (Self, Vec<u8>) where Self: std::marker::Sized;
}
