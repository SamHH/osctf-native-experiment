extern crate piston_window;
extern crate specs;
extern crate update_rate;

mod components;
mod config;
mod helpers;
mod resources;
mod systems;

use components::player::Player;
use components::position::Pos;
use components::renderable::Renderable;
use components::renderable::Model::Ball;
use components::team::Team;
use components::velocity::Vel;
use helpers::{balls, glyphs, teams};
use piston_window::{clear, ellipse, Button, ButtonEvent, ButtonState, Key, PistonWindow, Text, RenderEvent, Transformed, UpdateEvent, WindowSettings};
use resources::player_input::PlayerInput;
use resources::dt::DeltaTime;
use specs::{DispatcherBuilder, Join, World};
use update_rate::{RateCounter, RollingRateCounter};

fn main() {
    // Window
    let mut window: PistonWindow = WindowSettings::new(config::APP_NAME, [640, 480])
        .exit_on_esc(true)
        .fullscreen(false)
        .vsync(true)
        .opengl(config::OPENGL_VERSION)
        .build()
        .unwrap();

    // Assets (font)
    let mut glyphs = glyphs::get(&window);

    // FPS counter
    let mut fps_counter = RollingRateCounter::new(60);
    let fps_text = Text::new(10);

    // Define ECS world and its components
    let mut world = World::new();
    world.register::<Player>();
    world.register::<Pos>();
    world.register::<Renderable>();
    world.register::<Team>();
    world.register::<Vel>();

    // Define teams and create ECS ball entities
    let (team1, team2) = teams::get_std();
    balls::create_player(&mut world, team1);
    balls::create_other(&mut world, team2);

    // Add ECS resources w/ initial values
    world.add_resource(PlayerInput { up: false, down: false, left: false, right: false });
    world.add_resource(DeltaTime(0.0));

    // Build an ECS dispatcher, and add the systems to it
    let mut dispatcher = DispatcherBuilder::new()
        .with(systems::input::InterpretInput, "interpret_input", &[])
        .with(systems::movement::UpdatePos, "update_pos", &["interpret_input"])
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
                let positions = world.read_storage::<Pos>();
                let renderables = world.read_storage::<Renderable>();
                let teams = world.read_storage::<Team>();

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
            let btn_active: bool =
                if btn.state == ButtonState::Press { true }
                else { false };

            match btn.button {
                Button::Keyboard(key) => {
                    let mut input = world.write_resource::<PlayerInput>();

                    match key {
                        Key::W => {
                            input.up = btn_active;
                        }
                        Key::A => {
                            input.left = btn_active;
                        }
                        Key::S => {
                            input.down = btn_active;
                        }
                        Key::D => {
                            input.right = btn_active;
                        }
                        // Key::X => {
                        //     // Reset the ball
                        //     // user_ball = Team1Ball::new([200.0, 200.0]);
                        // }
                        _ => {}
                    };
                }
                _ => {}
            };
        }
    }
}
