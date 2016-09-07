use std::vec::Vec;

pub mod star_field;

use self::star_field::{stars, Star};
use super::BoundingBox;

pub struct Constellation {
  pub stars: Vec<Star>
}

impl Constellation {
  pub fn new(bounds: &BoundingBox) -> Constellation {
    Constellation {
      stars: stars(bounds)
    }
  }
}