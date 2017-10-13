use components::player::Player;
use components::position::Pos;
use components::renderable::Renderable;
use components::renderable::Model::Ball;
use components::team::Team;
use components::velocity::Vel;
use specs::{Entity, World};

// Create a ball
pub fn create_other(world: &mut World, team: Team) -> Entity {
    return world.create_entity()
        .with(Renderable { model: Ball, color: None })
        .with(team)
        .with(Pos { x: 200.0, y: 200.0 }) // TODO fetch these coords from map spawn points data
        .with(Vel { x: 0.0, y: 0.0 })
        .build();
}


// Create a ball, and add the Player component to it
pub fn create_player(mut world: &mut World, team: Team) -> Entity {
    let ball_entity = create_other(&mut world, team);

    world.write::<Player>().insert(ball_entity, Player);

    return ball_entity;
}
