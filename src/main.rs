extern crate pajamas;

fn main() {
    use pajamas::algebras::setoid::Setoid;
    use pajamas::algebras::semigroup::Semigroup;
    use pajamas::algebras::monoid::Monoid;

    let x: i32 = 5;
    let y: i32 = 10;

    println!("x = {}, y = {}", x, y);
    println!("x == x is {}, x == y is {}", x.equals(&x), x.equals(&y));
    println!("x.combine(y) is {}", x.combine(&y));

    println!("x.combine(y).combine(i32::empty()) is {}", x.combine(&y).combine(&Monoid::empty()));
    println!("\"hello\".to_string().combine(String::empty()) is {}", "hello".to_string().combine(&Monoid::empty()))
}
