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
