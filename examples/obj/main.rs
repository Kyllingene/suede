use suede::Collector;

mod walk;
mod coll;

use coll::*;

struct Foo {
    w: i64,
    x: [bool; 3],
    y: Bar,
}

struct Bar {
    x: i64,
    y: String,
}

#[allow(unused)]
fn main() {
    let x = Foo {
        w: 123,
        x: [true, false, true],
        y: Bar {
            x: 456,
            y: "Hello, World!".into(),
        },
    };

    println!("{}", Dict::serialize(&x).unwrap());
}
