#[test]
fn test_chain() {
    use input_core::*;
    use input_more::*;

    type TimestampMs = u64;

    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    enum RawSwitch {
        Key(&'static str),
        //Button(&'static str),
    }

    let keyboard_processor_1 = (
        |(state, sw): (State, RawSwitch)| (state.0, sw, (state.1, state.2, sw)),
        (
            ModifiersPressProcessor,
            |(state, result, args): (
                Modifiers<RawSwitch>,
                Result<(), ModifiersPressError>,
                (
                    TimedState<RawSwitch>,
                    LongPressSchedulerState<TimestampMs>,
                    RawSwitch,
                ),
            )| {
                result.unwrap();
                ((state, args.0, args.1), args.2)
            },
        ),
    );

    let processor = keyboard_processor_1;

    type State = (
        Modifiers<RawSwitch>,
        TimedState<RawSwitch>,
        LongPressSchedulerState<TimestampMs>,
    );

    let state = State::default();

    let sw = RawSwitch::Key("LeftCtrl");
    processor.exec((state, sw));
}
