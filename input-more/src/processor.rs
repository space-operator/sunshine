pub trait Processor<Input> {
    type Output;
    fn exec(&self, input: Input) -> Self::Output;
}

impl<T, U, Input> Processor<Input> for (T, U)
where
    T: Processor<Input>,
    U: Processor<T::Output>,
{
    type Output = U::Output;
    fn exec(&self, input: Input) -> Self::Output {
        self.1.exec(self.0.exec(input))
    }
}

impl<Input> Processor<Input> for () {
    type Output = Input;
    fn exec(&self, input: Input) -> Self::Output {
        input
    }
}

impl<F, Input, Output> Processor<Input> for F
where
    F: Fn(Input) -> Output,
{
    type Output = Output;
    fn exec(&self, input: Input) -> Self::Output {
        self(input)
    }
}

#[test]
fn test() {
    #[derive(Debug, Default)]
    pub struct ModifiersProcessor;

    #[derive(Clone, Debug, Default)]
    pub struct ModifiersState(u32);

    #[derive(Clone, Debug, Default)]
    pub struct Switch(u32);

    #[derive(Debug, Default)]
    pub struct TimedProcessor;

    #[derive(Clone, Debug, Default)]
    pub struct TimedState(u32);

    impl<T> Processor<(Switch, ModifiersState, T)> for ModifiersProcessor {
        type Output = (Switch, ModifiersState, T);
        fn exec(&self, input: (Switch, ModifiersState, T)) -> Self::Output {
            (
                Switch(input.0 .0),
                ModifiersState(input.1 .0 + input.0 .0),
                input.2,
            )
        }
    }

    impl<T> Processor<(Switch, TimedState, T)> for TimedProcessor {
        type Output = (u32, TimedState, T);
        fn exec(&self, input: (Switch, TimedState, T)) -> Self::Output {
            (32, TimedState(input.1 .0 + input.0 .0), input.2)
        }
    }

    #[derive(Debug, Default)]
    pub struct FilterByModifiers;

    impl Processor<(Switch, ModifiersState, (TimedState, (u32, u32)))> for FilterByModifiers {
        type Output = (Switch, TimedState, (ModifiersState, (u32, u32)));
        fn exec(&self, input: (Switch, ModifiersState, (TimedState, (u32, u32)))) -> Self::Output {
            (input.0, input.2 .0, (input.1, input.2 .1))
        }
    }

    type Chain = ((ModifiersProcessor, FilterByModifiers), TimedProcessor);

    println!(
        "{:?}",
        Chain::default().exec((
            Switch(10),
            ModifiersState(100),
            (TimedState(1000), (123, 234))
        ))
    );

    // let (state, events) = KeyboardChain::exec((ev, state, modifiers))
    // let (state, events) = MouseChain::exec((ev, state, modifiers))

    // (((AdderWithInc(11), Tuplicator), (Tuplicator, ((), ()))), ((22, 22), (22, 22)))
    panic!();
}

/*
    A: how much is done
    B: how good is it

    A       B
     16%     70%
     30%     70%
     45%     70%
     62%     40%
     23%     80%
     41%     80%
     61%     80%
     10%     90%
     45%     90%
     71%     90%
     12%    132%
     25%    132%
      5%   1235%

*/

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
