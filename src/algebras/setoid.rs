use std::cmp::Eq;

pub trait Setoid<Other: ?Sized = Self> {
  fn equals(&self, other: &Other) -> bool;
}

impl<T> Setoid<T> for T where T: Eq {
  fn equals(&self, other: &T) -> bool {
    self.eq(other)
  }
}