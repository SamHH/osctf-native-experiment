use balls::helpers::base::Ball;

pub struct Team2Ball {
    pub color: [f32; 4],
    pub position: [f64; 4],
}

impl Team2Ball {
    pub fn new(position: [f64; 2]) -> Ball {
        let color_rgb: [f32; 3] = [0.355, 0.24, 1.0];

        Ball::new(
            color_rgb,
            position
        )
    }
}
