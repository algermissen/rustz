//use std::result;
//use std::collections::LinkedList;
//
//trait Semigroup {
//    fn mappend(&self, b: Self) -> Self;
//}
//
//impl<T: Clone> Semigroup for LinkedList<T> {
//    fn mappend(&self, b: LinkedList<T>) -> LinkedList<T> {
//        let mut n = self.clone();
//        for e in b.iter() {
//            n.push_back(e.clone());
//        }
//        n
//    }
//}
//
//
//pub enum Validation<E, A> {
//    Success(A),
//    Failure(E)
//}
//
//pub type ValidationNel<E, A> = Validation<LinkedList<E>, A>;
//
//pub fn failure_nel<E, A: Clone>(e: E) -> ValidationNel<E, A> {
//    let mut li: LinkedList<E> = LinkedList::new();
//    li.push_back(e);
//    Validation::Failure(li)
//}
//
//pub fn success_nel<E: Clone, A>(item: A) -> ValidationNel<E, A> {
//    Validation::Success(item)
//}
//
//impl<E: Semigroup, A> Validation<E, A> {
//    fn fm2(a: Validation<E, A>, e: E) -> E {
//        match a {
//            Validation::Failure(x) => x.mappend(e),
//            Validation::Success(_) => e,
//        }
//    }
//
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
