use std::result;
use std::collections::LinkedList;
use semigroup::Semigroup;

#[derive(Clone, Debug)]
pub enum Validation<E, A> {
    Success(A),
    Failure(E),
}

pub fn success<E: Clone, A: Clone>(item: A) -> Validation<E, A> {
    Validation::Success(item)
}

pub fn failure<E: Clone, A: Clone>(e: E) -> Validation<E, A> {
    Validation::Failure(e)
}

impl<E: Clone + Semigroup, A: Clone> Validation<E, A> {
    // This is the Applicative ap function but somehow I do not need it - this indicates
    // serious misunderstanding in the code below and should be fixed at some point.
    //pub fn ap<R, F>(&self, vf: Validation<E, F>) -> Validation<E, R>
    //where
    //    F: FnOnce(A) -> R,
    //{
    //    match self {
    //        &Validation::Failure(ref e) => {
    //            match vf {
    //                Validation::Failure(e2) => Validation::Failure(e2.mappend(e.clone())),
    //                Validation::Success(_) => Validation::Failure(e.clone()),
    //            }
    //        }
    //        &Validation::Success(ref a) => {
    //            match vf {
    //                Validation::Failure(ref e2) => Validation::Failure(e2.clone()),
    //                Validation::Success(f) => Validation::Success(f(a.clone())),
    //            }
    //        }
    //    }
    //}

    pub fn map<B, F>(&self, f: F) -> Validation<E, B>
    where
        F: FnOnce(A) -> B,
    {
        match self {
            &Validation::Success(ref a) => Validation::Success(f(a.clone())),
            &Validation::Failure(ref e) => Validation::Failure::<E, B>(e.clone()),
        }
    }

    pub fn get_or_else(self, fallback: A) -> A {
        match self {
            Validation::Success(a) => a,
            Validation::Failure(_) => fallback,
        }
    }

    pub fn unwrap(self) -> A {
        match self {
            Validation::Success(a) => a,
            Validation::Failure(_) => panic!("Validation is a failure"),
        }
    }
    pub fn is_success(&self) -> bool {
        match self {
            &Validation::Success(_) => true,
            &Validation::Failure(_) => false,
        }
    }
    pub fn is_failure(&self) -> bool {
        !self.is_success()
    }
}

fn collect_err1<A, E>(a: Validation<E, A>, e: E) -> E
where
    A: Clone,
    E: Clone + Semigroup,
{
    match a {
        Validation::Failure(x) => x.mappend(e),
        Validation::Success(_) => e,
    }
}
fn collect_err2<A, B, E>(a: Validation<E, A>, b: Validation<E, B>, e: E) -> E
where
    A: Clone,
    B: Clone,
    E: Clone + Semigroup,
{
    match b {
        Validation::Failure(x) => x.mappend(collect_err1(a, e)),
        Validation::Success(_) => collect_err1(a, e),
    }
}
fn collect_err3<A, B, C, E>(
    a: Validation<E, A>,
    b: Validation<E, B>,
    c: Validation<E, C>,
    e: E,
) -> E
where
    A: Clone,
    B: Clone,
    C: Clone,
    E: Clone + Semigroup,
{
    match c {
        Validation::Failure(x) => x.mappend(collect_err2(a, b, e)),
        Validation::Success(_) => collect_err2(a, b, e),
    }
}

// Runs a function f in the success of the Validation a or passing the failure
// of a through.
pub fn apply2<A, B, R, F, E>(a: Validation<E, A>, b: Validation<E, B>, f: F) -> Validation<E, R>
where
    A: Clone,
    B: Clone,
    R: Clone,
    E: Clone + Semigroup,
    F: FnOnce(A, B) -> R,
{

    match b {
        Validation::Failure(e) => Validation::Failure(collect_err1(a, e)),
        Validation::Success(b2) => {
            let p = |a2: A| f(a2, b2);
            a.map(p)
        }
    }
}

