mod impls;

/// An object that collects bits of incongruous data, producing the final result
/// of the ser/de chain.
///
/// Think of this as sort of a bucket that items are thrown into.
///
/// See [`Collect`] and [`Provide`] for related traits.
pub trait Collector {
    /// The final output type.
    type Output;

    /// The single error type this collector is ever allowed to return.
    type Error;

    /// The metadata that must be passed with each bit of data.
    type Meta: ?Sized;

    /// Attempt to finalize the collector, returning the final result.
    fn finish(self) -> Result<Self::Output, Self::Error>;

    /// Create this collector, and use it once to serialize a given object.
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

/// A [`Collector`] that can collect a given type.
pub trait Collect<T: ?Sized>: Collector {
    fn collect(&mut self, data: &T, meta: &Self::Meta) -> Result<(), Self::Error>;
}

/// A [`Collector`] that can nest other collectors.
pub trait Provide<M: ?Sized>: Collector {
    type Adapter: Collector<Error = Self::Error, Meta = M>;

    fn provide(&self) -> Self::Adapter;
    fn restore(&mut self, adapter: Self::Adapter, meta: &Self::Meta) -> Result<(), Self::Error>;
}

/// An object that can be walked over, submitting its items to a [`Collector`].
///
/// When calling [`walk`](Walker::walk), the item should fill the collector,
/// acting as the "root" object, the only object the collector will directly
/// accept.
pub trait Walker<C: Collector> {
    fn walk(&self, collector: &mut C) -> Result<(), C::Error>;
}

/// An object that can be submitted to a collector like any other.
pub trait Append<C: Collector> {
    fn append(&self, collector: &mut C, meta: &C::Meta) -> Result<(), C::Error>;
}

// TODO: provide impls for `&Walker`, `Box<Append>`, etc.

/// A temporary trait while things are worked out.
pub trait Atom {}

pub use never::Never;
mod never {
    #[doc(hidden)]
    pub trait Extract { type R; }
    impl<R> Extract for fn() -> R { type R = R; }

    /// The `!` type, extracted for your convenience.
    pub type Never = <fn() -> ! as Extract>::R;
}
