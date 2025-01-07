use crate::hvf::hvf::HVF;

use super::interpolator::Interpolator;


pub struct Animator {
    active_ring: HVF
}

impl Animator {
    pub fn new(default_ring: &HVF) -> Self {
        
        Self {
            active_ring: default_ring.clone()
        }
    }

    fn get_path_len(&self, path: &Vec<Vec<f64>>) -> f64 {
        let mut b = path.last().unwrap();
        let mut xb = b[0];
        let mut yb = b[1];
        let mut xa: f64;
        let mut ya: f64;
        let mut path_len = 0.0;

        for i in 0..path.len(){
            xa = xb;
            ya = yb;
            b = &path[i];
            xb = b[0];
            yb = b[1];

            xa -= xb;
            ya -= yb;
            
            // Calculate hypotenuse
            path_len += (xa.powi(2) + ya.powi(2)).sqrt()
        }

        path_len
    }

    fn get_distance(&self, a: &Vec<f64>, b: &Vec<f64>) -> f64 {
        let ax = a[0];
        let ay = a[1];
        let bx = b[0];
        let by = b[1];
        ((ax - bx) * (ax - bx) + (ay - by) * (ay - by)).sqrt()
    }

    fn get_point_along(&self, a:&Vec<f64>, b: &Vec<f64>, pct:f64) -> Vec<f64> {
        let ax = a[0];
        let ay = a[1];
        let bx = b[0];
        let by = b[1];

        let new_point = vec![ax + ((bx - ax) * pct), ay + ((by - ay) * pct)];

        // tracing::info!("New point added: {:?} at pct: {}", new_point, pct);
        new_point
    }

    fn add_points(&mut self, path: &mut Vec<Vec<f64>>, size: usize) {
        let desired_len = path.len() + size;
        let step = self.get_path_len(path) / size as f64;

        let mut i = 0;
        let mut cursor = 0.0;
        let mut insert_at = step / 2.;

        while path.len() < desired_len {
            let a = &path[i];
            let b = &path[(i+1) % path.len()];

            let segment = self.get_distance(a, b);

            if insert_at <= (cursor + segment) {
                if segment != 0.0 {
                    path.insert(i+1, self.get_point_along(a, b, (insert_at - cursor)/segment));
                } else {
                    path.insert(i+1, a.clone());
                }

                insert_at += step;
                continue;
            } 
            cursor += segment;
            i += 1;
        }
    }

    fn normalize(&mut self, current_path: &mut Vec<Vec<f64>>, target_path: &mut Vec<Vec<f64>>) {
        let diff = current_path.len() as i32 - target_path.len() as i32;

        if diff < 0 {
            self.add_points(current_path, (diff * -1) as usize);
        } else if diff > 0 {
            self.add_points(target_path, diff as usize);
        }
    }

    pub fn get_interpolaror(&mut self, target_ring: &HVF) ->  Vec<Interpolator>{
        // TODO: asset if target_ring is compatible!
        let mut collection: Vec<Interpolator> = Vec::new();
        // Loop through each vector inside target_ring and active_ring;
        for (i, path) in target_ring.values().iter().enumerate() {
            // let mut current_path = self.active_ring.values()[i].as_array_mut().unwrap();
            let current_path = &mut self.active_ring.values()[i].clone();
            let target_path = &mut path.clone();

            // normalize them!
            self.normalize(current_path, target_path);
            collection.push(Interpolator::new(current_path.to_owned(), target_path.to_owned()));
        }

        collection
    }
}