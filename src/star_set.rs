extern crate rand;

use std::vec::Vec;
use self::rand::{thread_rng, Rng};
use self::rand::distributions::{IndependentSample, Range};

pub struct Point {
    x: f32,
    y: f32
}

impl Point {
    pub fn new() -> Point {
        let mut rng = thread_rng();

        Point {
            x: rng.gen::<f32>() * 100.0,
            y: rng.gen::<f32>() * 100.0
        }
    }
}

pub fn stars() -> Vec<Point> {
    let mut rng = thread_rng();
    let count = Range::new(6, 13).ind_sample(&mut rng);
    
    (0..count)
        .map(|_| Point::new())
        .collect::<Vec<Point>>()
}

#[cfg(test)]
mod tests {
    use super::stars;

    #[test]
    fn count_stars() {
        let subject = stars();

        for _ in 0..100 {
            assert!(6 <= subject.len());
            assert!(13 >= subject.len());
        }
    }

    #[test]
    fn check_star_positions() {
        let subject = stars();

        for _ in 0..100 {
            assert!(subject.iter().all(|p| p.x >= 0.0));
            assert!(subject.iter().all(|p| p.y >= 0.0));
            assert!(subject.iter().all(|p| p.x <= 100.0));
            assert!(subject.iter().all(|p| p.y <= 100.0));
        }
    }
}
