pub trait Processor<Input>: Sized {
    type Output;
    fn exec(input: Input) -> Self::Output;
}

impl<T, U, Input> Processor<Input> for (T, U)
where
    T: Processor<Input>,
    U: Processor<T::Output>,
{
    type Output = U::Output;
    fn exec(input: Input) -> Self::Output {
        U::exec(T::exec(input))
    }
}

impl<Input> Processor<Input> for () {
    type Output = Input;
    fn exec(input: Input) -> Self::Output {
        input
    }
}

#[test]
fn test() {
    #[derive(Debug)]
    pub struct ModifiersProcessor;

    #[derive(Debug)]
    pub struct Tuplicator;

    #[derive(Clone, Debug)]
    pub struct ModifiersState;

    impl Processor<(u32, ModifiersState)> for ModifiersProcessor {
        type Output = (u64, ModifiersState);
        fn exec(input: (u32, ModifiersState)) -> Self::Output {
            (input.0 as u64 + 123, input.1)
        }
    }

    impl<T: Clone> Processor<T> for Tuplicator {
        type Output = (T, T);
        fn exec(input: T) -> Self::Output {
            (input.clone(), input)
        }
    }

    type Chain = ((ModifiersProcessor, Tuplicator), (Tuplicator, ((), ())));

    println!("{:?}", Chain::exec((12, ModifiersState)));

    // (((AdderWithInc(11), Tuplicator), (Tuplicator, ((), ()))), ((22, 22), (22, 22)))
    panic!();
}

/*
pub trait Processor<Input>: Sized {
    type Output;
    fn exec(self, input: Input) -> (Self, Self::Output);
}

impl<T, U, Input> Processor<Input> for (T, U)
where
    T: Processor<Input>,
    U: Processor<T::Output>,
{
    type Output = U::Output;
    fn exec(self, input: Input) -> (Self, Self::Output) {
        let (first, immediate) = self.0.exec(input);
        let (second, output) = self.1.exec(immediate);
        ((first, second), output)
    }
}

impl<Input> Processor<Input> for () {
    type Output = Input;
    fn exec(self, input: Input) -> (Self, Self::Output) {
        (self, input)
    }
}

#[test]
fn test() {
    #[derive(Debug)]
    pub struct AdderWithInc(u64);

    #[derive(Debug)]
    pub struct Tuplicator;

    impl Processor<u32> for AdderWithInc {
        type Output = u64;
        fn exec(self, input: u32) -> (Self, Self::Output) {
            let input = input as u64;
            (Self(self.0 + 1), input + self.0)
        }
    }

    impl<T: Clone> Processor<T> for Tuplicator {
        type Output = (T, T);
        fn exec(self, input: T) -> (Self, Self::Output) {
            (self, (input.clone(), input))
        }
    }

    println!(
        "{:?}",
        ((AdderWithInc(10), Tuplicator), (Tuplicator, ((), ()))).exec(12)
    );

    // (((AdderWithInc(11), Tuplicator), (Tuplicator, ((), ()))), ((22, 22), (22, 22)))
    panic!();
}
*/
/*
(MouseDown::Lbm(20, 10), &mapping)
((MouseDown::Lbm(20, 10), [Key::Ctrl + Key::Shift]), &mapping)
((MouseDown::Lbm(20, 10), [Key::Ctrl + Key::Shift]), FilteredMapping1(&, &, &))
((MouseDown::Lbm(20, 10), [Key::Ctrl + Key::Shift], Click), FilteredMapping1(&, &, &))
((MouseDown::Lbm(20, 10), [Key::Ctrl + Key::Shift], Click), FilteredMapping2(&, &, &))
(
    (MouseDown::Lbm(20, 10), [Key::Ctrl + Key::Shift], Click, NotDragAndDrop),
    FilteredMapping2(&, &, &)
)
(
    (MouseDown::Lbm(20, 10), [Key::Ctrl + Key::Shift], Click, NotDragAndDrop),
    [(sws : binding1), (sws : binding2), (sws : binding3)]
)
(
    (MouseDown::Lbm(20, 10), [Key::Ctrl + Key::Shift], Click, NotDragAndDrop),
    [(sws : binding1), (sws : binding2)] // longest
)
(
    (MouseDown::Lbm(20, 10), [Key::Ctrl + Key::Shift], Click, NotDragAndDrop),
    [(sws : binding1), (sws : binding2)] // same modifiers
)
(
    [event for binding1 with (20, 10), event for binding2 with (20, 10)]
)

A
(A, (...))

(A, (B, (C, (D, ()))))
*/
