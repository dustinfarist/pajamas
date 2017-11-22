use super::semigroupoid::Semigroupoid;

pub trait Category<T: Semigroupoid = Self> {
  fn id() -> Self;
}