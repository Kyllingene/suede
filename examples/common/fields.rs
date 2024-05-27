use core::marker::PhantomData;

pub struct Identity;

impl Identity {
    pub fn pass<T>(t: T) -> T { t }
}

pub struct ObjAdapter<C>(PhantomData<fn() -> C>);

impl<C> ObjAdapter<C> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

macro_rules! fields_walker {
    ( @error $error:ident $e:ident) => { $error<$e> };
    ( @error $e:ident ) => { $e };

    ( @wrap_err $error:ident $other:ident $e:expr ) => { $e.map_err($error::$other)? };
    ( @wrap_err $e:expr ) => { $e? };

    (
        token = $token:path;
        $( ?error = $error:ident / $other:ident; )?
        $v:vis struct $name:ident {
            $( $fv:vis $field:ident : $typ:ty = $tok:ident ? $err:ident ),* $(,)?
        }
    ) => {
        $v struct $name {
            $( $fv $field : $typ ),*
        }

        impl ::suede::Walker<(&'static str, $token)> for $name
        {
            type Adapter<C> = ObjCollector<C>;
            type Error<E> = $crate::fields_walker(@error $( $error )? E);

            fn walk<T, C>(
                self,
                transformer: &mut T,
                adapter: &mut Self::Adapter<C>,
                collector: &mut C,
            ) -> Result<(), Self::Error<C::Error>>
            where
                T: ::suede::Transformer<$token>,
                C: ::suede::Collector<T::Output>,
                Self::Adapter<C>: ::suede::Adapter<C>,
            {
                $(
                    $crate::fields_walker(@wrap_err 
                        $( $error $other )?
                        self.$field.walk($token::$tok, transformer, adapter, collector)
                    );
                )*

                collector.finish()
            }
        }
    };
}
