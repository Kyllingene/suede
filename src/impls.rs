use crate::{Append, Collector, Provide, Walker};

impl<T, C> Walker<C> for [T]
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

impl<T, C> Append<C> for [T]
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

impl<'a, W, C> Walker<C> for &'a W
where
    W: Walker<C>,
    C: Collector,
{
    fn walk(&self, collector: &mut C) -> Result<(), C::Error> {
        (*self).walk(collector)
    }
}

impl<'a, W, C> Walker<C> for &'a mut W
where
    W: Walker<C>,
    C: Collector,
{
    fn walk(&self, collector: &mut C) -> Result<(), C::Error> {
        (**self).walk(collector)
    }
}

impl<'a, W, C> Append<C> for &'a W
where
    W: Append<C>,
    C: Collector,
{
    fn append(&self, collector: &mut C, meta: &C::Meta) -> Result<(), C::Error> {
        (*self).append(collector, meta)
    }
}

impl<'a, W, C> Append<C> for &'a mut W
where
    W: Append<C>,
    C: Collector,
{
    fn append(&self, collector: &mut C, meta: &C::Meta) -> Result<(), C::Error> {
        (**self).append(collector, meta)
    }
}

#[cfg(feature = "std")]
mod with_std {
    use super::*;
    use std::rc::Rc;
    use std::sync::Arc;

    impl<T, C> Walker<C> for Vec<T>
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

    impl<T, C> Append<C> for Vec<T>
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

    impl<W, C> Walker<C> for Box<W>
    where
        W: Walker<C>,
        C: Collector,
    {
        fn walk(&self, collector: &mut C) -> Result<(), C::Error> {
            (**self).walk(collector)
        }
    }

    impl<W, C> Append<C> for Box<W>
    where
        W: Append<C>,
        C: Collector,
    {
        fn append(&self, collector: &mut C, meta: &C::Meta) -> Result<(), C::Error> {
            (**self).append(collector, meta)
        }
    }

    impl<W, C> Walker<C> for Rc<W>
    where
        W: Walker<C>,
        C: Collector,
    {
        fn walk(&self, collector: &mut C) -> Result<(), C::Error> {
            (**self).walk(collector)
        }
    }

    impl<W, C> Append<C> for Rc<W>
    where
        W: Append<C>,
        C: Collector,
    {
        fn append(&self, collector: &mut C, meta: &C::Meta) -> Result<(), C::Error> {
            (**self).append(collector, meta)
        }
    }

    impl<W, C> Walker<C> for Arc<W>
    where
        W: Walker<C>,
        C: Collector,
    {
        fn walk(&self, collector: &mut C) -> Result<(), C::Error> {
            (**self).walk(collector)
        }
    }

    impl<W, C> Append<C> for Arc<W>
    where
        W: Append<C>,
        C: Collector,
    {
        fn append(&self, collector: &mut C, meta: &C::Meta) -> Result<(), C::Error> {
            (**self).append(collector, meta)
        }
    }
}
