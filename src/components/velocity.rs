use specs::{Component, HashMapStorage};

pub struct Vel {
    pub x: f64,
    pub y: f64,
}

impl Component for Vel {
    type Storage = HashMapStorage<Self>;
}
