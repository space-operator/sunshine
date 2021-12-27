macro_rules! impl_with {
    ( $name:ident $ti:tt $sw:tt $rq:tt $co:tt $dr:tt $mo:tt $td:tt $po:tt $da:tt) => {
        pub trait $name<
            impl_with!( @impl arg Ti $ti )
            impl_with!( @impl arg Sw $ti )
            Rq
        > {}
    };
    ( @impl arg $arg:ident 0 ) => {
        $arg,
    };
    ( @impl arg $arg:ident 1 ) => {}
}

// Ti, Sw, Rq, Co, Dr, Mo, Td, Po, Da

impl_with!(with_time   1 0 0 0 0 0 0 0 0);
impl_with!(with_switch 0 1 0 0 0 0 0 0 0);
