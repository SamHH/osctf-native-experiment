extern crate find_folder;
extern crate piston_window;
extern crate specs;
extern crate update_rate;

mod components;
mod config;
mod resources;
mod systems;

use piston_window::{Glyphs, PistonWindow, TextureSettings};

fn get_glyphs(window: &PistonWindow) -> Glyphs {
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
    println!("{:?}", assets);

    let ref font = assets.join("FiraSans-Regular.ttf");
    let factory = window.factory.clone();

    return Glyphs::new(font, factory, TextureSettings::new()).unwrap();
}

fn main() {
    use components::position::Pos;
    use components::renderable::Renderable;
    use components::renderable::Model::Ball;
    use components::team::Team;
    use components::velocity::Vel;
    use piston_window::{clear, ellipse, Button, ButtonEvent, Key, Text, RenderEvent, Transformed,
                        UpdateEvent, WindowSettings};
    use resources::dt::DeltaTime;
    use specs::{DispatcherBuilder, Join, World};
    use update_rate::{RateCounter, RollingRateCounter};

    // Window
    let mut window: PistonWindow = WindowSettings::new(config::APP_NAME, [640, 480])
        .exit_on_esc(true)
        .fullscreen(false)
        .vsync(true)
        .opengl(config::OPENGL_VERSION)
        .build()
        .unwrap();

    // Assets (font)
    let mut glyphs = get_glyphs(&window);

    // FPS counter
    let mut fps_counter = RollingRateCounter::new(60);
    let fps_text = Text::new(10);

    // Define teams
    let std_team_1: Team = Team {
        id: "std_team_1",
        name: "Orange Team",
        color_rgba: [1.0, 0.405, 0.24, 1.0],
    };

    let std_team_2: Team = Team {
        id: "std_team_2",
        name: "Purple Team",
        color_rgba: [0.332, 0.22, 1.0, 1.0],
    };

    // Define ECS world and its components
    let mut world = World::new();
    world.register::<Pos>();
    world.register::<Renderable>();
    world.register::<Team>();
    world.register::<Vel>();

    // Balls
    world.create_entity()
        .with(Renderable { model: Ball, color: None })
        .with(std_team_1)
        .with(Pos { x: 200.0, y: 200.0 })
        .build();

    world.create_entity()
        .with(Renderable { model: Ball, color: None })
        .with(std_team_2)
        .with(Pos { x: 300.0, y: 300.0 })
        .build();

    // Add ECS resources w/ initial values
    world.add_resource(DeltaTime(0.0));

    // Build an ECS dispatcher, and add the systems to it
    let mut dispatcher = DispatcherBuilder::new()
        .add(systems::controls::InterpretControls, "interpret_controls", &[])
        .add(systems::movement::UpdatePos, "update_pos", &["interpret_controls"])
        .build();

    // Game loop
    while let Some(evt) = window.next() {
        if evt.render_args().is_some() {
            window.draw_2d(&evt, |c, g| {
                // Render background color
                clear([1.0; 4], g);

                // Render text
                let fps_text_position = c.transform.trans(5.0, 15.0);
                fps_text
                    .draw(
                        &format!("{:.0} FPS", fps_counter.rate()),
                        &mut glyphs,
                        &c.draw_state,
                        fps_text_position,
                        g,
                    )
                    .unwrap();

                // Render ECS entities
                let positions = world.read::<Pos>();
                let renderables = world.read::<Renderable>();
                let teams = world.read::<Team>();

                for entity in world.entities().join() {
                    if let (
                        Some(ren),
                        Some(pos),
                    ) = (
                        renderables.get(entity),
                        positions.get(entity),
                    ) {
                        if ren.model == Ball {
                            // Priority: team, individual. Fallback: transparent.
                            let color_rgba: [f32; 4] = match teams.get(entity) {
                                Some(team) => team.color_rgba,
                                None => match ren.color {
                                    Some(color) => color,
                                    None => [0.0; 4],
                                },
                            };

                            ellipse(
                                color_rgba,
                                [pos.x, pos.y, 50.0, 50.0], // X, Y, W, H
                                c.transform, // ?
                                g, // ?
                            );
                        }
                    }
                }
            });
        }

        if let Some(update) = evt.update_args() {
            // Update FPS counter
            fps_counter.update();

            // Update ECS resources
            {
                let mut delta = world.write_resource::<DeltaTime>();
                delta.0 = update.dt;
            }

            // Execute all ECS systems
            dispatcher.dispatch(&mut world.res);
        }

        if let Some(btn) = evt.button_args() {
            match btn.button {
                Button::Keyboard(key) => {
                    match key {
                        Key::W => {
                            // user_ball.apply_movement("up");
                        }
                        Key::A => {
                            // user_ball.apply_movement("left");
                        }
                        Key::S => {
                            // user_ball.apply_movement("down");
                        }
                        Key::D => {
                            // user_ball.apply_movement("right");
                        }
                        Key::X => {
                            // Reset the ball
                            // user_ball = Team1Ball::new([200.0, 200.0]);
                        }
                        _ => {} // Catch all keys
                    };
                }
                _ => {} // Catch non-keyboard buttons
            };
        }
    }
}
