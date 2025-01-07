pub struct Interpolator {
    current_path: Vec<Vec<f64>>,
    target_path: Vec<Vec<f64>>,

    target_size: usize,
}

impl Interpolator {
    pub fn new(current_path: Vec<Vec<f64>>, target_path: Vec<Vec<f64>>) -> Self {
        let target_size = target_path.len();

        Self {
            current_path,
            target_path,
            
            target_size,
        }
    }

    pub fn interpolate(&mut self, t: f64) -> Vec<Vec<f64>> {
        let mut target: Vec<Vec<f64>> = Vec::new();

        // Interpolation logic here
        for i in 0..self.target_size {
            let from_point = &self.current_path[i];
            let to_point = &self.target_path[i];

            let xt = from_point[0] + t * (to_point[0] - from_point[0]);
            let yt = from_point[1] + t * (to_point[1] - from_point[1]);
            target.push(vec![xt, yt]);
        }

        target
    }
}
