/*
use suede::{Collector, Transformer, Walker};

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

#[derive(Debug)]
enum Token {
    Int(i32),
    Str(String),
    Bool(bool),
    Arr(Vec<Token>),
}

#[derive(Default)]
struct OptFoo {
    w: Option<i32>,
    x: Option<String>,
    y: Option<Vec<bool>>,
    z: Option<Bar>,
}

impl Collector<(String, Token)> for OptFoo {
    type Output = Foo;
    type Error = String;

    fn collect(&mut self, (ident, tok): (String, Token)) -> Result<(), Self::Error> {
        match (ident.as_ref(), tok) {
            ("w", Token::Int(w)) => self.w = Some(w),
            ("w", tok) => return Err(format!("expected int for w, got {tok:?}")),
            ("x", Token::Str(x)) => self.x = Some(x),
            ("x", tok) => return Err(format!("expected str for x, got {tok:?}")),
            ("y", Token::Arr(toks)) => {
                let mut y = Vec::new();
                for tok in toks {
                    let Token::Bool(b) = tok else {
                        return Err(format!("expected bool, get {tok:?}"));
                    };
                    y.push(b);
                }
                self.y = Some(y);
            }
            ("y", tok) => return Err(format!("expected arr for y, got {tok:?}")),
            _ => return Err(format!("unexpected field {ident}")),
        }

        Ok(())
    }

    fn nest(&mut self, other: Self) -> Result<(), Self::Error> {
        panic!()
    }

    fn finish(self) -> Result<Self::Output, Self::Error> {
        let Self {
            w: Some(w),
            x: Some(x),
            y: Some(y),
            z: Some(z),
        } = self
        else {
            return Err("not all fields filled".into());
        };

        Ok(Foo { w, x, y, z })
    }
}

#[derive(Default)]
struct OptBar {
    x: Option<i32>,
    y: Option<String>,
}

impl Collector<(String, Token)> for OptBar {
    type Output = Bar;
    type Error = String;

    fn collect(&mut self, (ident, tok): (String, Token)) -> Result<(), Self::Error> {
        match (ident.as_ref(), tok) {
            ("x", Token::Int(x)) => self.x = Some(x),
            ("x", tok) => return Err(format!("expected int for x, got {tok:?}")),
            ("y", Token::Str(y)) => self.y = Some(y),
            ("y", tok) => return Err(format!("expected str for y, got {tok:?}")),
            _ => return Err(format!("unexpected field {ident}")),
        }

        Ok(())
    }

    fn nest(&mut self, other: Self) -> Result<(), Self::Error> {
        panic!()
    }

    fn finish(self) -> Result<Self::Output, Self::Error> {
        let Self {
            x: Some(x),
            y: Some(y),
        } = self
        else {
            return Err("not all fields filled".into());
        };

        Ok(Bar { x, y })
    }
}

fn parse_token(tok: &str) -> Option<Token> {
    if let Ok(i) = tok.parse() {
        Some(Token::Int(i))
    } else if tok == "true" {
        Some(Token::Bool(true))
    } else if tok == "false" {
        Some(Token::Bool(false))
    } else if let Some(rest) = tok.strip_prefix('[') {
        let mid = rest.strip_suffix(']')?;
        let bits = mid.split(',');
        let mut toks = Vec::new();

        for bit in bits {
            if bit.is_empty() {
                break;
            }
            toks.push(parse_token(bit.trim())?);
        }

        Some(Token::Arr(toks))
    } else {
        None
    }
}

fn field_transformer((ident, tok): (String, String)) -> Option<(String, Token)> {
    Some((ident, parse_token(&tok)?))
}

enum ObjError<E> {
    Parse(String),
    Sub(E),
}

struct ObjWalker(String);

impl Walker<(String, String)> for ObjWalker {
    type Error<E> = ObjError<E>;

    fn walk<T, C>(self, transformer: &mut T, collector: &mut C) -> Result<(), Self::Error<C::Error>>
    where
        T: Transformer<(String, String)>,
        C: Collector<T::Output>,
    {
        let Some(rest) = self.0.strip_prefix('{') else {
            return Err(ObjError::Parse(format!("expected {{, got {:?}", self.0.chars().next())));
        };

        let Some(mut mid) = rest.strip_suffix('}') else {
            return Err(ObjError::Parse(format!("expected }}, got {:?}", rest.chars().last())));
        };
        mid = mid.trim();

        while !mid.is_empty() {
            let Some((ident, rest)) = mid.split_once(':') else {
                return Err(ObjError::Parse(format!("expected :, got {mid}")));
            };

            let (tok, rest) = mid.split_once(',').unwrap_or((rest, ""));
            let data = transformer.transform((ident.into(), tok.into()));
            collector.collect(data).map_err(ObjError::Sub)?;

            mid = rest.trim();
        }

        Ok(())
    }
}

*/
fn main() {}
