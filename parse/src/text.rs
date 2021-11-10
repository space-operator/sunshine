#[derive(Clone, Debug)]
pub struct Text {
    value: String,
    style: Style,
}

#[derive(Clone, Debug)]
pub enum Style {
    Default,
    Bold,
    SuperBold,
}
