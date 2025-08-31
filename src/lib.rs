pub use paste;
pub use seq_macro;

#[macro_export]
macro_rules! define_context {
    ($ctx: ident {
        $($field: ident : $type: ty),* $(,)?
    }) => {
        pub struct $ctx {
            $(pub $field: $type),*
        }

        $crate::paste::paste! {
            pub trait [< From $ctx >] {
                fn from_context(ctx: &$ctx) -> Self;
            }

            $(
                impl [< From $ctx >] for $type {
                    fn from_context(ctx: &$ctx) -> Self {
                        ctx.$field.clone()
                    }
                }
            )*
        }
    };
}

#[macro_export]
macro_rules! context_as_params {
    (impl $ctx: ident, $max: expr) => {
        $crate::seq_macro::seq!(N in 1..=$max {
            $crate::paste::paste! {
                impl<F, #(T~N,)*> [< $ctx Handler >]<(#(T~N,)*)> for F
                where
                    F: Fn(#(T~N,)*),
                    #(T~N: [< From $ctx >],)*
                {
                    fn call(self, ctx: &$ctx) {
                        (self)(
                            #(T~N::from_context(&ctx),)*
                        );
                    }
                }
            }
        });

    };

    ($ctx: ident, $max: expr) => {
        $crate::paste::paste! {
            trait [< $ctx Handler >]<T> {
                fn call(self, ctx: &$ctx);
            }
        }
        $crate::seq_macro::seq!(N in 1..=$max {
            context_as_params!(impl $ctx, N);
        });
    };

    ($ctx: ident) => {
        context_as_params!($ctx, 6);
    };
}
