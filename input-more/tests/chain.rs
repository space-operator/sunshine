#[test]
fn test_chain() {
    use core::fmt::Debug;

    use input_core::*;
    use input_more::*;

    type TimestampMs = u64;
    type Coords = (u64, u64);

    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    enum Switch {
        Keyboard(KeyboardSwitch),
        Mouse(MouseSwitch),
    }

    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    struct KeyboardSwitch(&'static str);

    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    struct MouseSwitch(&'static str);

    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    enum RawEvent {
        KeyboardDown(SwitchEvent<KeyboardSwitch, TimestampMs, (), ()>),
        KeyboardUp(SwitchEvent<KeyboardSwitch, TimestampMs, (), ()>),
        MouseDown(SwitchEvent<MouseSwitch, TimestampMs, Coords, ()>),
        MouseUp(SwitchEvent<MouseSwitch, TimestampMs, Coords, ()>),
        //MouseMove(Coords, TimestampMs),
    }

    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    struct SwitchEvent<Sw, Ti, Co, Da> {
        switch: Sw,
        time: Ti,
        coords: Co,
        data: Da,
    }

    /*impl TakeSwitch for SwitchEvent<KeyboardSwitch, Ti, Co, Da> {

    }*/

    impl<Sw, Ti, Co> SwitchEvent<Sw, Ti, Co, ()> {
        fn new(switch: Sw, time: Ti, coords: Co) -> Self {
            SwitchEvent {
                switch: switch,
                time: time,
                coords: coords,
                data: (),
            }
        }

        fn with_data<Da>(self, data: Da) -> SwitchEvent<Sw, Ti, Co, Da> {
            SwitchEvent {
                switch: self.switch,
                time: self.time,
                coords: self.coords,
                data,
            }
        }
    }

    impl<Sw, Ti, Co, Da> SwitchEvent<Sw, Ti, Co, Da> {
        fn take_data(self) -> (Da, SwitchEvent<Sw, Ti, Co, ()>) {
            (
                self.data,
                SwitchEvent {
                    switch: self.switch,
                    time: self.time,
                    coords: self.coords,
                    data: (),
                },
            )
        }
    }

    let state: State<
        Modifiers<Switch>,
        TimedState<Switch>,
        SchedulerState<TimestampMs, Switch, LongPressHandleRequest>,
    > = State::new(Modifiers::new(), TimedState::new(), SchedulerState::new());
    let event = SwitchEvent::new(KeyboardSwitch("LeftCtrl"), 1000, ());
    let context = Context::new(state, event);

    context
        .with_modifiers_press_event()
        .map(|(state, (result, event))| {
            result.unwrap();
            let modifiers = state.modifiers.clone();
            (state, event.with_data(modifiers))
        });

    panic!();
}
