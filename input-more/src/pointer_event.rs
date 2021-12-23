use core::borrow::Borrow;
use core::hash::Hash;

use input_core::{PointerChangeEventData, PointerMoveEventData, PointerState};

use crate::{Context, Split, TakeState, WithState};

impl<St, Ev> Context<St, Ev> {
    pub fn with_pointer_change_event<Re, Ki1, Ev2, Ki2, Ev3, Sw, Co>(
        self,
    ) -> Context<Re::Output, (Option<PointerChangeEventData<Sw>>, Ev3)>
    where
        St: TakeState<PointerState<Sw, Co>, Rest = Re>,
        Re: WithState<PointerState<Sw, Co>>,
        Ev: Split<Sw, Ev2, Ki1>,
        Ev2: Split<Co, Ev3, Ki2>,
        Sw: Clone + Eq + Hash,
    {
        let (state, rest) = self.state.take_state();
        let (switch, event) = self.event.split();
        let (coords, event) = event.split();
        let (state, data) = state.with_change_event(switch, coords);
        Context::new(rest.with_state(state), (data, event))
    }
}

impl<St, Ev> Context<St, Ev> {
    pub fn with_pointer_move_event<Re, Ki1, Ev2, Sw, Co, F>(
        self,
    ) -> Context<Re::Output, ((Vec<PointerMoveEventData<Sw>>, F), Ev2)>
    where
        St: TakeState<PointerState<Sw, Co>, Rest = Re>,
        Re: WithState<PointerState<Sw, Co>>,
        Ev: Split<F, Ev2, Ki1>,
        Sw: Clone + Eq + Hash,
        F: FnMut(&Co) -> bool,
    {
        let (state, rest) = self.state.take_state();
        let (is_dragged_fn, event) = self.event.split();
        let (state, data) = state.with_move_event(is_dragged_fn);
        Context::new(rest.with_state(state), (data, event))
    }
}
