use crate::*;
use input_core::*;

pub type PressMapping<Sw, Mo, Pd, Bu> = SwitchMapping<Sw, Mo, (), Pd, Bu>;
pub type ReleaseMapping<Sw, Mo, Pd, Bu> =
    SwitchMapping<Sw, Mo, Option<TimedReleaseEventData>, Pd, Bu>;
pub type LongPressMapping<Sw, Mo, Pd, Bu> = SwitchMapping<Sw, Mo, TimedLongPressEventData, Pd, Bu>;
pub type ClickExactMapping<Sw, Mo, Pd, Bu> =
    SwitchMapping<Sw, Mo, TimedClickExactEventData, Pd, Bu>;

#[derive(Clone, Debug)]
pub struct DeviceMapping<Sw, Mo, Pd, Bu> {
    press: PressMapping<Sw, Mo, Pd, Bu>,
    release: ReleaseMapping<Sw, Mo, Pd, Bu>,
    long_press: LongPressMapping<Sw, Mo, Pd, Bu>,
    click_exact: ClickExactMapping<Sw, Mo, Pd, Bu>,
    // pointer-move-mapping
    // trigger
}
