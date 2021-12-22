use core::borrow::Borrow;
use std::collections::BTreeMap;

use crate::{LongPressHandleRequest, MultiClickHandleRequest};

#[derive(Clone, Debug)]
pub struct SchedulerState<Ti, Sw, Rq> {
    requests: BTreeMap<Ti, Vec<(Sw, Rq)>>,
}

pub type LongPressSchedulerState<Ti, Sw> = SchedulerState<Ti, Sw, LongPressHandleRequest>;
pub type MultiClickSchedulerState<Ti, Sw> = SchedulerState<Ti, Sw, MultiClickHandleRequest>;

impl<Ti, Sw, Rq> SchedulerState<Ti, Sw, Rq> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn into_requests(self) -> BTreeMap<Ti, Vec<(Sw, Rq)>> {
        self.requests
    }

    pub fn schedule(self, time: Ti, switch: Sw, request: Rq) -> Self
    where
        Ti: Ord,
    {
        let mut requests = self.requests;
        let requests_by_time = requests.entry(time).or_default();
        requests_by_time.push((switch, request));
        Self::from(requests)
    }

    pub fn take_scheduled<TiRef>(self, time: TiRef) -> (Self, (TiRef, Vec<(Ti, Vec<(Sw, Rq)>)>))
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
            (time, scheduled.into_iter().collect()),
        )
    }
}

impl<Ti, Sw, Rq> From<BTreeMap<Ti, Vec<(Sw, Rq)>>> for SchedulerState<Ti, Sw, Rq> {
    fn from(requests: BTreeMap<Ti, Vec<(Sw, Rq)>>) -> Self {
        Self { requests }
    }
}

impl<Ti, Sw, Rq> Default for SchedulerState<Ti, Sw, Rq> {
    fn default() -> Self {
        Self {
            requests: BTreeMap::default(),
        }
    }
}
