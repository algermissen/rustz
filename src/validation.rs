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

    impl Semigroup for i32 {
        fn mappend(&self, b: i32) -> i32 {
            self + b
        }
    }

    fn add(s: i32, t: i32) -> i32 {
        s + t
    }

    fn div(s: i32, t: i32) -> ValidationNel<String, i32> {
        match t {
            0 => failure_nel::<String, i32>("Cannot devide by 0".to_owned()),
            _ => success_nel::<String, i32>(s / t),
        }
    }


    #[test]
    fn it_works() {
        let a = success::<i32, i32>(1);
        let b = failure::<i32, i32>(99);
        let c = success::<i32, i32>(1);

        //let r = apply2(a, c, add);
        //let z = r.get_or_else(99999);
        //println!("R {:?}", z);

        //let a = success_nel::<String, i32>(10);
        //let b = success_nel::<String, i32>(20);
        //let r = apply2(a, b, add);
        //match r {
        //    Validation::Success(v) => println!("OK {}", v),
        //    Validation::Failure(e) => println!("FAIL"),
        //}
        //let aa = success_nel::<String, i32>(10);
        //let bb = success_nel::<String, i32>(2);
        //let rr = apply2(aa, bb, div);
        //match rr {
        //    Validation::Success(v) => println!("OK {:?}", v),
        //    Validation::Failure(e) => println!("FAIL "),
        //}

        let aa = success_nel::<String, i32>(10);
        let bb = success_nel::<String, i32>(0);
        let rr = div(10, 1);
        let rr2 = div(20, 2);
        let r = apply2(rr, rr2, add);
        println!("RR: {:?}", r);
        match r {
            Validation::Success(v) => println!("OK {:?}", v),
            Validation::Failure(e) => println!("FAIL "),
        }


        assert!(true);
    }

}
