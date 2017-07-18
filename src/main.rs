extern crate find_folder;
extern crate piston_window;
extern crate update_rate;

use piston_window::*;
use update_rate::UpdateRateCounter;

mod ball;
use ball::Ball;

fn get_glyphs(window: &PistonWindow) -> Glyphs {
    let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
    println!("{:?}", assets);

    let ref font = assets.join("FiraSans-Regular.ttf");
    let factory = window.factory.clone();

    return Glyphs::new(font, factory).unwrap();
}

fn main() {
    // Window
    let mut window: PistonWindow =
        WindowSettings::new("Hello World! :-)", [640, 480])
            .exit_on_esc(true)
            .fullscreen(false)
            .vsync(true)
            .opengl(OpenGL::V3_2)
            .build().unwrap();

    // Assets (font)
    let mut glyphs = get_glyphs(&window);

    // FPS counter
    let mut fps_counter = UpdateRateCounter::new(60);
    let fps_text = Text::new(10);

    // Ball
    let mut ball1 = Ball::new();

    while let Some(e) = window.next() {
        match e {
            Input::Render(r) => {
                window.draw_2d(&e, |c, g| {
                    // Here we can access the functions here:
                    // http://docs.piston.rs/mush/graphics/

                    // Background color
                    clear([1.0; 4], g);

                    // Text
                    let fps_text_position = c.transform.trans(5.0, 15.0);
                    fps_text.draw(
                        &format!("{:.0} FPS", fps_counter.rate()),
                        &mut glyphs,
                        &c.draw_state,
                        fps_text_position,
                        g
                    );

                    // Ball
                    ellipse(ball1.color, ball1.position, c.transform, g);
                });
            }

            Input::Update(u) => {
                ball1.update(u.dt);
                fps_counter.update();
            }

            Input::Press(b) => {
                       match b {
                           Button::Keyboard(k) => {
                               match k {
                                   Key::W => {
                                       ball1.apply_movement("up");
                                   }
                                   Key::A => {
                                       ball1.apply_movement("left");
                                   }
                                   Key::S => {
                                       ball1.apply_movement("down");
                                   }
                                   Key::D => {
                                       ball1.apply_movement("right");
                                   }
                                   Key::F5 => {
                                       // Reset the ball
                                       ball1 = Ball::new();
                                   }
                                   _ => {} // Catch all keys
                               };
                           }
                           _ => {} // Catch non-keyboard buttons
                       };
                    }
            _ => {} // Catch uninteresting events
        }
    }
}
