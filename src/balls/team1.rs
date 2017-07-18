use balls::helpers::base::Ball;

pub struct Team1Ball {
    pub color: [f32; 4],
    pub position: [f64; 4],
}

impl Team1Ball {
    pub fn new(position: [f64; 2]) -> Ball {
        let color_rgb: [f32; 3] = [1.0, 0.405, 0.24];

        Ball::new(
            color_rgb,
            position
        )
    }
}
