use std::marker::PhantomData;

pub struct State<'state, S, A> {
    run: Box<Fn(S) -> (S, A) + 'state>,
    state_type: PhantomData<S>,
    content_type: PhantomData<A>,
}

impl<'state, S: 'state + Clone + Copy, A: 'state> State<'state, S, A> {
    pub fn new<F>(f: F) -> State<'state, S, A>
    where
        F: Fn(S) -> (S, A) + 'state,
    {
        State {
            run: Box::new(f),
            state_type: PhantomData,
            content_type: PhantomData,
        }
    }

    pub fn run(&self, s: S) -> (S, A) {
        (self.run)(s)
    }

    pub fn map<B: 'state, G>(self, f: G) -> State<'state, S, B>
    where
        G: Fn(A) -> B + 'state,
    {
        let h = move |s: S| {
            let (s1, a) = (self.run)(s);
            (s1, f(a))
        };
        State::new(h)
    }

    pub fn flat_map<B: 'state, G>(self, f: G) -> State<'state, S, B>
    where
        G: Fn(A) -> State<'state, S, B> + 'state,
    {
        let h = move |s: S| {
            let (s1, a) = (self.run)(s);
            (f(a).run)(s1)
        };
        State::new(h)
    }

    pub fn get(self) -> State<'state, S, S> {
        let f = move |s: S| {
            let s2 = s.clone();
            (s, s2)
        };
        State::new(f)
    }

    pub fn gets<F>(self, f: F) -> State<'state, S, A>
    where
        F: Fn(S) -> A + 'state,
    {
        let g = move |s: S| {
            let s2 = s.clone();
            (s, f(s2))
        };
        State::new(g)
    }

    pub fn put(self, s: S) -> State<'state, S, ()> {
        let s2 = s.clone();
        let f = move |_| (s2, ());
        State::new(f)
    }

    pub fn modify<F>(self, f: F) -> State<'state, S, ()>
    where
        F: Fn(S) -> S + 'state,
    {
        let g = move |s| (f(s), ());
        State::new(g)
    }
}


#[cfg(test)]
mod tests {

    //use super::state::State;
    use super::State;

    #[derive(Debug, Clone, Copy)]
    struct Account {
        balance: i32,
    }

    fn deduct(d: i32) -> State<'static, Account, i32> {
        State::new(move |a: Account| (Account { balance: a.balance - d }, 0))
    }
    fn contribute(d: i32) -> State<'static, Account, i32> {
        State::new(move |a: Account| (Account { balance: a.balance + d }, 0))
    }
    // https://youtu.be/9uRXjxy7JDE?t=10m39s
    //

    #[test]
    fn it_works() {

        let account = Account { balance: 0 };

        let x = contribute(10).flat_map(move |_: i32| deduct(5));

        //let q = |x: i32| State::new(move |a: Account| (Account { balance: 111 + x }, 500));

        //let s = State::new(|a: Account| (Account { balance: 1 }, 5))
        //    .flat_map(q)
        //    .map(|a4| 100);
        let (account2, fee) = (x.run)(account);

        println!("____ Balance: {:?} Fee {:?}", account2.balance, fee);

        assert!(true);
    }

}
