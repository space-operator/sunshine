pub trait StructTakeField<T, U> {
    type Rest;

    fn take_field(self) -> (T, Self::Rest);
}

#[macro_export]
macro_rules! define_struct_take_field {
    ( $name:ident { $( $fields:ident: $types:ident + $markers:ident ),* $(,)? } ) => {
        $crate::define_struct_take_field!(
            @impl next
            $name
            { $( $types ),* }
            {}
            { $( $fields: $types + $markers ),* }
        );
    };
    (
        @impl next
        $name:ident
        { $( $types:ident ),* }
        { $( $prev_fields:ident: $prev_types:ident + $prev_markers:ident ),* }
        {
            $field:ident: $type:ident + $marker:ident
            $(, $next_fields:ident: $next_types:ident + $next_markers:ident )*
        }
    ) => {
        $crate::define_struct_take_field!(
            @impl field
            $name
            { $( $types ),* }
            { $( $prev_fields: $prev_types + $prev_markers ),* }
            { $field: $type + $marker }
            { $( $next_fields: $next_types + $next_markers ),* }
        );
    };
    (
        @impl next
        $name:ident
        { $( $types:ident ),* }
        { $( $prev_fields:ident: $prev_types:ident + $prev_markers:ident ),* }
        {}
    ) => {};
    (
        @impl field
        $name:ident
        { $( $types:ident ),* }
        { $( $prev_fields:ident: $prev_types:ident + $prev_markers:ident ),* }
        { $field:ident: $type:ident + $marker:ident }
        { $( $next_fields:ident: $next_types:ident + $next_markers:ident ),* }
    ) => {
        impl< $( $types ),* > $crate::StructTakeField<$type, $marker> for $name< $( $types ),* >
        {
            type Rest = $name< $( $prev_types, )* ::core::marker::PhantomData<$type> $(, $next_types)* >;

            fn take_field(self) -> ($type, Self::Rest) {
                (
                    self.$field,
                    $name {
                        $( $prev_fields: self.$prev_fields, )*
                        $field: ::core::marker::PhantomData,
                        $( $next_fields: self.$next_fields, )*
                    },
                )
            }
        }

        $crate::define_struct_take_field!(
            @impl next
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
}
