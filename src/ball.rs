pub struct Ball {
    pub color: [f32; 4],
    pub position: [f64; 4],
    velocity: [f64; 2],
    max_velocity: f64,
    acceleration_diff: f64,
}

impl Ball {
    pub fn new() -> Self {
        Ball {
            // RGB, alpha ?
            color: [0.0, 0.2, 0.85, 1.0],
            /*
                Offset from left in px
                Offset from top in px
                Horizontal size/scaling ?
                Vertical size/scaling ?
            */
            position: [200.0, 200.0, 50.0, 50.0],
            // To be factored into calc (X, Y). Range -1.0 to 1.0
            velocity: [0.0, 0.0],
            max_velocity: 1.0,
            acceleration_diff: 0.4
        }
    }

    pub fn update(&mut self, dt: f64) {
        // Indices represent Y (0) and X (1). At the moment "15.0" is a random
        // number for controlling the approximate speed we want. This may not be
        // optimal. "dt" is delta time to avoid tying game speed logic to speed
        // of host computer. "80.0" is presently an arbitrary value for
        // controlling speed
        for i in 0..2 {
            self.position[i] += self.velocity[i] * dt * 80.0;
        }
    }

    // TODO research this &'static thing
    pub fn apply_movement(&mut self, direction: &str) {
        let (axis, increase) = match direction {
            "up" => { (1, false) }
            "down" => { (1, true) }
            "left" => { (0, false) }
            "right" => { (0, true) }
            _ => panic!("Unexpected direction supplied {}", direction)
        };

        let calc_accel: f64 = if increase { self.acceleration_diff } else { self.acceleration_diff * -1.0 };

        // Don't allow below (max_velocity * -1) or above (max_velocity)
        let mut calc_velocity: f64 = self.max_velocity.min(self.velocity[axis] + calc_accel);
        calc_velocity = calc_velocity.max(self.max_velocity * -1.0);

        self.velocity[axis] = calc_velocity;
    }
}
