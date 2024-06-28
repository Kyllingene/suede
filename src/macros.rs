#[macro_export]
macro_rules! atom {
    ($(
        $collector:path : $( $item:path ),+ $(,)?
    );* $(;)?) => {
        $(
            $(
                impl $crate::Submit<$collector> for $item {
                    fn submit(
                        &self,
                        collector: &mut $collector,
                        meta: &<$collector as $crate::Collector>::Meta
                    ) -> Result<(), <$collector as $crate::Collector>::Error> {
                        $crate::Collect::collect(collector, self, meta)
                    }
                }
            )+
        )*
    };
}
