use suede::*;

use crate::{Bar, Foo};

impl<C> Walker<C> for Foo
where
    C: Collector<Meta = str> + Provide<str>,
    i64: Append<C>,
    [bool; 3]: Append<C>,
    Bar: Append<C>,
{
    fn walk(&self, collector: &mut C) -> Result<(), C::Error> {
        self.w.append(collector, "w")?;
        self.x.append(collector, "x")?;
        self.y.append(collector, "y")?;
        Ok(())
    }
}

impl<C> Append<C> for Foo
where
    C: Provide<str>,
    Self: Walker<C::Adapter>,
{
    fn append(&self, collector: &mut C, meta: &C::Meta) -> Result<(), C::Error> {
        let mut dict = collector.provide();
        self.walk(&mut dict)?;
        collector.restore(dict, meta)
    }
}

impl<C> Walker<C> for Bar
where
    C: Collector<Meta = str> + Provide<str>,
    i64: Append<C>,
    String: Append<C>,
{
    fn walk(&self, collector: &mut C) -> Result<(), C::Error> {
        self.x.append(collector, "x")?;
        self.y.append(collector, "y")?;
        Ok(())
    }
}

impl<C> Append<C> for Bar
where
    C: Provide<str>,
    Self: Walker<C::Adapter>,
{
    fn append(&self, collector: &mut C, meta: &C::Meta) -> Result<(), C::Error> {
        let mut dict = collector.provide();
        self.walk(&mut dict)?;
        collector.restore(dict, meta)
    }
}
