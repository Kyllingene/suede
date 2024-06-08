use crate::{Append, Collector, Provide, Walker};

// impls for Vec and [T] not provided yet, but are trivial
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
