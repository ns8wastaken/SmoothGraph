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
            let x = self.target_points[i] - self.current_points[i];
            self.current_points[i] += x / 1.0;
        }
    }

    pub fn update_func(&mut self, x_origin: f64, unit_length: f64, func: fn(f64) -> f64) {
        for i in 0..1920 {
            self.target_points[i] = func((i as f64 - x_origin) / unit_length);
        }
    }
}
