pub trait Semigroup<Other: ?Sized = Self> {
  fn combine(&self, other: &Other) -> Self;
}

impl Semigroup for i32 {
  fn combine(&self, other: &i32) -> i32 {
    self + other
  }
}

impl Semigroup for String {
  fn combine(&self, other: &String) -> String {
    format!("{}{}", self, other)
  }
}