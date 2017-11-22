use super::semigroup::Semigroup;

pub trait Monoid<T: ?Sized + Semigroup = Self> {
  fn empty() -> Self;
}

impl Monoid for i32 {
  fn empty() -> i32 {
    0
  }
}

impl Monoid for String {
  fn empty() -> String {
    "".to_string()
  }
}

mod pajamas {
  mod algebras {
    mod monoid {
      
    }
  }
}