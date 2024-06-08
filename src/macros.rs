/// Implements [`Append`] for each item, for the given collector.
#[macro_export]
macro_rules! atom {
    ($(
        $(#[$($gen:tt)*])?
        $collector:ty : $atom:ty $(, $rest:ty)*
    );* $(;)?) => {$(
        impl$(<$($gen)*>)? $crate::Append<$collector> for $atom {
            fn append(
                &self,
                collector: &mut $collector,
                meta: &<$collector as $crate::Collector>::Meta,
            ) -> Result<(), <$collector as $crate::Collector>::Error> {
                collector.collect(self, meta)
            }
        }

        $crate::atom![$(#[$($gen)*])? $collector : $($rest),*];
    )*};

    ($(
        $(#[$($gen:tt)*])?
        $collector:ty :
    );* $(;)?) => {};
}
