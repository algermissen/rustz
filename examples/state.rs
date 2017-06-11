extern crate rustz;

use rustz::State;

#[derive(Debug,Clone,Copy)]
struct Account {
    balance: i32,
}


fn deduct(d: i32) -> State<Account, i32> {
    State::new(move |a: Account| (Account { balance: a.balance - d }, 1))
}
fn contribute(d: i32) -> State<Account, i32> {
    State::new(move |a: Account| (Account { balance: a.balance + d }, 1))
}

fn main() {
    let add10ThenSubtract5WithLastFee = contribute(10).flat_map(move |i: i32| deduct(5));
    let account = Account { balance: 0 };
    let (account2, fee) = add10ThenSubtract5WithLastFee.run(account);
    println!("New balance: {:?} , last transaction fee {:?}",
             account2.balance,
             fee);

    //let q = |x: i32| State::new(move |a: Account| (Account { balance: 111 + x }, 500));
    //let s = State::new(|a: Account| (Account { balance: 1 }, 5))
    //    .flatMap(q)
    //    .map(|a4| 100);
}
