extern crate rustz;

use std::marker::PhantomData;

struct State<S, A> {
    run: Box<Fn(S) -> (S, A)>,
    state_type: PhantomData<S>,
    content_type: PhantomData<A>,
}

impl<S: 'static + Clone + Copy, A: 'static> State<S, A> {
    fn new<F>(f: F) -> State<S, A>
        where F: Fn(S) -> (S, A) + 'static
    {
        State {
            run: Box::new(f),
            state_type: PhantomData,
            content_type: PhantomData,
        }
    }

    fn map<B: 'static, G>(self, f: G) -> State<S, B>
        where G: Fn(A) -> B + 'static
    {
        let h = move |s: S| {
            let (s1, a) = (self.run)(s);
            (s1, f(a))
        };
        State::new(h)
    }

    fn flatMap<B: 'static, G>(self, f: G) -> State<S, B>
        where G: Fn(A) -> State<S, B> + 'static
    {
        let h = move |s: S| {
            let (s1, a) = (self.run)(s);
            (f(a).run)(s1)
        };
        State::new(h)
    }

    fn get(self) -> State<S, S> {
        let f = move |s: S| {
            let s2 = s.clone();
            (s, s2)
        };
        State::new(f)
    }

    fn gets<F>(self, f: F) -> State<S, A>
        where F: Fn(S) -> A + 'static
    {
        let g = move |s: S| {
            let s2 = s.clone();
            (s, f(s2))
        };
        State::new(g)
    }

    fn put(self, s: S) -> State<S, ()> {
        let s2 = s.clone();
        let f = move |_| (s2, ());
        State::new(f)
    }

    fn modify<F>(self, f: F) -> State<S, ()>
        where F: Fn(S) -> S + 'static
    {
        let g = move |s| (f(s), ());
        State::new(g)
    }
}

#[derive(Debug,Clone,Copy)]
struct Account {
    balance: i32,
}



fn deduct(d: i32) -> State<Account, i32> {
    State::new(move |a: Account| (Account { balance: a.balance - d }, 0))
}
fn contribute(d: i32) -> State<Account, i32> {
    State::new(move |a: Account| (Account { balance: a.balance + d }, 0))
}
// https://youtu.be/9uRXjxy7JDE?t=10m39s


fn main() {
    let account = Account { balance: 0 };

    let x = contribute(10).flatMap(move |i: i32| deduct(5));

    //let q = |x: i32| State::new(move |a: Account| (Account { balance: 111 + x }, 500));

    //let s = State::new(|a: Account| (Account { balance: 1 }, 5))
    //    .flatMap(q)
    //    .map(|a4| 100);
    let (account2, fee) = (x.run)(account);

    println!("____ Balance: {:?} Fee {:?}", account2.balance, fee);

}
