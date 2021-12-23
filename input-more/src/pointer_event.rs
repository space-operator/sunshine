use core::borrow::Borrow;
use core::hash::Hash;

use input_core::{PointerChangeEventData, PointerMoveEventData, PointerState};

use crate::{Context, TakeCoords, TakeIsDraggedFn, TakeState, TakeSwitch, WithState};

impl<St, Ev> Context<St, Ev> {
    pub fn with_pointer_change_event<Re, Ev2, Sw, Co>(
        self,
    ) -> Context<Re::Output, (Option<PointerChangeEventData<Sw>>, Ev2::Rest)>
    where
        St: TakeState<PointerState<Sw, Co>, Rest = Re>,
        Re: WithState<PointerState<Sw, Co>>,
        Ev: TakeSwitch<Sw, Rest = Ev2>,
        Ev2: TakeCoords<Co>,
        Sw: Clone + Eq + Hash,
    {
        let (state, rest) = self.state.take_state();
        let (switch, event) = self.event.take_switch();
        let (coords, event) = event.take_coords();
        let (state, data) = state.with_change_event(switch, coords);
        Context::new(rest.with_state(state), (data, event))
    }
}

impl<St, Ev> Context<St, Ev> {
    pub fn with_pointer_move_event<Re, Sw, Co, F>(
        self,
    ) -> Context<Re::Output, ((Vec<PointerMoveEventData<Sw>>, F), Ev::Rest)>
    where
        St: TakeState<PointerState<Sw, Co>, Rest = Re>,
        Re: WithState<PointerState<Sw, Co>>,
        Ev: TakeIsDraggedFn<F>,
        Sw: Clone + Eq + Hash,
        F: FnMut(&Co) -> bool,
    {
        let (state, rest) = self.state.take_state();
        let (is_dragged_fn, event) = self.event.take_is_dragged_fn();
        let (state, data) = state.with_move_event(is_dragged_fn);
        Context::new(rest.with_state(state), (data, event))
    }
}
