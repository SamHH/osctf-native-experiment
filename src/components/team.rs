use specs::{Component, HashMapStorage};

pub struct Team {
    pub id: &'static str,
    pub name: &'static str,
    pub color_rgba: [f32; 4],
}

impl Component for Team {
    type Storage = HashMapStorage<Self>;
}
