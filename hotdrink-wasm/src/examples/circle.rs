//! A circle type and operations on it.

use wasm_bindgen::prelude::wasm_bindgen;

/// A circle with a position and a radius.
#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Circle {
    /// The x-coordinate of the circle.
    pub x: i32,
    /// The y-coordinate of the circle.
    pub y: i32,
    /// The radius of the circle.
    pub r: usize,
}

#[wasm_bindgen]
impl Circle {
    /// Constructs a new `Circle`.
    #[wasm_bindgen(constructor)]
    pub fn new(x: i32, y: i32, r: usize) -> Self {
        Self { x, y, r }
    }
}

impl Circle {
    /// Calculates the distance in x-coordinates between two circles.
    pub fn dx(&self, b: &Circle) -> f64 {
        (b.x - self.x) as f64
    }

    /// Calculates the distance in y-coordinates between two circles.
    pub fn dy(&self, b: &Circle) -> f64 {
        (b.y - self.y) as f64
    }

    /// Calculates the distance between two circles.
    pub fn dist(&self, b: &Circle) -> f64 {
        (self.dx(b).powf(2.0) + self.dy(b).powf(2.0)).sqrt()
    }

    /// Calculates the angle between two circles.
    pub fn angle(&self, b: &Circle) -> f64 {
        self.dy(b).atan2(self.dx(b))
    }

    /// Constructs a circle that does not overlap
    /// with the first circle, similar to moving
    /// the second argument.
    pub fn shift(&self, b: &Circle) -> Circle {
        // Compute angle between circles
        let angle = self.angle(b);

        // Do not do anything if they do not overlap
        let old_dist = self.dist(b);
        let new_dist = (self.r + b.r) as f64;
        if old_dist > new_dist {
            return b.clone();
        }

        // Compute the new position
        let new_x = self.x + (angle.cos() * new_dist) as i32;
        let new_y = self.y + (angle.sin() * new_dist) as i32;

        Circle::new(new_x, new_y, b.r)
    }
}

#[cfg(test)]
mod circle_tests {

    use super::Circle;

    #[test]
    fn non_overlapping_circle_is_not_moved_1() {
        let a = Circle::new(0, 0, 5);
        let b = Circle::new(15, 0, 7);
        let shifted = a.shift(&b);
        assert_eq!(shifted, b);
    }

    #[test]
    fn non_overlapping_circle_is_not_moved_2() {
        let a = Circle::new(-3, -4, 5);
        let b = Circle::new(15, 6, 7);
        let shifted = a.shift(&b);
        assert_eq!(shifted, b);
    }

    #[test]
    fn non_overlapping_circle_is_not_moved_3() {
        let a = Circle::new(13, 1, 5);
        let b = Circle::new(13, 20, 5);
        let shifted = a.shift(&b);
        assert_eq!(shifted, b);
    }

    #[test]
    fn overlapping_circle_to_the_north_is_moved_to_border() {
        let a = Circle::new(0, 0, 5);
        let b = Circle::new(0, 10, 7);
        let shifted = a.shift(&b);
        assert_eq!(shifted, Circle::new(0, 12, 7));
    }

    #[test]
    fn overlapping_circle_to_the_east_is_moved_to_border() {
        let a = Circle::new(0, 0, 5);
        let b = Circle::new(10, 0, 7);
        let shifted = a.shift(&b);
        assert_eq!(shifted, Circle::new(12, 0, 7));
    }

    #[test]
    fn overlapping_circle_to_the_south_is_moved_to_border() {
        let a = Circle::new(0, 0, 5);
        let b = Circle::new(5, -4, 5);
        let shifted = a.shift(&b);
        assert_eq!(shifted, Circle::new(7, -6, 5));
    }

    #[test]
    fn overlapping_circle_to_the_west_is_moved_to_border() {
        let a = Circle::new(3, 0, 5);
        let b = Circle::new(-10, 0, 10);
        let shifted = a.shift(&b);
        assert_eq!(shifted, Circle::new(-12, 0, 10));
    }

    #[test]
    fn contained_circle_is_moved() {
        let a = Circle::new(0, 0, 10);
        let b = Circle::new(1, 0, 5);
        let shifted = a.shift(&b);
        assert_eq!(shifted, Circle::new(15, 0, 5));
    }

    #[test]
    fn containing_circle_is_moved() {
        let a = Circle::new(0, 0, 5);
        let b = Circle::new(1, 0, 10);
        let shifted = a.shift(&b);
        assert_eq!(shifted, Circle::new(15, 0, 10));
    }
}
