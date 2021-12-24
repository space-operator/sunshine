//#[cfg(feature = "qwdsadfrgsfg")]
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
        KeyboardDown(KeyboardSwitchEvent),
        KeyboardUp(KeyboardSwitchEvent),
        MouseDown(MouseSwitchEvent),
        MouseUp(MouseSwitchEvent),
        //MouseMove(Coords, TimestampMs),
    }

    pub type KeyboardSwitchEvent = SwitchEvent<TimestampMs, KeyboardSwitch, (), ()>;
    pub type MouseSwitchEvent = SwitchEvent<TimestampMs, MouseSwitch, Coords, ()>;

    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    struct SwitchEvent<Ti, Sw, Co, Mo> {
        time: Ti,
        switch: Sw,
        coords: Co,
        modifiers: Mo,
    }
    impl<Ti, Sw, Co> SwitchEvent<Ti, Sw, Co, ()> {
        fn new(time: Ti, switch: Sw, coords: Co) -> Self {
            SwitchEvent {
                time: time,
                switch: switch,
                coords: coords,
                modifiers: (),
            }
        }

        fn with_modifiers<Mo>(self, modifiers: Mo) -> SwitchEvent<Ti, Sw, Co, Mo> {
            SwitchEvent {
                time: self.time,
                switch: self.switch,
                coords: self.coords,
                modifiers,
            }
        }
    }

    /*
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
    }*/

    let modifiers: Modifiers<Switch> = Modifiers::new();
    let timed_state: TimedState<KeyboardSwitch> = TimedState::new();
    let scheduler: SchedulerState<TimestampMs, KeyboardSwitchEvent, LongPressHandleRequest> =
        SchedulerState::new();

    let event = SwitchEvent::new(1000, KeyboardSwitch("LeftCtrl"), ());

    let (modifiers, result) = modifiers.with_press_event(Switch::Keyboard(event.switch));
    result.unwrap();
    event.with_modifiers(modifiers.clone());
    let (timed_state, result) = timed_state.with_press_event(event.switch);
    let request = result.unwrap();
    let scheduler = scheduler.schedule(event.time, event, request);

    println!("{:?}", modifiers);
    println!("{:?}", timed_state);
    println!("{:?}", scheduler);
    println!("{:?}", event);
    panic!();

    /*let state: State<
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

    panic!();*/
}
