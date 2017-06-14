extern crate rustz;

use rustz::Reader;
use std::vec::Vec;


#[derive(Debug)]
struct User {
    id: i32,
    some_val: i32,
}

#[derive(Debug,Clone)]
struct KV {
    //state: Box<i32>,
    state: i32,
}

impl KV {
    fn get(&self, space: &str, key: i32) -> i32 {
        key * 2
    }
}


#[derive(Debug,Clone)]
struct GraphDB {
    //state: Box<i32>,
    state: i32,
}
impl GraphDB {
    fn find_all(&self, space: &str, key: i32) -> Vec<i32> {
        vec![1, 2]
    }
}

struct Env {
    kv: KV,
    gdb: GraphDB,
}

fn get_kv<'a>() -> Reader<'a, Env, KV> {
    Reader::new(|env: &Env| env.kv.clone())
}
fn get_gdb<'a>() -> Reader<'a, Env, GraphDB> {
    Reader::new(|env: &Env| env.gdb.clone())
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

fn get_friends<'a>(user_id: i32) -> Reader<'a, Env, Vec<User>> {
    get_gdb().map(move |gdb: GraphDB| {
                      let _ = gdb.find_all("friends", user_id);
                      let u1 = User { id: 1, some_val: 2 };
                      let u2 = User { id: 2, some_val: 4 };
                      vec![u1, u2]
                  })
}

fn get_friends_of_user<'a>(user_id: i32) -> Reader<'a, Env, Vec<User>> {
    get_user(user_id).flat_map(|user| get_friends(user.id))
}


fn main() {
    //let kv = KV { state: Box::new(1) };
    //let gdb = GraphDB { state: Box::new(1) };
    //
    let kv = KV { state: 1 };
    let gdb = GraphDB { state: 1 };
    let env = Env { kv: kv, gdb: gdb };

    let user_reader = get_user(10);
    //let friends_reader = get_friends(10).flat_map(|user| get_friends(user.id));
    let friends_reader = get_friends_of_user(10);

    let user = user_reader.run(&env);
    let friends = friends_reader.run(&env);

    println!("User: {:?} ", user);
    println!("Friends: {:?} ", friends);
}
