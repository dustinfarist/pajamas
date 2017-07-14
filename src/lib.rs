pub trait HKT<In, Out> {
    type R; // This will be the container type with Out, e.g. Vec<Out>, Option<Out>, etc.
}

pub trait Functor<In, Out>: HKT<In, Out> {
    fn map<F>(&self, f: F) -> Self::R where F: Fn(&In) -> Out;
}

// Implementations for Vec<T>
impl<In, Out> HKT<In, Out> for Vec<In> {
    type R = Vec<Out>;
}

impl<In, Out> Functor<In, Out> for Vec<In> {
    fn map<F>(&self, f: F) -> Vec<Out>
        where F: Fn(&In) -> Out
    {
        let mut result = Vec::with_capacity(self.len());
        for value in self {
            result.push(f(value));
        }
        result
    }
}

// Implementations for Option<T>
impl<In, Out> HKT<In, Out> for Option<In> {
    type R = Option<Out>;
}

impl<In, Out> Functor<In, Out> for Option<In> {
    fn map<F>(&self, f: F) -> Option<Out>
        where F: Fn(&In) -> Out
    {
        match *self {
            Some(ref value) => Some(f(value)),
            None => None,
        }
    }
}

trait HKT<B> {
    type A;
    type T;
}

trait Functor<B>: HKT<B> {
    fn map<F>(&self, F) -> Self::T where F: Fn(&Self::A) -> B;
}

trait Apply<B>: Functor<B> {
    fn ap<F>(&self, <Self as HKT<F>>::T) -> <Self as HKT<B>>::T where Self: HKT<F>, F: Fn(&<Self as HKT<B>>::A) -> B;
}

trait Applicative<B>: Apply<B> {
    fn pure_value(B) -> Self::T;// where Self: HKT<B, A=B>;
}

// Vector implementations
impl<A, B> HKT<B> for Vec<A> {
    type A = A;
    type T = Vec<B>;
}

impl<A, B> Functor<B> for Vec<A> {
    fn map<F>(&self, f: F) -> Vec<B>
        where F: Fn(&A) -> B
    {
        let mut result = Vec::with_capacity(self.len());
        for value in self {
            result.push(f(value));
        }
        result
    }
}

impl<A, B> Apply<B> for Vec<A> {
    fn ap<F>(&self, fs: <Self as HKT<F>>::T) -> Vec<B>
        where F: Fn(&A) -> B
    {
        let mut result = Vec::new();
        for (i, f) in fs.into_iter().enumerate() {
            let v = (f)(&self[i]);
            result.push(v);
        }
        result
    }
}

impl<A, B> Applicative<B> for Vec<A> {
    fn pure_value(value: B) -> Self::T {
        vec![value]
    }
}

// Option implementations
impl<A, B> HKT<B> for Option<A> {
    type A = A;
    type T = Option<B>;
}

impl<A, B> Functor<B> for Option<A> {
    fn map<F>(&self, f: F) -> Option<B>
        where F: Fn(&A) -> B
    {
        match *self {
            Some(ref value) => Some(f(value)),
            None => None,
        }
    }
}

impl<A, B> Apply<B> for Option<A> {
    fn ap<F>(&self, ff: <Self as HKT<F>>::T) -> Option<B>
        where F: Fn(&A) -> B
    {
        match *self {
            Some(ref value) => {
                match ff {
                    Some(f) => Some(f(value)),
                    None => None,
                }
            }
            None => None,
        }
    }
}

impl<A, B> Applicative<B> for Option<A> {
    fn pure_value(value: B) -> Self::T {
        Some(value)
    }
}

fn main() {
    let vi: Vec<i32> = vec![1, 2, 3];
    let vs: Vec<String> = vi.map(|x| (x * 2).to_string());
    println!("{:?} => {:?}", vi, vs);
    // [1, 2, 3] => ["2", "4", "6"]

    let oi: Option<i32> = Some(5);
    let os: Option<String> = oi.map(|x| (x * 2).to_string());
    println!("{:?} => {:?}", oi, os);
    // Some(5) => Some("10")

    let f: &Fn(&i32) -> i32 = &|x| x * 3;
    let o = oi.ap(Some(f));
    println!("{:?}", o);

    let o = oi.ap::<&Fn(&i32) -> i32>(None);
    println!("{:?}", o);

    let oi: Option<i32> = None;
    let os: Option<String> = oi.map(|x| (x * 2).to_string());
    println!("{:?} => {:?}", oi, os);
    // None => None

    let o = <Option<i32> as Applicative<i32>>::pure_value(3);
    println!("{:?}", o)
}
