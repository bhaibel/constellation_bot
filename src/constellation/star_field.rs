extern crate rand;

use std::vec::Vec;
use self::rand::{thread_rng, Rng};
use self::rand::distributions::{IndependentSample, Range};

use super::super::BoundingBox;

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

pub struct Star {
    pub x: f32,
    pub y: f32
}

impl Star {
    pub fn new(bounds: &BoundingBox) -> Star {
        let mut rng = thread_rng();

        Star {
            x: rng.gen::<f32>() * bounds.width as f32 + bounds.origin_x as f32,
            y: rng.gen::<f32>() * bounds.height as f32 + bounds.origin_y as f32
        }
    }
}

pub fn stars(bounds: &BoundingBox) -> Vec<Star> {
    let mut rng = thread_rng();
    let count = Range::new(6, 13).ind_sample(&mut rng);
    
    (0..count)
        .map(|_| Star::new(&bounds))
        .collect::<Vec<Star>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::super::BoundingBox;

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
            assert!(subject.iter().all(|p| p.x >= bounds.origin_x as f32));
            assert!(subject.iter().all(|p| p.y >= bounds.origin_y as f32));
            assert!(subject.iter().all(|p| p.x <= bounds.width as f32));
            assert!(subject.iter().all(|p| p.y <= bounds.height as f32));
        }
    }
}
