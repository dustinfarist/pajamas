use std::cmp::Eq;

pub trait Setoid<Other: ?Sized = Self> {
  fn equals(&self, other: &Other) -> bool;
}

impl<T: Eq> Setoid<T> for T {
  fn equals(&self, other: &T) -> bool {
    self.eq(other)
  }
}