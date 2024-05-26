#[cfg(test)]
mod test;

pub trait Walker<Fmt> {
    fn walk<T, C, S>(self, transformer: T, collector: &mut C) -> Result<(), C::Error>
    where
        T: Transformer<Fmt, S>,
        C: Collector<T::Output, State = S>;
}

pub trait Transformer<Fmt, State> {
    type Output;

    fn transform(&mut self, data: Fmt, state: State) -> Self::Output;
}

pub trait Collector<Fmt>: Default {
    type Output;
    type State;
    type Error;

    fn state(&self) -> Self::State;
    fn collect(&mut self, data: Fmt) -> Result<(), Self::Error>;
    fn nest(&mut self, other: Self) -> Result<(), Self::Error>;
    fn finish(self) -> Result<Self::Output, Self::Error>;
}

impl<F: FnMut(Fmt, State) -> O, Fmt, State, O> Transformer<Fmt, State> for F {
    type Output = O;

    fn transform(&mut self, data: Fmt, state: State) -> Self::Output {
        self(data, state)
    }
}

impl<Fmt, State> Transformer<Fmt, State> for () {
    type Output = Fmt;

    fn transform(&mut self, data: Fmt, _state: State) -> Self::Output {
        data
    }
}
