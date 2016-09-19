extern crate rand;

use std::vec::Vec;

use super::super::bounding_box::BoundingBox;
use super::super::color::{Rgb, Hsl};
use super::super::scalar::{Scalar, roll_some_dice};

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

const MIN_STAR_COUNT: i32 = 6;
const MAX_STAR_COUNT: i32 = 12;

const RED_MIN: i32 = 0;
const YELLOW_MAX: i32 = 60;
const BLUE_MIN: i32 = 205;
const BLUE_MAX: i32 = 220;

pub struct Star {
    pub x: i32,
    pub y: i32,
    pub size: Scalar,
    pub color: Rgb
}

impl Star {
    pub fn new(bounds: &BoundingBox) -> Star {
        Star {
            x: roll_some_dice(0, bounds.width, 1) + bounds.origin_x,
            y: roll_some_dice(0, bounds.height, 1) + bounds.origin_y,
            size: Scalar::rand(2),
            color: star_color().as_rgb()
        }
    }
}

pub fn stars(bounds: &BoundingBox) -> Vec<Star> {
    let count = roll_some_dice(MIN_STAR_COUNT, MAX_STAR_COUNT + 1, 1);
    
    (0..count)
        .map(|_| Star::new(&bounds))
        .collect::<Vec<Star>>()
}

fn star_color() -> Hsl {
    let hue_base = roll_some_dice(RED_MIN, YELLOW_MAX + BLUE_MAX - BLUE_MIN, 1);
    let hue = if hue_base < YELLOW_MAX {
                hue_base
            } else {
                hue_base - YELLOW_MAX + BLUE_MIN
            };

    Hsl {
        h: hue as u16,
        s: roll_some_dice(95, 101, 2) as u8,
        l: roll_some_dice(85, 101, 2) as u8
    }
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
