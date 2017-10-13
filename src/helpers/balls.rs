use components::position::Pos;
use components::renderable::Renderable;
use components::renderable::Model::Ball;
use components::team::Team;
use components::velocity::Vel;
use resources::player_entity::PlayerEntity;
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


// Create a ball, and pass it on as the player resource entity
pub fn create_player(mut world: &mut World, team: Team) -> Entity {
    let ball_entity = create_other(&mut world, team);

    let mut player_entity = world.write_resource::<PlayerEntity>();
    player_entity.0 = Some(ball_entity);

    return ball_entity;
}
