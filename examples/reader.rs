extern crate rustz;

use rustz::Reader;
use std::vec::Vec;


struct User {
    id: i32,
    some_val: i32,
}

struct Project {
    id: i32,
}

struct KV {
    state: Box<i32>,
}

impl KV {
    fn get(&self, space: &str, key: i32) -> i32 {
        key * 2
    }
}


struct GraphDB {
    state: Box<i32>,
}
impl GraphDB {
    fn find_all(&self, space: &str, key: i32) -> Vec<i32> {
        vec![1, 2, 3]
    }
}

struct Env {
    kv: KV,
    gdb: GraphDB,
}

fn get_kv<'a>() -> Reader<'a, Env, KV> {
    Reader::new(|env: &Env| env.kv)
}

fn get_user<'a>(user_id: i32) -> Reader<'a, Env, User> {
    get_kv().map(move |kv: KV| {
                     let v = kv.get("users", user_id);
                     User {
                         id: user_id,
                         some_val: v,
                     }
                 })
}


fn main() {
    let kv = KV { state: Box::new(1) };
    let gdb = GraphDB { state: Box::new(1) };
    let env = Env { kv: kv, gdb: gdb };

    //let r = get_user(10).flat_map(|a| get_other(a + 2000));

    //let u = r.run(&env);

    //println!("Result: {:?} ", u);
}
