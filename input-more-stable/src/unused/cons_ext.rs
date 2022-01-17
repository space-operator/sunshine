#[macro_export]
macro_rules! define_struct_from_into_cons_and_take_put {
    ( $name:ident, $( $names:ident: $args:ident + $markers:ident ),* $(,)? ) => {
        $crate::define_struct_from_into_cons_and_take_put!(
            @impl build
            ( $name )
            ( $( $names: $args + $markers ),* )
            ( $crate::define_struct_from_into_cons_and_take_put!(
                @impl cons idents $( $args ),* )
            )
        );
    };

    ( @impl build
        ( $name:ident )
        ( $( $names:ident: $args:ident + $markers:ident ),* )
        ( $args_cons:ty )
    ) => {
        impl< $( $args ),* > ::core::convert::From<$name<$( $args ),*>> for $args_cons {
            fn from(value: $name<$( $args ),*>) -> Self {
                $crate::define_struct_from_into_cons_and_take_put!(
                    @impl cons fields value $( $names ),*
                )
            }
        }

        impl< $( $args ),* > ::core::convert::From<$args_cons> for $name< $( $args ),* > {
            fn from( $crate::define_struct_from_into_cons_and_take_put!(
                @impl cons idents $( $names ),*
            ): $args_cons) -> Self {
                Self::new($( $names ),*)
            }
        }

        impl< $( $args ),* > $crate::ConsView for $name< $( $args ),* > {
            type View = $args_cons;
        }

        $crate::define_struct_from_into_cons_and_take_put!(
            @impl value
            ( $name )
            ( $( $args ),* )
            ( $( $args: $markers ),* )
            ( () )
        );
    };

    ( @impl cons idents ) => { () };
    ( @impl cons idents $first:ident $(, $value:ident)* ) => {
        ( $first, $crate::define_struct_from_into_cons_and_take_put!(
            @impl cons idents $( $value ),* )
        )
    };

    ( @impl cons fields $source:ident ) => { () };
    ( @impl cons fields $source:ident $first:ident $(, $value:ident)* ) => {
        ( $source.$first, $crate::define_struct_from_into_cons_and_take_put!(
            @impl cons fields $source $( $value ),* )
        )
    };

    ( @impl value $_1:tt $_2:tt () $_3:tt ) => {};
    ( @impl value
        ( $name:ident )
        ( $( $params:ident ),* )
        ( $arg:ident: $marker:ident $(, $args:ident: $markers:ident )* )
        ( $scheme:ty )
    ) => {
        impl< $( $params ),* > $crate::ConsValue<$arg, $marker> for $name<$( $params ),*> {
            type Scheme = $scheme;
        }

        $crate::define_struct_from_into_cons_and_take_put!(
            @impl value
            ( $name )
            ( $( $params ),* )
            ( $( $args: $markers ),* )
            ( ( $scheme, ) )
        );
    };
}

pub trait Take<Va, Ma> {
    type Rest;

    fn take(self) -> (Va, Self::Rest);
}

pub trait Put<Va, Ma, Da> {
    fn put(self, value: Va) -> Da;
}

impl<Da, Vi, Re, Va, Ma> Take<Va, Ma> for Da
where
    Da: ConsView<View = Vi> + ConsValue<Va, Ma>,
    Vi: From<Da> + ConsTake<Va, Da::Scheme, Rest = Re>,
{
    type Rest = Vi::Rest;

    fn take(self) -> (Va, Self::Rest) {
        Vi::from(self).take()
    }
}

impl<Da, Vi, Re, Va, Ma> Put<Va, Ma, Da> for Re
where
    Da: ConsView<View = Vi> + ConsValue<Va, Ma> + From<Vi>,
    Re: ConsPut<Va, Da::Scheme, Output = Vi>,
{
    fn put(self, value: Va) -> Da {
        self.put(value).into()
    }
}

pub trait ConsView {
    type View;
}

pub trait ConsValue<Va, Ma> {
    type Scheme;
}

pub trait ConsTake<Va, Sc> {
    type Rest;

    fn take(self) -> (Va, Self::Rest);
}

pub trait ConsPut<Va, Sc> {
    type Output;

    fn put(self, value: Va) -> Self::Output;
}

impl<T1, T2> ConsTake<T1, ()> for (T1, T2) {
    type Rest = ((), T2);

    fn take(self) -> (T1, Self::Rest) {
        (self.0, ((), self.1))
    }
}

impl<T1, T2, T3, T4> ConsTake<T1, (T2,)> for (T3, T4)
where
    T4: ConsTake<T1, T2>,
{
    type Rest = (T3, T4::Rest);

    fn take(self) -> (T1, (T3, T4::Rest)) {
        let (value, rest) = self.1.take();
        (value, (self.0, rest))
    }
}

impl<T1, T2> ConsPut<T1, ()> for ((), T2) {
    type Output = (T1, T2);

    fn put(self, value: T1) -> Self::Output {
        (value, self.1)
    }
}

impl<T1, T2, T3, T4> ConsPut<T1, (T2,)> for (T3, T4)
where
    T4: ConsPut<T1, T2>,
{
    type Output = (T3, T4::Output);

    fn put(self, value: T1) -> Self::Output {
        (self.0, self.1.put(value))
    }
}
