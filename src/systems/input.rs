use components::velocity::Vel;
use resources::player_entity::PlayerEntity;
use resources::player_input::PlayerInput;
use specs::{Fetch, System, WriteStorage};

pub struct InterpretInput;

impl<'a> System<'a> for InterpretInput {
    type SystemData = (
        Fetch<'a, PlayerEntity>,
        Fetch<'a, PlayerInput>,
        WriteStorage<'a, Vel>,
    );

    fn run(&mut self, (player, input, mut velc): Self::SystemData) {
        if let Some(vel) = player.0.and_then(|ent| velc.get_mut(ent)) {
            if input.up && !input.down {
                vel.y -= 1.0;
            }
            if !input.up && input.down {
                vel.y += 1.0;
            }
            if input.left && !input.right {
                vel.x -= 1.0;
            }
            if !input.left && input.right {
                vel.x += 1.0;
            }
        }
    }
}
