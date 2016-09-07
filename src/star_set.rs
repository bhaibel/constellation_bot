extern crate rand;

use std::vec::Vec;
use self::rand::{thread_rng, sample};
use self::rand::distributions::{IndependentSample, Range};

pub struct Point {
    x: f32,
    y: f32
}

pub fn stars() -> Vec<Point> {
    let mut rng = thread_rng();
    let count = Range::new(6, 13).ind_sample(&mut rng);
    
    (0..count)
        .map(|_| Point { x: 0.0, y: 0.0 })
        .collect::<Vec<Point>>()
}

#[cfg(test)]
mod tests {
    use super::stars;
    use super::Point;

    #[test]
    fn test_star_properties() {
        for _ in 0..100 {
            let subject = stars();

            count_stars(subject);
        }
    }

    fn count_stars(subject: Vec<Point>) {
        assert!(6 <= subject.len());
        assert!(13 >= subject.len());
    }
}
