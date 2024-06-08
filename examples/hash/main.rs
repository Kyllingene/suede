use suede::*;

mod coll;
mod walk;

use coll::Hasher;

struct Foo {
    w: i64,
    x: [bool; 3],
    y: Bar,
}

struct Bar {
    x: i64,
    y: &'static str,
}

fn main() {
    let x = Foo {
        w: 123,
        x: [true, false, true],
        y: Bar {
            x: 456,
            y: "Hello, World!",
        },
    };

    println!("{:x}", Hasher::serialize(&x.y).unwrap());
}
