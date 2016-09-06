use std::vec::Vec;

pub struct Point {
  x: f32,
  y: f32
}

pub fn stars(count: i32) -> Vec<Point> {
  let mut stars = Vec::new();
  for x in 0..(count) {
    stars.push(Point { x: 0.0, y: 0.0 });
  }
  stars
}

#[cfg(test)]
mod tests {
    use super::stars;

    #[test]
    fn count_stars() {
        assert_eq!(4, stars(4).len());
    }
}
