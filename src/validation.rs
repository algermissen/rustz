use semigroup::Semigroup;

use std::collections::LinkedList;

pub enum Validation<E, A> {
    Success(A),
    Failure(E),
}

pub fn success<E: Clone, A>(item: A) -> Validation<E, A> {
    Validation::Success(item)
}

pub fn failure<E, A: Clone>(e: E) -> Validation<E, A> {
    Validation::Failure(e)
}

pub type ValidationNel<E, A> = Validation<LinkedList<E>, A>;
pub fn failure_nel<E, A: Clone>(e: E) -> ValidationNel<E, A> {
    let mut li: LinkedList<E> = LinkedList::new();
    li.push_back(e);
    Validation::Failure(li)
}


pub fn success_nel<E: Clone, A>(item: A) -> ValidationNel<E, A> {
    Validation::Success(item)
}



impl<E: Semigroup + Copy, A: Copy> Validation<E, A> {
    pub fn ap<B, F>(&self, x: Validation<E, F>) -> Validation<E, B>
        where F: FnOnce(A) -> B
    {
        match (self, x) {
            (&Validation::Success(_), Validation::Failure(e2)) => Validation::Failure::<E, B>(e2),
            (&Validation::Failure(ref e1), Validation::Failure(ref e2)) => {
                Validation::Failure::<E, B>(e1.mappend(*e2))
            }
            (&Validation::Failure(ref e1), Validation::Success(_)) => {
                Validation::Failure::<E, B>(*e1)
            }
            (&Validation::Success(a), Validation::Success(f)) => {
                let y = f(a);
                Validation::Success::<E, B>(y)
            }
        }
    }

    pub fn map<B, F>(&self, f: F) -> Validation<E, B>
        where F: FnOnce(A) -> B
    {
        match self {
            &Validation::Success(ref a) => Validation::Success(f(*a)),
            &Validation::Failure(ref e) => Validation::Failure::<E, B>(*e),
        }
    }

    pub fn get_or_else(&self, x: A) -> A {
        match self {
            &Validation::Success(ref a) => *a,
            &Validation::Failure(_) => x,
        }
    }
}

pub fn apply2<E: Semigroup + Copy, A: Copy, B: Copy, R, F: FnOnce(A, B) -> R>
    (a: Validation<E, A>,
     b: Validation<E, B>,
     f: F)
     -> Validation<E, R> {
    let fv = b.map(move |b2: B| {
                       let p = move |u: A| f(u, b2);
                       p
                   });
    a.ap(fv)
}

//pub fn apply3<E: Semigroup + Copy, A: Copy, B: Copy, C:Copy,R, F: FnOnce(A, B) -> R>
//    (a: Validation<E, A>,
//     b: Validation<E, B>,
//     c: Validation<E, C>,
//     f: F)
//     -> Validation<E, R> {
//    let fv = c.map(move |c2: C| {
//                       let p = move |a2: A, b2: B| f(a2, b2,c2);
//                       p
//                   });
//    apply2(a,b,
//    a.ap(fv)
//}

//
//fn apply3<A, B, C, R, E: Clone, F: FnOnce(A, B, C) -> R>(a: Validation<A, E>,
//                                                         b: Validation<B, E>,
//                                                         c: Validation<C, E>,
//                                                         f: F)
//                                                         -> Validation<R, E> {
//    match c {
//        Err(x) => Err(fm(b, fm(a, x))),
//        Ok(x) => {
//            let p = |u: A, v: B| f(u, v, ok_c);
//            apply2(a, b, p)
//        }
//    }
//}

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


    #[test]
    fn it_works() {
        let a = success::<i32, i32>(1);
        let b = failure::<i32, i32>(99);
        let c = success::<i32, i32>(1);

        let y = a.get_or_else(2);
        println!("XXXXXXX {:?}", y);

        let q = a.map(|p| p + 1000);
        println!("XXXXXXX {:?}", q.get_or_else(0));

        let r = apply2(a, c, add);
        let z = r.get_or_else(99999);
        println!("XXXXXXXZ {:?}", z);



        assert!(true);
    }

}

//
//    // Runs a function f in the success of the Validation a or passing the failure
//    // of a through.
//    pub fn apply1<R, F: FnOnce(A) -> R>(a: Validation<E, A>, f: F) -> Validation<E, R> {
//        match a {
//            Validation::Failure(x) => Validation::Failure(x),
//            Validation::Success(x) => Validation::Success(f(x)),
//        }
//    }
//
//
//    // Runs a function f in the success of the Validations a and b or passing any
//    // failures through, accumulating errors along the way. This means that evaluation
//    // is not short-circuited, but that all failures are collected using the supplied
//    // semigroup error type.
//    pub fn apply2<B, R, F: FnOnce(A, B) -> R>(a: Validation<E, A>, b: Validation<E, B>, f: F) -> Validation<E, R> {
//        match b {
//            Validation::Failure(x) => Validation::Failure(Validation::fm2(a, x)),
//            Validation::Success(x) => {
//                let p = |u: A| { f(u, x) };
//                Validation::apply1(a, p)
//            }
//        }
//    }
//
//    // Runs a function f in the success of the Validations a,b, and c or passing any
//    // failures through, accumulating errors along the way. This means that evaluation
//    // is not short-circuited, but that all failures are collected using the supplied
//    // semigroup error type.
//    fn apply3<B, C, R, F: FnOnce(A, B, C) -> R>(a: Validation<E, A>, b: Validation<E, B>, c: Validation<E, C>, f: F) -> Validation<E, R> {
//        match c {
//            Validation::Failure(x) => Validation::Failure(Validation::fm2(b, Validation::fm2(a, x))),
//            Validation::Success(x) => {
//                let p = |u: A, v: B| { f(u, v, x) };
//                Validation::apply2(a, b, p)
//            }
//        }
//    }
//}
//