// Runs a function f in the success of the Validations a,b,and c or passing any
// failures through, accumulating errors along the way. This means that evaluation
// is not short-circuited, but that all failures are collected using the supplied
// semigroup error type.
pub fn apply3<A, B, C, R, F, E>(
    a: Validation<E, A>,
    b: Validation<E, B>,
    c: Validation<E, C>,
    f: F,
) -> Validation<E, R>
where
    A: Clone,
    B: Clone,
    C: Clone,
    R: Clone,
    E: Clone + Semigroup,
    F: FnOnce(A, B, C) -> R,
{

    match c {
        Validation::Failure(e) => Validation::Failure(collect_err2(a, b, e)),
        Validation::Success(c2) => {
            let p = |a2: A, b2: B| f(a2, b2, c2);
            apply2(a, b, p)
        }
    }
}
// Runs a function f in the success of the Validations a,b,c, and d or passing any
// failures through, accumulating errors along the way. This means that evaluation
// is not short-circuited, but that all failures are collected using the supplied
// semigroup error type.
pub fn apply4<A, B, C, D, R, F, E>(
    a: Validation<E, A>,
    b: Validation<E, B>,
    c: Validation<E, C>,
    d: Validation<E, D>,
    f: F,
) -> Validation<E, R>
where
    A: Clone,
    B: Clone,
    C: Clone,
    D: Clone,
    R: Clone,
    E: Clone + Semigroup,
    F: FnOnce(A, B, C, D) -> R,
{

    match d {
        Validation::Failure(e) => Validation::Failure(collect_err3(a, b, c, e)),
        Validation::Success(d2) => {
            let p = |a2: A, b2: B, c2: C| f(a2, b2, c2, d2);
            apply3(a, b, c, p)
        }
    }
}

impl<T: Clone> Semigroup for LinkedList<T> {
    fn mappend(&self, b: LinkedList<T>) -> LinkedList<T> {
        let mut cloned_list = self.clone();
        for e in b.iter() {
            cloned_list.push_back(e.clone());
        }
        cloned_list
    }
}

pub type ValidationNel<E: Clone, A: Clone> = Validation<LinkedList<E>, A>;

pub fn failure_nel<E: Clone, A: Clone>(e: E) -> ValidationNel<E, A> {
    let mut li: LinkedList<E> = LinkedList::new();
    li.push_back(e);
    Validation::Failure(li)
}

pub fn success_nel<E: Clone, A: Clone>(a: A) -> ValidationNel<E, A> {
    Validation::Success(a)
}

#[cfg(test)]
mod tests {
    use validation::*;
    use super::success;
    use super::failure;

    // Let's use i32 as error counter error type
    impl Semigroup for i32 {
        fn mappend(&self, b: i32) -> i32 {
            self + b
        }
    }


    // A function that takes two parameters
    fn add2(a: i32, b: i32) -> i32 {
        a + b
    }
    // A function that takes three parameters
    fn add3(a: i32, b: i32, c: i32) -> i32 {
        a + b + c
    }

    // A function that works in the context of validation and returns an error if
    // if devision by 0 would occur.
    fn div(s: i32, t: i32) -> ValidationNel<String, i32> {
        match t {
            0 => failure_nel::<String, i32>("Cannot devide by 0".to_owned()),
            _ => success_nel::<String, i32>(s / t),
        }
    }

    #[test]
    fn it_works() {
        let a = success::<i32, i32>(1);
        let b = success::<i32, i32>(2);
        let r = apply2(a, b, add2);
        assert!(r.unwrap() == 3);

        let a = success::<i32, i32>(1);
        let b = success::<i32, i32>(2);
        let c = success::<i32, i32>(3);
        let r = apply3(a, b, c, add3);
        assert!(r.unwrap() == 6);

        let a = success::<i32, i32>(1);
        let b = success::<i32, i32>(2);
        let e = failure::<i32, i32>(3);
        let r = apply3(a, b, e, add3);
        assert!(r.is_failure());

        let r = div(10, 0);
        assert!(r.is_failure());

        let r = div(10, 2);
        assert!(r.unwrap() == 5);
    }

}
