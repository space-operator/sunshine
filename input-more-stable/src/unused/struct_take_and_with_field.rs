#[macro_export]
macro_rules! define_struct_take_and_with_field {
    ( $( $tt:tt )* ) => {
        $crate::define_struct_take_field!( $($tt) *);
        $crate::define_struct_with_field!( $($tt) *);
    };
}
