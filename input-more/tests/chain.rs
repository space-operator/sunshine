//#[cfg(feature = "qwdsadfrgsfg")]
#[test]
fn test_chain() {
    use core::fmt::Debug;

    use input_core::*;
    use input_more::*;

    type TimestampMs = u64;
    type Coords = (u64, u64);

    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct TimeMarker;

    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct SwitchMarker;

    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct RequestMarker;

    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct CoordsMarker;

    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct IsDraggedFnMarker;

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
    struct SwitchEvent<Ti, Sw, Co, Da> {
        time: Ti,
        switch: Sw,
        coords: Co,
        data: Da,
    }

    impl<Ti, Sw, Co> SwitchEvent<Ti, Sw, Co, ()> {
        fn new(time: Ti, switch: Sw, coords: Co) -> Self {
            SwitchEvent {
                time: time,
                switch: switch,
                coords: coords,
                data: (),
            }
        }

        fn with_data<Da>(self, data: Da) -> SwitchEvent<Ti, Sw, Co, Da> {
            SwitchEvent {
                time: self.time,
                switch: self.switch,
                coords: self.coords,
                data,
            }
        }
    }

    impl<Ti, Sw, Co, Da> SwitchEvent<Ti, Sw, Co, Da> {
        fn take_data(self) -> (Da, SwitchEvent<Ti, Sw, Co, ()>) {
            (
                self.data,
                SwitchEvent {
                    time: self.time,
                    switch: self.switch,
                    coords: self.coords,
                    data: (),
                },
            )
        }

        fn map_data<F, Da2>(self, func: F) -> SwitchEvent<Ti, Sw, Co, Da2>
        where
            F: FnOnce(Da) -> Da2,
        {
            SwitchEvent {
                time: self.time,
                switch: self.switch,
                coords: self.coords,
                data: func(self.data),
            }
        }
    }

    impl AllowSplitFromItself<TimeMarker> for TimestampMs {}

    impl<Ti, Sw, Co, Da> Split<Ti, SwitchEvent<Ti, Sw, Co, Da>, TimeMarker>
        for SwitchEvent<Ti, Sw, Co, Da>
    where
        Ti: Clone,
    {
        fn split(self) -> (Ti, SwitchEvent<Ti, Sw, Co, Da>) {
            (self.time.clone(), self)
        }
    }

    impl<Ti, Co, Da> Split<Switch, SwitchEvent<Ti, KeyboardSwitch, Co, Da>, SwitchMarker>
        for SwitchEvent<Ti, KeyboardSwitch, Co, Da>
    {
        fn split(self) -> (Switch, SwitchEvent<Ti, KeyboardSwitch, Co, Da>) {
            (Switch::Keyboard(self.switch.clone()), self)
        }
    }

    impl<Ti, Sw, Co, Da> Split<Sw, SwitchEvent<Ti, Sw, Co, Da>, SwitchMarker>
        for SwitchEvent<Ti, Sw, Co, Da>
    where
        Sw: Clone,
    {
        fn split(self) -> (Sw, SwitchEvent<Ti, Sw, Co, Da>) {
            (self.switch.clone(), self)
        }
    }

    impl<Ti, Sw, Co, Rq> Split<Rq, SwitchEvent<Ti, Sw, Co, ()>, RequestMarker>
        for SwitchEvent<Ti, Sw, Co, Rq>
    {
        fn split(self) -> (Rq, SwitchEvent<Ti, Sw, Co, ()>) {
            (
                self.data,
                SwitchEvent::new(self.time, self.switch, self.coords),
            )
        }
    }

    impl<Ti, Sw, Co, Da, Rq> Split<Rq, SwitchEvent<Ti, Sw, Co, Da>, RequestMarker>
        for SwitchEvent<Ti, Sw, Co, (Rq, Da)>
    {
        fn split(self) -> (Rq, SwitchEvent<Ti, Sw, Co, Da>) {
            (
                self.data.0,
                SwitchEvent::new(self.time, self.switch, self.coords).with_data(self.data.1),
            )
        }
    }

    let state: State<
        Modifiers<Switch>,
        TimedState<KeyboardSwitch>,
        SchedulerState<TimestampMs, KeyboardSwitch, LongPressHandleRequest>,
    > = State::new(Modifiers::new(), TimedState::new(), SchedulerState::new());
    let event = SwitchEvent::new(1000, KeyboardSwitch("LeftCtrl"), ());

    let (state, event) = Context::new(state, event)
        .with_modifiers_press_event()
        .map(|state, (result, event)| {
            result.unwrap();
            let modifiers = state.modifiers.clone();
            (state, event.map_data(|()| modifiers))
        })
        .with_timed_press_event()
        .map_event(|(result, event)| {
            let request = result.unwrap();
            event.map_data(|modifiers| (request, modifiers))
        })
        .with_scheduler()
        .split();

    println!("{:?}", state);
    println!("{:?}", event);
    println!();

    let event = 3000 - 1000;
    let (state, ()) = Context::new(state, event)
        .with_scheduled_taken()
        .map(|mut state, ((_, requests), ())| {
            for (time, requests) in requests {
                for (switch, request) in requests {
                    let event = SwitchEvent::new(time, switch, ()).with_data(request);
                    let (new_state, event) = Context::new(state, event)
                        .with_long_press_event()
                        .map_event(|(data, event)| {
                            event.with_data(data.map(|result| result.unwrap()))
                        })
                        .split();

                    state = new_state;
                    println!("{:?}", state);
                    println!("{:?}", event);
                }
            }
            (state, ())
        })
        .split();

    println!("{:?}", state);

    panic!();
}
