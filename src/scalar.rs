extern crate rand;
extern crate num;

use std::ops::{Div, Sub, Mul, Add};

use self::num::FromPrimitive;
use self::rand::thread_rng;
use self::rand::distributions::{IndependentSample, Range};

const MAX: u8 = 255;

pub struct Scalar {
    pub val: u8
}

impl Scalar {
    pub fn rand(num: u8) -> Scalar {
        Scalar { val: roll_some_dice(0, MAX as i32, num) as u8 }
    }

    pub fn scaled<S>(&self, max: S, min: S) -> S
        where S: FromPrimitive + Copy + Add<Output=S> + Mul<Output=S> + Div<Output=S> + Sub<Output=S> {
        let range = max - min;
        let scale_factor = range / FromPrimitive::from_u8(MAX).unwrap();
        scale_factor * FromPrimitive::from_u8(self.val).unwrap() + min
    }
}

pub fn roll_some_dice(min: i32, max: i32, count: u8) -> i32 {
    let mut rng = thread_rng();

    (0..count)
        .map(|_| Range::new(min, max + 1).ind_sample(&mut rng))
        .sum::<i32>()
        / count as i32
}