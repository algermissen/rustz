use std::marker::PhantomData;

pub struct Reader<'reader, R, A> {
    run: Box<Fn(&R) -> A + 'reader>,
    state_type: PhantomData<R>,
    content_type: PhantomData<A>,
}

impl<'reader, R: 'reader, A: 'reader> Reader<'reader, R, A> {
    pub fn new<F>(f: F) -> Reader<'reader, R, A>
        where F: Fn(&R) -> A + 'reader
    {
        Reader {
            run: Box::new(f),
            state_type: PhantomData,
            content_type: PhantomData,
        }
    }

    pub fn run(&self, r: &R) -> A {
        (self.run)(r)
    }

    pub fn map<B: 'reader, G>(self, f: G) -> Reader<'reader, R, B>
        where G: Fn(A) -> B + 'reader
    {
        let h = move |s: &R| {
            let a = (self.run)(s);
            f(a)
        };
        Reader::new(h)
    }

    pub fn flat_map<B: 'reader, G>(self, f: G) -> Reader<'reader, R, B>
        where G: Fn(A) -> Reader<'reader, R, B> + 'reader
    {
        let h = move |s: &R| {
            let a = (self.run)(s);
            (f(a).run)(s)
        };
        Reader::new(h)
    }
}


#[cfg(test)]
mod tests {

    use super::Reader;

    #[derive(Debug)]
    struct Connection {
        x: i32,
    }

    fn get_user(id: i32) -> Reader<'static, Connection, i32> {
        Reader::new(move |c: &Connection| id + c.x)
    }
    fn get_other(id: i32) -> Reader<'static, Connection, i32> {
        Reader::new(move |c: &Connection| id + c.x)
    }

    #[test]
    fn it_works() {

        let conn = Connection { x: 9 };

        //let r = get_user(10).map(|a| a + 20);
        let r = get_user(10).flat_map(|a| get_other(a + 1000));

        let u = (r.run)(&conn);


        println!("##### Result: {:?} ", u);
        assert!(true);
    }
}
