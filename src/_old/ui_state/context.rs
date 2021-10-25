use std::sync::Arc;

use crate::ui_event::*;

pub type ScheduledTimeout = Arc<UiEventTimeStampMs>;

pub trait Context {
    fn max_click_time(&self) -> UiEventTimeDeltaMs;
    fn max_dbl_click_interval(&self) -> UiEventTimeDeltaMs;
    fn min_long_touch_time(&self) -> UiEventTimeDeltaMs;

    fn schedule_timeout(&mut self, timestamp: UiEventTimeStampMs) -> ScheduledTimeout;

    fn emit_event(&mut self, ev: UiEvent);
}
