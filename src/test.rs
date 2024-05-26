use super::*;

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

enum Token {
    Int(i32),
    Str(String),
    Bool(bool),
    Arr(Vec<Token>),
}

fn str_tok(tok: Token) -> String {
    match tok {
        Token::Int(i) => format!("{i}"),
        Token::Str(s) => format!("{s:?}"),
        Token::Bool(b) => format!("{b:?}"),
        Token::Arr(a) => {
            let mut s = "[".to_string();
            for tok in a {
                s.push_str(&str_tok(tok));
                s.push(',');
                s.push(' ');
            }
            s.push(']');
            s
        }
    }
}

fn field_transformer((ident, tok): (&'static str, Token), (): ()) -> String {
    format!("{ident}: {}", str_tok(tok))
}

impl Walker<(&'static str, Token)> for Foo {
    fn walk<T, C, S>(self, mut transformer: T, mut collector: &mut C) -> Result<(), C::Error>
    where
        T: Transformer<(&'static str, Token), S>,
        C: Collector<T::Output, State = S>,
    {
        let w = transformer.transform(("w", Token::Int(self.w)), collector.state());
        collector.collect(w)?;

        let x = transformer.transform(("x", Token::Str(self.x)), collector.state());
        collector.collect(x)?;

        let toks = self.y.into_iter().map(Token::Bool).collect();
        let y = transformer.transform(("y", Token::Arr(toks)), collector.state());
        collector.collect(y)?;

        let mut z = C::default();
        self.z.walk(transformer, &mut z)?;
        collector.nest(z)?;

        Ok(())
    }
}

impl Walker<(&'static str, Token)> for Bar {
    fn walk<T, C, S>(self, mut transformer: T, mut collector: &mut C) -> Result<(), C::Error>
    where
        T: Transformer<(&'static str, Token), S>,
        C: Collector<T::Output, State = S>,
    {
        let x = transformer.transform(("x", Token::Int(self.x)), collector.state());
        collector.collect(x)?;

        let y = transformer.transform(("y", Token::Str(self.y)), collector.state());
        collector.collect(y)?;

        Ok(())
    }
}

#[derive(Default)]
struct ObjCollector(String);

impl Collector<String> for ObjCollector {
    type Output = String;
    type State = ();
    type Error = ();

    fn state(&self) -> Self::State {}

    fn collect(&mut self, data: String) -> Result<(), Self::Error> {
        self.0.push_str(&data);
        self.0.push(',');
        self.0.push(' ');
        Ok(())
    }

    fn nest(&mut self, other: Self) -> Result<(), Self::Error> {
        self.0.push_str(&other.finish()?);
        self.0.push(',');
        self.0.push(' ');
        Ok(())
    }

    fn finish(self) -> Result<Self::Output, Self::Error> {
        Ok(format!("{{ {} }}", &self.0[..self.0.len() - 2]))
    }
}

#[test]
fn main() {
    let foo = Foo {
        w: 2,
        x: "Hello, World!".into(),
        y: vec![true, false, false],
        z: Bar {
            x: 2,
            y: "Goodbye, World!".into(),
        },
    };

    let mut coll = ObjCollector(String::new());
    let trans = field_transformer;

    foo.walk(trans, &mut coll).unwrap();
    println!("{}", coll.finish().unwrap());
    panic!();
}
