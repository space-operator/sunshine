use core::borrow::Borrow;
use std::collections::BTreeMap;

use crate::{LongPressHandleRequest, MultiClickHandleRequest};

#[derive(Clone, Debug)]
pub struct SchedulerState<Ti, Re> {
    requests: BTreeMap<Ti, Vec<Re>>,
}

pub type LongPressSchedulerState<Ti> = SchedulerState<Ti, LongPressHandleRequest>;
pub type MultiClickSchedulerState<Ti> = SchedulerState<Ti, MultiClickHandleRequest>;

impl<Ti, Re> SchedulerState<Ti, Re> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn into_requests(self) -> BTreeMap<Ti, Vec<Re>> {
        self.requests
    }

    pub fn schedule(self, time: Ti, request: Re) -> Self
    where
        Ti: Ord,
    {
        let mut requests = self.requests;
        let requests_by_time = requests.entry(time).or_default();
        requests_by_time.push(request);
        Self::from(requests)
    }

    pub fn scheduled<TiRef>(self, time: TiRef) -> (Self, (TiRef, Vec<Re>))
    where
        Ti: Ord,
        TiRef: Borrow<Ti>,
    {
        let mut scheduled = self.requests;
        let mut requests = scheduled.split_off(time.borrow());
        if let Some((key, value)) = requests.remove_entry(time.borrow()) {
            scheduled.insert(key, value);
        }
        (
            Self::from(requests),
            (time, scheduled.into_values().flatten().collect()),
        )
    }
}

impl<Ti, Re> From<BTreeMap<Ti, Vec<Re>>> for SchedulerState<Ti, Re> {
    fn from(requests: BTreeMap<Ti, Vec<Re>>) -> Self {
        Self { requests }
    }
}

impl<Ti, Re> Default for SchedulerState<Ti, Re> {
    fn default() -> Self {
        Self {
            requests: BTreeMap::default(),
        }
    }
}
