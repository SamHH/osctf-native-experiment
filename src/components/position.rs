use specs::{Component, VecStorage};

pub struct Pos {
    pub x: f64,
    pub y: f64,
}

impl Component for Pos {
    type Storage = VecStorage<Self>;
}
