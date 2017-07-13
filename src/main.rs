extern crate piston_window;
use piston_window::*;

struct ColoredRect {
    pub color: [f32; 4],
    pub position: [f64; 4],
    velocity: [f64; 2]
}

impl ColoredRect {
    fn new() -> Self {
        ColoredRect {
            color: [1.0, 1.0, 1.0, 1.0],
            position: [0.0, 0.0, 100.0, 100.0],
            velocity: [0.3, 0.3]
        }
    }
    fn update(&mut self, dt: f64, window_size: [f64; 2]) {
        // Update color RGB values
        self.color[0] = Self::update_color(dt as f32, self.color[0], 0.001);
        self.color[1] = Self::update_color(dt as f32, self.color[1], 0.002);
        self.color[2] = Self::update_color(dt as f32, self.color[2], 0.003);

        // Invert velocities if we hit a window border, then add velocity to the
        // current position. Indices represent X (0) and Y (1)
        for (i, size) in window_size.iter().enumerate() {
            if
                self.position[i] + self.position[i + 2] >= size.abs() ||
                self.position[i] < 0.0
            { self.velocity[i] = -self.velocity[i]; }

            self.position[i] += self.velocity[i] * dt * 120.0;
        }
    }
    fn update_color(dt: f32, color: f32, change: f32)->f32 {
        if color <= 0.0 { 1.0 }
        else { color - (change * dt * 120.0) }
    }
    fn change_velocity(&mut self, factor: f64) {
        self.velocity[0] *= factor;
        self.velocity[1] *= factor;
    }
}

fn main() {
    let mut rect = ColoredRect::new();
    let mut window: PistonWindow =
        WindowSettings::new("Hello World! :-)", [640, 480])
        .exit_on_esc(true)
        .vsync(true)
        .build().unwrap();

    let mut window_size: [f64; 2] = [0.0, 0.0];

    while let Some(e) = window.next() {
        match e {
            Input::Render(r) => {
                window_size = [r.width as f64, r.height as f64];
                window.draw_2d(&e, |c, g| {
                    clear([1.0; 4], g); // Clear to white
                    rectangle(rect.color, // Color
                              rect.position, // Position/size
                              c.transform, g);
                });
            }
            Input::Update(u) => {
                rect.update(u.dt, window_size);
            }
            Input::Press(b) => {
                       match b {
                           Button::Keyboard(k) => {
                               match k {
                                   Key::W => {
                                       rect.change_velocity(1.1);
                                   }
                                   Key::S => {
                                       rect.change_velocity(0.9);
                                   }
                                   Key::F5 => {
                                       // Reset the rectangle
                                       rect = ColoredRect::new();
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