use suede::*;

use std::fmt::Debug;

struct Foo {
    w: i32,
    x: String,
}

struct Printer;

impl Collector for Printer {
    type Meta = str;
    type Error = Never;
}

impl<T: Debug> Collect<T> for Printer {
    fn collect(&mut self, data: &T, meta: &str) -> Result<(), Never> {
        println!("\t{meta}: {data}:?");
    }
}

fn main() {}
