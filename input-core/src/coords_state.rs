/// A structure that stores last device coordinates.
#[derive(Clone, Debug)]
pub struct CoordsState<Co> {
    coords: Co,
}

impl<Co> CoordsState<Co> {
    /// Constructs a `CoordsState` with specified coordinates.
    pub fn with_coords(coords: Co) -> Self {
        Self { coords }
    }

    /// Sets device coordinates.
    pub fn set_coords(&mut self, coords: Co) {
        self.coords = coords;
    }

    /// Returns a reference to the contained coordinates.
    pub fn coords(&self) -> &Co {
        &self.coords
    }
}
