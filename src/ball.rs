pub struct Ball {
    pub color: [f32; 4],
    pub position: [f64; 4],
    velocity: [f64; 2]
}

impl Ball {
    pub fn new() -> Self {
        Ball {
            color: [1.0, 1.0, 1.0, 1.0],
            position: [0.0, 0.0, 100.0, 100.0],
            velocity: [0.3, 0.3]
        }
    }

    pub fn update(&mut self, dt: f64, window_size: [f64; 2]) {
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

    pub fn change_velocity(&mut self, factor: f64) {
        self.velocity[0] *= factor;
        self.velocity[1] *= factor;
    }
}
