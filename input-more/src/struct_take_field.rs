pub trait StructTakeField<T, U> {
    type Rest;

    fn take_field(self) -> (T, Self::Rest);
}

#[macro_export]
macro_rules! define_struct_take_field {
    ( $name:ident {} ) => {};
    ( $name:ident { $($fields:ident: $types:ident + $markers:ident ),* $(,)? } ) => {
        $crate::define_struct_take_field!(
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
        $crate::define_struct_take_field!(
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
        impl< $( $types ),* > $crate::StructTakeField<$type, $marker> for $name< $( $types ),* >
        {
            type Rest = $name< $( $prev_types, )* () $(, $next_types)* >;

            fn take_field(self) -> ($type, Self::Rest) {
                (
                    self.$field,
                    $name {
                        $( $prev_fields: self.$prev_fields, )*
                        $field: (),
                        $( $next_fields: self.$next_fields, )*
                    },
                )
            }
        }

        $crate::define_struct_take_field!(
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
