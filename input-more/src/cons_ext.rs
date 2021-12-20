pub trait ConsWithLast<T> {
    type Output;

    fn with_last(self, value: T) -> Self::Output;
}

impl<T> ConsWithLast<T> for () {
    type Output = (T, ());

    fn with_last(self, value: T) -> Self::Output {
        (value, ())
    }
}

impl<T, T1, T2> ConsWithLast<T> for (T1, T2)
where
    T2: ConsWithLast<T>,
{
    type Output = (T1, T2::Output);

    fn with_last(self, value: T) -> Self::Output {
        (self.0, self.1.with_last(value))
    }
}

#[test]
fn test() {
    let value = (1, (2, ()));
    let value = value.with_last(3);
    let value = value.with_last(4);
    let value = value.with_last(5);
    assert_eq!(value, (1, (2, (3, (4, (5, ()))))));
}

/*pub trait SplitTuple {
    type First;
    type Rest;

    fn split_first_rest(self) -> (Self::First, Self::Rest);
}

pub trait JoinTuple<Lhs, Rhs> {
    type Output;

    fn from(self) -> (Self::First, Self::Rest);
}

#[macro_export]
macro_rules! impl_array_ext {
    () => {};
    ( $first_idx:tt: $first_ty:ident $(, $rest_idx:tt: $rest_ty:ident)* $(,)? ) => {
        impl<$first_ty $(, $rest_ty)*> SplitTuple for ( $first_ty, $( $rest_ty, )* ) {
            type First = $first_ty;
            type Rest = ( $($rest_ty,)* );

            fn split_first_rest(self) -> (Self::First, Self::Rest) {
                ( self.$first_idx, ( $( self.$rest_idx, )* ))
            }
        }
    };
}

impl_array_ext!(0: T0);
impl_array_ext!(0: T0, 1: T1);
impl_array_ext!(0: T0, 1: T1, 2: T2);
impl_array_ext!(0: T0, 1: T1, 2: T2, 3: T3);
impl_array_ext!(0: T0, 1: T1, 2: T2, 3: T3, 4: T4);
*/
/*
impl<T1> TupleExt for (T1,) {
    type First = T1;
    type Rest = ();

    fn split_first_rest(self) -> (Self::First, Self::Rest) {
        (self.0, ())
    }
}

impl<T1, T2> TupleExt for (T1, T2) {
    type First = T1;
    type Rest = (T2,);

    fn split_first_rest(self) -> (Self::First, Self::Rest) {
        (self.0, (self.1,))
    }
}
*/
