use components::position::Pos;
use components::velocity::Vel;
use resources::dt::DeltaTime;
use specs::{Fetch, Join, ReadStorage, System, WriteStorage};

pub struct UpdatePos;

impl<'a> System<'a> for UpdatePos {
    type SystemData = (
        Fetch<'a, DeltaTime>,
        ReadStorage<'a, Vel>,
        WriteStorage<'a, Pos>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (delta, vel, mut pos) = data;

        let dt = delta.0;

        for (vel, pos) in (&vel, &mut pos).join() {
            pos.x += vel.x * dt;
            pos.y += vel.y * dt;
        }
    }
}
