pub type PressMapping<Sw, Mo> = Mapping<Sw, Mo, (), BasicAppEventBuilder>;
pub type ReleaseMapping = Mapping<Sw, Mo, Option<TimedReleaseEventData>, BasicAppEventBuilder>;
pub type LongPressMapping = Mapping<Sw, Mo, TimedLongPressEventData, BasicAppEventBuilder>;
pub type ClickExactMapping = Mapping<Sw, Mo, TimedClickExactEventData, BasicAppEventBuilder>;

#[derive(Clone, Debug)]
pub struct GlobalMapping {
    keyboard_press: KeyboardPressMapping,
    keyboard_release: KeyboardReleaseMapping,
    keyboard_long_press: KeyboardLongPressMapping,
    keyboard_click_exact: KeyboardClickExactMapping,
}
