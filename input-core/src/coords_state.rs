#[derive(Clone, Debug)]
pub struct CoordsState<Co> {
    coords: Co,
}

impl<Co> CoordsState<Co> {
    pub fn with_coords(coords: Co) -> Self {
        Self { coords }
    }

    pub fn coords(&self) -> &Co {
        &self.coords
    }
}
