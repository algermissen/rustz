extern crate rustz;

use rustz::Reader;

#[derive(Debug,Clone)]
struct Connection {
    x: i32,
}

fn get_user(id: i32) -> Reader<Connection, i32> {
    Reader::new(move |c: &Connection| id + c.x)
}
fn get_other(id: i32) -> Reader<Connection, i32> {
    Reader::new(move |c: &Connection| id + c.x)
}


fn main() {
    let conn = Connection { x: 9 };

    let r = get_user(10).flat_map(|a| get_other(a + 1000));

    let u = r.run(&conn);

    println!("Result: {:?} ", u);
}
