pub trait Walker<Unit> {
    type Adapter<C>;
    type Error<E>;

    fn is_flat(&self) -> bool;
    fn walk<T, C>(
        self,
        transformer: &mut T,
        adapter: &mut Self::Adapter<C>,
        collector: &mut C,
    ) -> Result<(), Self::Error<C::Error>>
    where
        T: Transformer<Unit>,
        C: Collector<T::Output>,
        Self::Adapter<C>: Adapter<C>;
}

pub trait Transformer<Unit> {
    type Output;

    fn transform(&mut self, data: Unit) -> Self::Output;
}

pub trait Adapter<C> {}

pub trait Collector<Unit> {
    type Output;
    type Error;

    fn collect(&mut self, data: Unit) -> Result<(), Self::Error>;
    fn finish(self) -> Result<Self::Output, Self::Error>;
}

impl<F: FnMut(Unit) -> Output, Unit, Output> Transformer<Unit> for F {
    type Output = Output;

    fn transform(&mut self, data: Unit) -> Self::Output {
        self(data)
    }
}

#[macro_export]
macro_rules! wrap_unit {
    ( $typ:path, $unit:path, $wrapper:expr ) => {
        impl Walker<$unit> for $typ {
            type Adapter<C> = ();
            type Error<E> = E;

            fn is_flat(&self) -> bool { true }
            fn walk<T, C>(
                self,
                transformer: &mut T,
                adapter: &mut Self::Adapter<C>,
                collector: &mut C,
            ) -> Result<(), Self::Error<C::Error>>
            where
                T: Transformer<$unit>,
                C: Collector<T::Output>,
                Self::Adapter<C>: Adapter<C>
            {
                let data = transformer.transform($wrapper(self));
                collector.collect(data)
            }
        }
    };
}
