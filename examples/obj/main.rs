use suede::Collector;

mod coll;
mod walk;

use coll::*;

struct Foo {
    w: i64,
    x: [bool; 3],
    y: Bar,
    z: [Baz; 2],
}

struct Bar {
    x: i64,
    y: String,
}

enum Baz {
    A { x: i64 },
    B {
        y: i64,
        z: [bool; 3],
    },
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
        z: [
            Baz::A { x: 123 },
            Baz::B {
                y: 456,
                z: [false, true, false],
            },
        ],
    };

    println!("{}", Dict::serialize(&x).unwrap());
}
