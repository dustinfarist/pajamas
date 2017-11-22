pub trait Semigroupoid<Other: ?Sized = Self> {
  fn compose(&self, other: &Other) -> Self;
}