extern crate piston_window;
use piston_window::*;

mod ball;
use ball::Ball;

mod fps_counter;
use fps_counter::FPSCounter;

fn main() {
    let mut ball1 = Ball::new();
    let mut window: PistonWindow =
        WindowSettings::new("Hello World! :-)", [640, 480])
        .exit_on_esc(true)
        .vsync(true)
        .build().unwrap();

    let mut window_size: [f64; 2] = [0.0, 0.0];
    let mut fps_counter = FPSCounter::new();

    while let Some(e) = window.next() {
        match e {
            Input::Render(r) => {
                window_size = [r.width as f64, r.height as f64];
                window.draw_2d(&e, |c, g| {
                    // Here we can access the functions here: http://docs.piston.rs/mush/graphics/fn.ellipse.html

                    // Background color
                    clear([1.0; 4], g);

                    // Ball
                    ellipse(ball1.color, ball1.position, c.transform, g);
                });
                println!("{:.0} FPS", fps_counter.get_fps());
            }

            Input::Update(u) => {
                ball1.update(u.dt, window_size);
                fps_counter.update(u.dt, 0.25);
            }

            Input::Press(b) => {
                       match b {
                           Button::Keyboard(k) => {
                               match k {
                                   Key::W => {
                                       ball1.change_velocity(1.1);
                                   }
                                   Key::S => {
                                       ball1.change_velocity(0.9);
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
