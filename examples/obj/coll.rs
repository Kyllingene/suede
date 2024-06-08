use suede::prelude::*;

use std::fmt::Write;

#[derive(Debug, Clone, Copy)]
pub struct Full;

suede::atom![
    Dict: i64, bool, String;
    List: i64, bool, String;
    Enum: i64, bool, String;
];

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

pub struct Enum {
    data: String,
}

impl Default for Enum {
    fn default() -> Self {
        Self::new()
    }
}

impl Enum {
    pub fn new() -> Self {
        Self { data: String::new() }
    }
}

impl Collector for Enum {
    type Output = String;
    type Error = Full;
    type Meta = EnumTag;

    fn finish(self) -> Result<Self::Output, Self::Error> {
        Ok(self.data)
    }
}

impl Provide<str> for Enum {
    type Adapter = Dict;

    fn provide(&self) -> Self::Adapter { Dict::new() }
    fn restore(&mut self, adapter: Self::Adapter, meta: &Self::Meta) -> Result<(), Self::Error> {
        if !self.data.is_empty() {
            return Err(Full);
        }

        let s = adapter.finish()?;
        self.data.push_str(meta);
        self.data.push(' ');
        self.data.push_str(&s);

        Ok(())
    }
}

impl Provide<()> for Enum {
    type Adapter = List;

    fn provide(&self) -> Self::Adapter { List::new() }
    fn restore(&mut self, adapter: Self::Adapter, meta: &Self::Meta) -> Result<(), Self::Error> {
        if !self.data.is_empty() {
            return Err(Full);
        }

        let s = adapter.finish()?;

        self.data.push_str(meta);
        self.data.push(' ');
        self.data.push_str(&s);

        Ok(())
    }
}

impl Collect<i64> for Enum {
    fn collect(&mut self, data: &i64, meta: &Self::Meta) -> Result<(), Self::Error> {
        if !self.data.is_empty() {
            return Err(Full);
        }

        self.data.push_str(meta);
        self.data.push(' ');
        write!(self.data, "{data}").unwrap();

        Ok(())
    }
}

impl Collect<bool> for Enum {
    fn collect(&mut self, data: &bool, meta: &Self::Meta) -> Result<(), Self::Error> {
        if !self.data.is_empty() {
            return Err(Full);
        }

        self.data.push_str(meta);
        self.data.push(' ');
        write!(self.data, "{data}").unwrap();

        Ok(())
    }
}

impl Collect<String> for Enum {
    fn collect(&mut self, data: &String, meta: &Self::Meta) -> Result<(), Self::Error> {
        if !self.data.is_empty() {
            return Err(Full);
        }

        self.data.push_str(meta);
        self.data.push(' ');
        write!(self.data, "{data:?}").unwrap();

        Ok(())
    }
}

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
            type Error = Full;
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

        impl Provide<EnumTag> for $name {
            type Adapter = Enum;

            fn provide(&self) -> Self::Adapter { Enum::new() }
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
