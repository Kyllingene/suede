mod fields {
    include!("common/fields.rs");
}

mod obj {
    include!("common/obj-fmt.rs");
}

struct Foo {
    w: i32,
    x: String,
    y: Vec<bool>,
    z: Bar,
}

struct Bar {
    x: i32,
    y: String,
}

fn main() {
    /*
    let val = Foo {
        w: 2,
        x: "Hello, World!".into(),
        y: vec![true, false, false],
        z: Bar {
            x: 2,
            y: "Goodbye, World!".into(),
        },
    };

    let mut coll = ObjCollector(String::new());
    let mut trans = field_transformer;

    val.walk(&mut trans, &mut coll).unwrap();
    println!("{}", coll.finish().unwrap());
    */
}
