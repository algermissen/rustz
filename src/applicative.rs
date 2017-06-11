//trait Applicative {
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
