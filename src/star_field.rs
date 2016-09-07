extern crate rand;

use std::vec::Vec;
use self::rand::{thread_rng, Rng};
use self::rand::distributions::{IndependentSample, Range};

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
    x: f32,
    y: f32
}

impl Star {
    pub fn new() -> Star {
        let mut rng = thread_rng();

        Star {
            x: rng.gen::<f32>() * 100.0,
            y: rng.gen::<f32>() * 100.0
        }
    }
}

pub fn stars() -> Vec<Star> {
    let mut rng = thread_rng();
    let count = Range::new(6, 13).ind_sample(&mut rng);
    
    (0..count)
        .map(|_| Star::new())
        .collect::<Vec<Star>>()
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
