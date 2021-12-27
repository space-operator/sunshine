pub trait StructWithField<T, U> {
    type Output;

    fn with_field(self, value: T) -> Self::Output;
}

#[macro_export]
macro_rules! define_struct_with_field {
    ( $name:ident {} ) => {};
    ( $name:ident { $($fields:ident: $types:ident + $markers:ident ),* $(,)? } ) => {
        $crate::define_struct_with_field!(
            @impl impl_for_next
            $name
            { $( $types ),* }
            {}
            { $( $fields: $types + $markers ),* }
        );
    };
    (
        @impl impl_for_next
        $name:ident
        { $( $types:ident ),* }
        { $( $prev_fields:ident: $prev_types:ident + $prev_markers:ident ),* }
        {
            $field:ident: $type:ident + $marker:ident
            $(, $next_fields:ident: $next_types:ident + $next_markers:ident )*
        }
    ) => {
        $crate::define_struct_with_field!(
            @impl impl_for_field
            $name
            { $( $types ),* }
            { $( $prev_fields: $prev_types + $prev_markers ),* }
            { $field: $type + $marker }
            { $( $next_fields: $next_types + $next_markers ),* }
        );
    };
    (
        @impl impl_for_next
        $name:ident
        { $( $types:ident ),* }
        { $( $prev_fields:ident: $prev_types:ident + $prev_markers:ident ),* }
        {}
    ) => {};
    (
        @impl impl_for_field
        $name:ident
        { $( $types:ident ),* }
        { $( $prev_fields:ident: $prev_types:ident + $prev_markers:ident ),* }
        { $field:ident: $type:ident + $marker:ident }
        { $( $next_fields:ident: $next_types:ident + $next_markers:ident ),* }
    ) => {
        impl< $( $types ),* > $crate::StructWithField<$type, $marker> for
            $name< $( $prev_types, )* () $(, $next_types)* >
        {
            type Output = $name< $( $types ),* >;

            fn with_field(self, value: $type) -> Self::Output {
                $name {
                    $( $prev_fields: self.$prev_fields, )*
                    $field: value,
                    $( $next_fields: self.$next_fields, )*
                }
            }
        }

        $crate::define_struct_with_field!(
            @impl impl_for_next
            $name
            { $( $types ),* }
            {
                $( $prev_fields: $prev_types + $prev_markers, )*
                $field: $type + $marker
            }
            {
                $( $next_fields: $next_types + $next_markers ),*
            }
        );
    };
    { $($tail:tt)* } => { compile_error!(stringify!($($tail)*)); };
}
