use ggez::graphics::Color;


pub struct Graph {
    current_points: Vec<f64>,
    target_points: Vec<f64>,
    pub color: Color,
}


impl Graph {
    pub fn new(elem_count: usize, color: Color) -> Self {
        Self {
            current_points: vec![0.0; elem_count],
            target_points: vec![0.0; elem_count],
            color
        }
    }

    pub fn get_points(&self) -> &[f64] {
        return self.current_points.as_slice();
    }

    pub fn lerp_points(&mut self) {
        for i in 0..self.current_points.len() {
            let x = self.target_points[i] - self.current_points[i];
            self.current_points[i] += x / 1.0;
        }
    }

    pub fn update_func(&mut self, x_origin: f64, unit_length: f64, func: fn(f64) -> f64) {
        for i in 0..self.current_points.len() {
            let x = func((i as f64 - x_origin) / unit_length);
            self.target_points[i] = if x.is_nan() { 0.0 } else { x };
        }
    }
}
