mod impls;

pub trait Collector {
    type Output;
    type Error;
    type Meta: ?Sized;

    fn finish(self) -> Result<Self::Output, Self::Error>;

    fn serialize<W>(walker: &W) -> Result<Self::Output, Self::Error>
    where
        Self: Default,
        W: Walker<Self>,
    {
        let mut this = Self::default();
        walker.walk(&mut this)?;
        this.finish()
    }
}

pub trait Collect<T: ?Sized>: Collector {
    fn collect(&mut self, data: &T, meta: &Self::Meta) -> Result<(), Self::Error>;
}

pub trait Provide<M: ?Sized>: Collector {
    type Adapter: Collector<Error = Self::Error, Meta = M>;

    fn provide(&self) -> Self::Adapter;
    fn restore(&mut self, adapter: Self::Adapter, meta: &Self::Meta) -> Result<(), Self::Error>;
}

pub trait Walker<C: Collector> {
    fn walk(&self, collector: &mut C) -> Result<(), C::Error>;
}

pub trait Append<C: Collector> {
    fn append(&self, collector: &mut C, meta: &C::Meta) -> Result<(), C::Error>;
}

pub trait Atom {}

pub use never::Never;
mod never {
    #[doc(hidden)]
    pub trait Extract { type R; }
    impl<R> Extract for fn() -> R { type R = R; }

    /// The `!` type, extracted for your convenience.
    pub type Never = <fn() -> ! as Extract>::R;
}
