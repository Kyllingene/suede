use suede::{Transformer, Collector, Adapter, Walker};

use std::fmt;

pub enum Token {
    Int(i32),
    Str(String),
    Bool(bool),
    Arr(Vec<Token>),
    Obj(Vec<(String, Token)>),
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Int(i) => write!(f, "{i}"),
            Self::Str(s) => write!(f, "{s:?}"),
            Self::Bool(b) => write!(f, "{b}"),
            Self::Arr(a) => write!(f, "{a:?}"),
            Self::Obj(a) => todo!(),
        }
    }
}

suede::wrap_unit!(i32, Token, Token::Int);
suede::wrap_unit!(String, Token, Token::Str);
suede::wrap_unit!(bool, Token, Token::Bool);
suede::wrap_unit!(Vec<Token>, Token, Token::Arr);
suede::wrap_unit!(Vec<(String, Token)>, Token, Token::Obj);

pub struct Serializer(Vec<(String, Token)>);

impl Collector<(String, Token)> for Serializer {
    type Output = String;
    type Error = core::convert::Infallible;

    fn collect(&mut self, data: (String, Token)) -> Result<(), Self::Error> {
        self.0.push(data);
        Ok(())
    }

    fn finish(mut self) -> Result<Self::Output, Self::Error> {
        let mut s = "{".to_string();

        let last = self.0.pop();
        for (ident, token) in self.0 {
            s.push_str(ident);
        }

        Ok(s)
    }
}

pub enum ObjError<E> {
    Parse(String),
    Other(E),
}

pub struct StrWalker(String);

impl Walker<(String, Token)> for StrWalker {
    type Adapter<C> = ();
    type Error<E> = ObjError<E>;

    fn is_flat(&self) -> bool { false }
    fn walk<T, C>(
        self,
        transformer: &mut T,
        adapter: &mut Self::Adapter<C>,
        collector: &mut C,
    ) -> Result<(), Self::Error<C::Error>>
    where
        T: Transformer<(String, Token)>,
        C: Collector<T::Output>,
        Self::Adapter<C>: Adapter<C>
    {
        todo!() // TODO: ugh parsing
    }
}
