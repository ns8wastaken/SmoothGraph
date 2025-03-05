pub struct Graph {
    current_points: [f64; 1920],
    target_points: [f64; 1920],
    thickness: f32,
    color: (u8, u8, u8),
}


impl Graph {
    pub fn new(points: [f64; 1920], thickness: f32, color: (u8, u8, u8)) -> Self {
        Self {
            current_points: points.clone(),
            target_points: points,
            thickness,
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

    pub fn update_points(&mut self, func: fn(i64) -> f64) {
        for i in 0..1920 {
            self.target_points[i] = func(i as i64); // TODO: Make this correct (not i as i64)
        }
    }
}
