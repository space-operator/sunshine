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

    impl<Ti, Sw, Co, Da> TakeTime<Ti> for SwitchEvent<Ti, Sw, Co, Da>
    where
        Ti: Clone,
    {
        type Rest = Self;

        fn take_time(self) -> (Ti, Self::Rest) {
            (self.time.clone(), self)
        }
    }

    /*impl<Ti, Sw, Co, Da> TakeRequestTime<Ti> for SwitchEvent<Ti, Sw, Co, Da>
    where
        Ti: Clone,
    {
        type Rest = Self;

        fn take_time(self) -> (Ti, Self::Rest) {
            (self.time.clone(), self)
        }
    }*/

    impl<Ti, Sw, Co, Da> TakeSwitch<Sw> for SwitchEvent<Ti, Sw, Co, Da>
    where
        Sw: Clone,
    {
        type Rest = Self;

        fn take_switch(self) -> (Sw, Self::Rest) {
            (self.switch.clone(), self)
        }
    }

    impl<Ti, Co, Da> TakeSwitch<Switch> for SwitchEvent<Ti, KeyboardSwitch, Co, Da> {
        type Rest = Self;

        fn take_switch(self) -> (Switch, Self::Rest) {
            (Switch::Keyboard(self.switch.clone()), self)
        }
    }

    impl<Ti, Sw, Co, Rq> TakeRequest<Rq> for SwitchEvent<Ti, Sw, Co, Rq> {
        type Rest = SwitchEvent<Ti, Sw, Co, ()>;

        fn take_request(self) -> (Rq, Self::Rest) {
            (
                self.data,
                SwitchEvent::new(self.time, self.switch, self.coords),
            )
        }
    }

    impl<Ti, Sw, Co, Rq, Da> TakeRequest<Rq> for SwitchEvent<Ti, Sw, Co, (Rq, Da)> {
        type Rest = SwitchEvent<Ti, Sw, Co, Da>;

        fn take_request(self) -> (Rq, Self::Rest) {
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
