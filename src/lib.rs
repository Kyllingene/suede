#![cfg_attr(not(any(test, feature = "std")), no_std)]

mod impls;
mod macros;

pub mod prelude {
    pub use crate::{Append, Collect, Collector, Never, Provide, Walker, EnumTag};
}

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
    /// Collect a single item, with relevant metadata.
    fn collect(&mut self, data: &T, meta: &Self::Meta) -> Result<(), Self::Error>;
}

/// A [`Collector`] that can nest other collectors.
pub trait Provide<M: ?Sized>: Collector {
    /// The collector to be returned.
    ///
    /// For simplicity's sake, it must share an error type with its parent.
    type Adapter: Collector<Error = Self::Error, Meta = M>;

    /// Create and return an adapter.
    fn provide(&self) -> Self::Adapter;

    /// Absorb an adapter as if it were a "normal" item.
    ///
    /// Essentially `collect` for `Self::Adapter`.
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

/// An object that can be submitted to a collector.
pub trait Append<C: Collector> {
    fn append(&self, collector: &mut C, meta: &C::Meta) -> Result<(), C::Error>;
}

/// The string name of an enum variant.
///
/// See [`new`](EnumTag::new).
#[repr(transparent)]
pub struct EnumTag(pub str);

impl EnumTag {
    /// Create a new tag from a string.
    pub fn new(tag: &str) -> &Self {
        // SAFETY: repr(transparent)
        unsafe { std::mem::transmute(tag) }
    }
}

impl core::ops::Deref for EnumTag {
    type Target = str;

    fn deref(&self) -> &str { &self.0 }
}

pub use never::Never;
mod never {
    #[doc(hidden)]
    pub trait Extract {
        type R;
    }
    impl<R> Extract for fn() -> R {
        type R = R;
    }

    /// The `!` type, extracted for your convenience.
    pub type Never = <fn() -> ! as Extract>::R;
}
