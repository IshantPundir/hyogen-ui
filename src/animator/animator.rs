use core::f64;
use std::time::{Duration, Instant};

use num_integer::gcd;

use crate::hvf::hvf::HVF;
use super::interpolator::Interpolator;

pub struct Animator {
    active_ring: HVF,                          // Current active ring (state of animation)
    active_interpolators: Vec<Interpolator>,   // Collection of interpolators for animation

    interpolate_instance: Instant,             // Time animation started
    interpolate_duration: Duration             // Total duration of the animation
}

impl Animator {
    pub fn new(default_ring: &HVF) -> Self {
        // Initialize animator with a default ring
        Self {
            active_ring: default_ring.clone(),
            active_interpolators: Vec::new(),
            interpolate_instance: Instant::now(),
            interpolate_duration: Duration::from_secs_f64(0.0),
        }
    }

    fn get_path_len(&self, path: &Vec<Vec<f64>>) -> f64 {
        // Calculate the total length of a closed path
        debug_assert!(path.len() > 1, "Path must contain at least two points");
        let mut path_len = 0.0;
        let mut prev = path.last().unwrap(); // Wraps to last point for closure
        for point in path {
            let dx = prev[0] - point[0];
            let dy = prev[1] - point[1];
            path_len += (dx * dx + dy * dy).sqrt();
            prev = point;
        }
        path_len
    }

    fn get_distance(&self, a: &Vec<f64>, b: &Vec<f64>) -> f64 {
        // Calculate the Euclidean distance between two points
        debug_assert!(a.len() == 2 && b.len() == 2, "Points must be 2D vectors");
        if a == b { return 0.0; } // Early return if identical
        ((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2)).sqrt()
    }

    fn get_point_along(&self, a: &Vec<f64>, b: &Vec<f64>, pct: f64) -> Vec<f64> {
        // Get a point along a segment at a given percentage
        let pct = pct.clamp(0.0, 1.0);
        if pct == 0.0 { return a.clone(); }
        if pct == 1.0 { return b.clone(); }
        vec![a[0] + (b[0] - a[0]) * pct, a[1] + (b[1] - a[1]) * pct]
    }

    fn add_points(&mut self, path: &mut Vec<Vec<f64>>, size: usize) {
        // Add evenly spaced points to match the desired path size
        let total_len = self.get_path_len(path);
        let step = total_len / size as f64;
        let mut new_path = Vec::with_capacity(path.len() + size);
        let mut cursor = 0.0;
        let mut insert_at = step / 2.0;

        let mut i = 0;
        while new_path.len() < path.len() + size {
            let a = &path[i];
            let b = &path[(i + 1) % path.len()];
            let segment = self.get_distance(a, b);

            if insert_at <= cursor + segment {
                if segment != 0.0 {
                    let t = (insert_at - cursor) / segment;
                    new_path.push(self.get_point_along(a, b, t));
                } else {
                    new_path.push(a.clone());
                }
                insert_at += step;
            } else {
                new_path.push(a.clone());
                cursor += segment;
                i = (i + 1) % path.len();
            }
        }

        *path = new_path;
    }

    fn offset_path(&mut self, path: &mut Vec<Vec<f64>>, path_len: usize, offset_val: usize) {
        // Offset the path cyclically by a given amount
        let offset_val = offset_val % path_len;
        if offset_val == 0 { return; } // No rotation needed
        let g_c_d = gcd(offset_val, path_len);

        for i in 0..g_c_d {
            let temp = std::mem::take(&mut path[i]);
            let mut j = i;
            loop {
                let k = (j + offset_val) % path_len;
                if k == i { break; }
                path[j] = std::mem::replace(&mut path[k], Vec::new());
                j = k;
            }
            path[j] = temp;
        }
    }

    fn rotate(&mut self, path: &mut Vec<Vec<f64>>, vs: &[Vec<f64>]) {
        // Align the path with the target using the smallest squared distance
        let path_len = vs.len();
        let mut min_val = f64::INFINITY;
        let mut best_offset = 0;

        for offset in 0..path_len {
            let sum_of_square: f64 = (0..path_len)
                .map(|i| {
                    let d = self.get_distance(&path[(offset + i) % path_len], &vs[i]);
                    d * d
                })
                .sum();

            if sum_of_square < min_val {
                min_val = sum_of_square;
                best_offset = offset;
                if min_val == 0.0 { break; } // Early exit for perfect match
            }
        }

        if best_offset != 0 {
            self.offset_path(path, path_len, best_offset);
        }
    }

    fn normalize(&mut self, current_path: &mut Vec<Vec<f64>>, target_path: &mut Vec<Vec<f64>>) {
        // Normalize two paths to have the same number of points and alignment
        let diff = current_path.len() as i32 - target_path.len() as i32;

        if diff < 0 {
            self.add_points(current_path, (-diff) as usize);
        } else if diff > 0 {
            self.add_points(target_path, diff as usize);
        }

        self.rotate(current_path, target_path);
    }

    fn get_interpolaror(&mut self, target_ring: &HVF) -> Vec<Interpolator> {
        // Create interpolators for animating paths
        let mut collection = Vec::new();
        for (i, path) in target_ring.values().iter().enumerate() {
            let current_path = &mut self.active_ring.values()[i].clone();
            let target_path = &mut path.clone();
            self.normalize(current_path, target_path);
            collection.push(Interpolator::new(current_path.to_owned(), target_path.to_owned()));
        }
        collection
    }

    pub fn animate(&mut self, target_ring: &HVF, duration: Duration) {
        // Initialize animation with a target ring and duration
        self.interpolate_instance = Instant::now();
        self.interpolate_duration = duration;
        self.active_interpolators = self.get_interpolaror(target_ring);
    }

    pub fn get_path(&mut self, time: Instant) -> Vec<Vec<Vec<f64>>> {
        // Get the current animation state based on elapsed time
        let mut paths = Vec::new();
        let elapsed = time.duration_since(self.interpolate_instance);
        let interpolate_value = (elapsed.as_secs_f64() / self.interpolate_duration.as_secs_f64()) % 1.0;

        for interpolator in self.active_interpolators.iter_mut() {
            paths.push(interpolator.interpolate(interpolate_value));
        }

        paths
    }
}
