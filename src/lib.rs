#![cfg_attr(not(test), no_std)]

mod macros;

pub trait Collector {
    type Error;
    type Meta: ?Sized;
}

pub trait Collect<T: ?Sized>: Collector {
    fn collect(&mut self, data: &T, meta: &Self::Meta) -> Result<(), Self::Error>;
}

pub trait Adapt<M: ?Sized>: Collector {
    type Adapter: Collector<Error = Self::Error, Meta = M>;

    fn adapt(&mut self) -> Self::Adapter;
    fn retract(&mut self, adapter: Self::Adapter, meta: &Self::Meta) -> Result<(), Self::Error>;
}

pub trait Submit<C: Collector> {
    fn submit(&self, collector: &mut C, meta: &C::Meta) -> Result<(), C::Error>;
}

pub trait Walk<C: Collector> {
    fn walk(&self, collector: &mut C) -> Result<(), C::Error>;
}

mod never {
    pub trait Extract {
        type R;
    }
    impl<T> Extract for fn() -> T {
        type R = T;
    }
    pub type Never = <fn() -> ! as Extract>::R;
}
pub use never::Never;
