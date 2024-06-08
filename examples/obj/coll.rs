use suede::*;

use std::fmt::Write;

collector![
    pub Dict = [
        "{ ",
        " }",
        str,
        |c, m| {
            c.data.push_str(m);
            c.data.push_str(": ");
        }
    ];

    pub List = [
        "[",
        "]",
        (),
        |_, _| {}
    ];
];

macro_rules! collector {
    ($(
        $v:vis $name:ident = [
            $start:expr,
            $end:expr,
            $meta:ty,
            $do_meta:expr
        ];
    )*) => {$(
        $v struct $name {
            data: String,
            first: bool,
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl $name {
            pub fn new() -> Self {
                Self { data: $start.to_string(), first: true }
            }
        }

        impl Collector for $name {
            type Output = String;
            type Error = Never;
            type Meta = $meta;

            fn finish(mut self) -> Result<Self::Output, Self::Error> {
                self.data.push_str($end);
                Ok(self.data)
            }
        }

        impl Provide<str> for $name {
            type Adapter = Dict;

            fn provide(&self) -> Self::Adapter { Dict::new() }
            fn restore(&mut self, adapter: Self::Adapter, meta: &Self::Meta) -> Result<(), Self::Error> {
                let s = adapter.finish()?;
                if !self.first {
                    self.data.push_str(", ");
                }
                self.first = false;

                let f: fn(&mut Self, &Self::Meta) = $do_meta;
                f(self, meta);
                self.data.push_str(&s);

                Ok(())
            }
        }

        impl Provide<()> for $name {
            type Adapter = List;

            fn provide(&self) -> Self::Adapter { List::new() }
            fn restore(&mut self, adapter: Self::Adapter, meta: &Self::Meta) -> Result<(), Self::Error> {
                let s = adapter.finish()?;
                if !self.first {
                    self.data.push_str(", ");
                }
                self.first = false;

                let f: fn(&mut Self, &Self::Meta) = $do_meta;
                f(self, meta);
                self.data.push_str(&s);

                Ok(())
            }
        }

        impl Collect<i64> for $name {
            fn collect(&mut self, data: &i64, meta: &Self::Meta) -> Result<(), Self::Error> {
                if !self.first {
                    self.data.push_str(", ");
                }
                self.first = false;

                let f: fn(&mut Self, &Self::Meta) = $do_meta;
                f(self, meta);
                write!(self.data, "{data}").unwrap();

                Ok(())
            }
        }

        impl Collect<bool> for $name {
            fn collect(&mut self, data: &bool, meta: &Self::Meta) -> Result<(), Self::Error> {
                if !self.first {
                    self.data.push_str(", ");
                }
                self.first = false;

                let f: fn(&mut Self, &Self::Meta) = $do_meta;
                f(self, meta);
                write!(self.data, "{data}").unwrap();

                Ok(())
            }
        }

        impl Collect<String> for $name {
            fn collect(&mut self, data: &String, meta: &Self::Meta) -> Result<(), Self::Error> {
                if !self.first {
                    self.data.push_str(", ");
                }
                self.first = false;

                let f: fn(&mut Self, &Self::Meta) = $do_meta;
                f(self, meta);
                write!(self.data, "{data:?}").unwrap();

                Ok(())
            }
        }
    )*};
}
use collector;
