/// A generic structure that stores input mapping rules for keyboard and mouse devices.
#[derive(Clone, Debug)]
pub struct GlobalMapping<Ke, Mo> {
    /// Keyboard device mapping rules.
    pub keyboard: Ke,
    /// Mouse device mapping rules.
    pub mouse: Mo,
}
