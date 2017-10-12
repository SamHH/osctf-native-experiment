extern crate find_folder;
extern crate piston_window;
extern crate update_rate;

mod config;
mod balls;

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
    use piston_window::{clear, ellipse, Button, ButtonEvent, Key, Text, RenderEvent, Transformed,
                        UpdateEvent, WindowSettings};
    use update_rate::{RateCounter, RollingRateCounter};
    use balls::team1::Team1Ball;
    use balls::team2::Team2Ball;

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

    // Balls
    let mut user_ball = Team1Ball::new([200.0, 200.0]);
    let other_ball = Team2Ball::new([300.0, 200.0]);

    // Game loop
    while let Some(evt) = window.next() {
        if evt.render_args().is_some() {
            window.draw_2d(&evt, |c, g| {
                // Background color
                clear([1.0; 4], g);

                // Text
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

                // Ball
                ellipse(user_ball.color, user_ball.position, c.transform, g);
                ellipse(other_ball.color, other_ball.position, c.transform, g);
            });
        }

        if let Some(update) = evt.update_args() {
            user_ball.update(update.dt);

            fps_counter.update();
        }

        if let Some(btn) = evt.button_args() {
            match btn.button {
                Button::Keyboard(key) => {
                    match key {
                        Key::W => {
                            user_ball.apply_movement("up");
                        }
                        Key::A => {
                            user_ball.apply_movement("left");
                        }
                        Key::S => {
                            user_ball.apply_movement("down");
                        }
                        Key::D => {
                            user_ball.apply_movement("right");
                        }
                        Key::X => {
                            // Reset the ball
                            user_ball = Team1Ball::new([200.0, 200.0]);
                        }
                        _ => {} // Catch all keys
                    };
                }
                _ => {} // Catch non-keyboard buttons
            };
        }
    }
}
