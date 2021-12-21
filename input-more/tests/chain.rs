#[test]
fn test_chain() {
    use core::fmt::Debug;

    use input_core::*;
    use input_more::*;

    type TimestampMs = u64;
    type Coords = (u64, u64);

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

    impl<Sw, Ti, Co> SwitchEvent<Sw, Ti, Co, ()> {
        fn with_data<Da>(self, data: Da) -> SwitchEvent<Sw, Ti, Co, Da> {
            SwitchEvent {
                switch: self.switch,
                time: self.time,
                coords: self.coords,
                data,
            }
        }
    }

    /*impl<Sw, Ti, Co> SplitEvent for SwitchEvent<Sw, Ti, Co, ()>
    where
        Sw: Clone,
    {
        type Data = Sw;
        type Event = Self;

        fn split(self) -> (Self::Data, Self::Event) {
            (self.switch, self)
        }
    }

    impl<Sw, Ti, Co, Da, T> UpgradeEvent<Modifiers<Sw>, T> for SwitchEvent<Sw, Ti, Co, Da> {
        type Output = SwitchEvent<Sw, Ti, Co, (T, Da)>;
        fn upgrade_with(self, data: T) -> Self::Output {
            Self::Output {
                switch: self.switch,
                time: self.time,
                coords: self.coords,
                data: (data, self.data),
            }
        }
    }

    impl<Sw, Ti, Co> SplitEvent for SwitchEvent<Sw, Ti, Co, (Modifiers<Sw>, ())>
    where
        Sw: Clone,
    {
        type Data = Sw;
        type Event = Self;

        fn split(self) -> (Self::Data, Self::Event) {
            (self.switch, self)
        }
    }

    impl<Sw, Ti, Co> SplitEvent
        for SwitchEvent<
            Sw,
            Ti,
            Co,
            (
                Modifiers<Sw>,
                (Result<LongPressHandleRequest, TimedPressError>, ()),
            ),
        >
    where
        Ti: Clone,
        Sw: Clone,
    {
        type Data = (Ti, LongPressHandleRequest);
        type Event = SwitchEvent<Sw, Ti, Co, (Modifiers<Sw>, ((), ()))>;

        fn split(self) -> (Self::Data, Self::Event) {
            let request = self.data.1 .0.unwrap();
            let time = self.time;
            let data = (self.data.0, ((), ()));
            let event = Self::Event {
                switch: self.switch,
                time: self.time,
                coords: self.coords,
                data,
            };
            ((self.time, request), event)
        }
    }

    type ChainProcessor<T> = (
        PopFirstState,
        (
            WithSplittedEvent,
            (T, (PushLastState, (WithUpgradedEvent, ()))),
        ),
    );*/

    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct BeforeModifiers;

    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct AfterModifiersBeforeTimed;

    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct AfterTimedBeforeScheduled;

    impl<Mo, St, Sw, Ti, Co> Processor<((Mo, St), SwitchEvent<Sw, Ti, Co, ()>)> for BeforeModifiers
    where
        Sw: Clone,
    {
        type Output = ((Mo, Sw), (St, SwitchEvent<Sw, Ti, Co, ()>));
        fn exec(
            &self,
            ((modifiers, state), event): ((Mo, St), SwitchEvent<Sw, Ti, Co, ()>),
        ) -> Self::Output {
            ((modifiers, event.switch.clone()), (state, event))
        }
    }

    impl<Mo, Er, Ts, St, Sw, Ti, Co>
        Processor<(
            (Mo, Result<(), Er>),
            ((Ts, St), SwitchEvent<Sw, Ti, Co, ()>),
        )> for AfterModifiersBeforeTimed
    where
        Mo: Clone,
        Er: Debug,
        St: ConsWithLast<Mo>,
        Sw: Clone,
    {
        type Output = ((Ts, Sw), (St::Output, SwitchEvent<Sw, Ti, Co, Mo>));
        fn exec(
            &self,
            ((modifiers, result), ((timed, state), event)): (
                (Mo, Result<(), Er>),
                ((Ts, St), SwitchEvent<Sw, Ti, Co, ()>),
            ),
        ) -> Self::Output {
            result.unwrap();
            (
                (timed, event.switch.clone()),
                (
                    state.with_last(modifiers.clone()),
                    event.with_data(modifiers),
                ),
            )
        }
    }

    impl<Ts, Re, Er, Sh, St, Sw, Ti, Co, Mo>
        Processor<(
            (Ts, Result<Re, Er>),
            ((Sh, St), SwitchEvent<Sw, Ti, Co, Mo>),
        )> for AfterTimedBeforeScheduled
    where
        Er: Debug,
        St: ConsWithLast<Ts>,
        Sw: Clone,
        Ti: Clone,
    {
        type Output = ((Sh, (Ti, Re)), (St::Output, SwitchEvent<Sw, Ti, Co, Mo>));
        fn exec(
            &self,
            ((timed, result), ((scheduled, state), event)): (
                (Ts, Result<Re, Er>),
                ((Sh, St), SwitchEvent<Sw, Ti, Co, Mo>),
            ),
        ) -> Self::Output {
            let request = result.unwrap();
            (
                (scheduled, (event.time.clone(), request)), // FIXME: time + duration
                (state.with_last(timed), event),
            )
        }
    }

    // Should be ( SchedulerProcessor, ());
    // No idea about error fix yet
    type SchedulerProcessorDummy = ();

    type KeyboardDownProcessor = (
        (
            (BeforeModifiers, (ModifiersPressProcessor, ())),
            (AfterModifiersBeforeTimed, (TimedPressProcessor, ())),
        ),
        (
            ((AfterTimedBeforeScheduled, (SchedulerProcessor, ())), ()),
            (),
        ),
    );
    /*
        (original_data, ..., ...)
            .before_modifiers()
            .modifiers_press_processor()
    */

    // Keyboard switch event
    let event = SwitchEvent {
        switch: KeyboardSwitch("LeftCtrl"),
        time: 1000,
        coords: (),
        data: (),
    };

    let state = (
        Modifiers::new(),
        (TimedState::new(), (SchedulerState::new(), ())),
    );

    let result = KeyboardDownProcessor::default().exec((state, event));
    println!("{:?}", result);
    panic!();
}
