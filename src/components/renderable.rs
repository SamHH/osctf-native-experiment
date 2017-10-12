use specs::{Component, VecStorage};

#[derive(PartialEq)]
pub enum Model {
    Ball,
}

pub struct Renderable {
    pub model: Model,
    pub color: Option<[f32; 4]>,
}

impl Component for Renderable {
    type Storage = VecStorage<Self>;
}
