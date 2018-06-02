use components::player::Player;
use components::velocity::Vel;
use resources::player_input::PlayerInput;
use specs::{Join, Read, ReadStorage, System, WriteStorage};

pub struct InterpretInput;

impl<'a> System<'a> for InterpretInput {
    type SystemData = (
        Read<'a, PlayerInput>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, Vel>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (input, is_player, mut vel) = data;

        for (_, vel) in (&is_player, &mut vel).join() {
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
