use ggez::graphics::Color;


pub struct Graph {
    current_points: [f64; 1920],
    target_points: [f64; 1920],
    pub color: Color,
}


impl Graph {
    pub fn new(points: [f64; 1920], color: Color) -> Self {
        Self {
            current_points: points.clone(),
            target_points: points,
            color
        }
    }

    pub fn get_points(&self) -> &[f64] {
        return self.current_points.as_slice();
    }

    pub fn lerp_points(&mut self) {
        for i in 0..1920 {
            self.current_points[i] = (self.target_points[i] - self.current_points[i]) / 2.0;
        }
    }

    pub fn update_func(&mut self, func: fn(i64) -> f64) {
        for i in 0..1920 {
            self.target_points[i] = func(i as i64); // TODO: Make this correct (not i as i64)
        }
    }
}
