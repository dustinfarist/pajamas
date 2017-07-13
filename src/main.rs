extern crate pajamas;

use pajamas::Functor;

fn main() {
    let vi: Vec<i32> = vec![1, 2, 3];
    let vs: Vec<String> = vi.map(|x| (x * 2).to_string());
    println!("{:?} => {:?}", vi, vs);
    // [1, 2, 3] => ["2", "4", "6"]

    let oi: Option<i32> = Some(5);
    let os: Option<String> = oi.map(|x| (x * 2).to_string());
    println!("{:?} => {:?}", oi, os);
    // Some(5) => Some("10")

    let oi: Option<i32> = None;
    let os: Option<String> = oi.map(|x| (x * 2).to_string());
    println!("{:?} => {:?}", oi, os);
    // None => None
}
