use crate::{Provide, Append, Atom, Walker, Collect, Collector};

impl<T, C, const N: usize> Walker<C> for [T; N]
where
    T: Append<C>,
    C: Collector<Meta = ()>,
{
    fn walk(&self, collector: &mut C) -> Result<(), C::Error> {
        for x in self {
            x.append(collector, &())?;
        }
        Ok(())
    }
}

impl<T, C, const N: usize> Append<C> for [T; N]
where
    T: Append<C::Adapter>,
    C: Provide<()>,
{
    fn append(&self, collector: &mut C, meta: &C::Meta) -> Result<(), C::Error> {
        let mut list = collector.provide();
        self.walk(&mut list)?;
        collector.restore(list, meta)
    }
}

impl Atom for i64 {}
impl Atom for bool {}
impl Atom for str {}
impl Atom for String {}
impl<'a, T: Atom> Atom for &'a T {}

impl<W, C> Append<C> for W
where
    W: Atom + ?Sized,
    C: Collect<W>,
{
    fn append(&self, collector: &mut C, meta: &C::Meta) -> Result<(), C::Error> {
        collector.collect(self, meta)
    }
}
