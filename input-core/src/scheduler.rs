use core::borrow::Borrow;
use std::collections::BTreeMap;

use crate::{LongPressHandleRequest, MultiClickHandleRequest};

#[derive(Clone, Debug)]
pub struct SchedulerState<Ti, Sw, Re> {
    requests: BTreeMap<Ti, Vec<(Sw, Re)>>,
}

pub type LongPressSchedulerState<Ti, Sw> = SchedulerState<Ti, Sw, LongPressHandleRequest>;
pub type MultiClickSchedulerState<Ti, Sw> = SchedulerState<Ti, Sw, MultiClickHandleRequest>;

impl<Ti, Sw, Re> SchedulerState<Ti, Sw, Re> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn into_requests(self) -> BTreeMap<Ti, Vec<(Sw, Re)>> {
        self.requests
    }

    pub fn schedule(self, time: Ti, switch: Sw, request: Re) -> Self
    where
        Ti: Ord,
    {
        let mut requests = self.requests;
        let requests_by_time = requests.entry(time).or_default();
        requests_by_time.push((switch, request));
        Self::from(requests)
    }

    pub fn take_scheduled<TiRef>(self, time: TiRef) -> (Self, (TiRef, Vec<(Sw, Re)>))
    where
        Ti: Ord,
        TiRef: Borrow<Ti>,
    {
        let mut scheduled = self.requests;
        let mut requests = scheduled.split_off(time.borrow());
        if let Some((key, value)) = requests.remove_entry(time.borrow()) {
            let prev = scheduled.insert(key, value);
            assert!(prev.is_none());
        }
        (
            Self::from(requests),
            (time, scheduled.into_values().flatten().collect()),
        )
    }
}

impl<Ti, Sw, Re> From<BTreeMap<Ti, Vec<(Sw, Re)>>> for SchedulerState<Ti, Sw, Re> {
    fn from(requests: BTreeMap<Ti, Vec<(Sw, Re)>>) -> Self {
        Self { requests }
    }
}

impl<Ti, Sw, Re> Default for SchedulerState<Ti, Sw, Re> {
    fn default() -> Self {
        Self {
            requests: BTreeMap::default(),
        }
    }
}
