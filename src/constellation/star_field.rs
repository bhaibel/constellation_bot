extern crate rand;

use std::vec::Vec;
use self::rand::thread_rng;
use self::rand::distributions::{IndependentSample, Range};

use super::super::bounding_box::BoundingBox;

// #[feature(macro_rules)]
// macro_rules! obvious_impl {
//     (impl $trait_: ident for $type_: ident { fn $method: ident }) => {
//         impl $trait_<$type_, $type_> for $type_ {
//             fn $method(&self, &$type_(b): &$type_) -> $type_ {
//                 let $type_(a) = *self;
//                 $type_(a.$method(&b))
//             }
//         }
//     }
// }
// used as:
//    obvious_impl! { impl ExactSizeIterator for StarField { fn len } }
// TODO: use to make a StarField newtype

const MIN_STAR_COUNT: i8 = 6;
const MAX_STAR_COUNT: i8 = 13;
const MIN_STAR_SIZE: i8 = 2;
const MAX_STAR_SIZE: i8 = 3;

pub struct Star {
    pub x: i32,
    pub y: i32,
    pub size: i8
}

impl Star {
    pub fn new(bounds: &BoundingBox) -> Star {
        let mut rng = thread_rng();

        Star {
            x: Range::new(0, bounds.width).ind_sample(&mut rng) + bounds.origin_x,
            y: Range::new(0, bounds.height).ind_sample(&mut rng) + bounds.origin_y,
            size: Range::new(MIN_STAR_SIZE, MAX_STAR_SIZE + 1).ind_sample(&mut rng)
        }
    }
}

pub fn stars(bounds: &BoundingBox) -> Vec<Star> {
    let mut rng = thread_rng();
    let count = Range::new(MIN_STAR_COUNT, MAX_STAR_COUNT + 1).ind_sample(&mut rng);
    
    (0..count)
        .map(|_| Star::new(&bounds))
        .collect::<Vec<Star>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::super::bounding_box::BoundingBox;

    #[test]
    fn count_stars() {
        let bounds = BoundingBox::default();
        let subject = stars(&bounds);

        for _ in 0..100 {
            assert!(6 <= subject.len());
            assert!(13 >= subject.len());
        }
    }

    #[test]
    fn check_star_positions() {
        let bounds = BoundingBox::default();
        let subject = stars(&bounds);

        for _ in 0..100 {
            assert!(subject.iter().all(|p| p.x >= bounds.origin_x));
            assert!(subject.iter().all(|p| p.y >= bounds.origin_y));
            assert!(subject.iter().all(|p| p.x <= bounds.width));
            assert!(subject.iter().all(|p| p.y <= bounds.height));
        }
    }
}
