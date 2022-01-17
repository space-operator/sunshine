//pub trait InputFrom<T> {}

/*
pub trait InputFrom<T> {
    fn from(_: T) -> Self;
}

pub trait InputInto<T> {
    fn into2(self) -> T; // TODO: Deal with it
}

impl<T, U> InputInto<U> for T
where
    U: InputFrom<T>,
{
    fn into2(self) -> U {
        U::from(self)
    }
}

pub trait A {
    fn v(self) -> i32;
}

pub trait B {
    fn v(self) -> i32;
}

impl A for i32 {
    fn v(self) -> i32 {
        6
    }
}

impl B for f64 {
    fn v(self) -> i32 {
        12
    }
}

#[test]
fn test() {
    println!("{}", 12_i32.v());
    println!("{}", (12.0_f64).v());
    panic!();
}
*/
