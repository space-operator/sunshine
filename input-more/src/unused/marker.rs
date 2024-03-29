#[macro_export]
macro_rules! define_markers {
    ( $( $name:ident ),* $(,)? ) => {
        $(
            #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
            pub struct $name;
        )*
    };
}
