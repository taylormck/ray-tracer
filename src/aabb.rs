//! Axis-Aligned Bounding Box
//! This should help speed up the rendering times dramatically

use crate::ray::Ray;
use std::ops::Range;

#[derive(Clone, Debug)]
pub struct AABB {
    x: Range<f64>,
    y: Range<f64>,
    z: Range<f64>,
}

impl AABB {
    pub fn new(x: Range<f64>, y: Range<f64>, z: Range<f64>) -> Self {
        // NOTE: Not sure if this it totally necessary
        let x = match x.start < x.end {
            true => x,
            false => x.end..x.start,
        };

        let y = match y.start < y.end {
            true => y,
            false => y.end..y.start,
        };

        let z = match z.start < z.end {
            true => z,
            false => z.end..z.start,
        };

        Self { x, y, z }
    }

    pub fn new_empty() -> Self {
        Self::new(0.0..0.0, 0.0..0.0, 0.0..0.0)
    }

    pub fn combine_bounds(a: &Self, b: &Self) -> Self {
        Self {
            x: Range {
                start: f64::min(a.x.start, b.x.start),
                end: f64::max(a.x.end, b.x.end),
            },
            y: Range {
                start: f64::min(a.y.start, b.y.start),
                end: f64::max(a.y.end, b.y.end),
            },
            z: Range {
                start: f64::min(a.z.start, b.z.start),
                end: f64::max(a.z.end, b.z.end),
            },
        }
    }

    pub fn hit(self: &Self, ray: &Ray, mut range: Range<f64>) -> bool {
        for (i, axis) in self.axes().enumerate() {
            let adinv: f64 = ray.direction()[i].recip();

            let t0 = (axis.start - ray.origin()[i]) * adinv;
            let t1 = (axis.end - ray.origin()[i]) * adinv;

            if t0 < t1 {
                range.start = f64::max(range.start, t0);
                range.end = f64::min(range.end, t1);
            } else {
                range.start = f64::max(range.start, t1);
                range.end = f64::min(range.end, t0);
            }

            if range.end <= range.start {
                return false;
            }
        }

        true
    }

    pub fn axes(&self) -> AabbAxisIterator<'_> {
        AabbAxisIterator {
            aabb: self,
            index: 0,
        }
    }

    pub fn axis(self: &Self, i: usize) -> Option<Range<f64>> {
        match i {
            0 => Some(self.x.clone()),
            1 => Some(self.y.clone()),
            2 => Some(self.z.clone()),
            _ => None,
        }
    }
}

pub struct AabbAxisIterator<'a> {
    aabb: &'a AABB,
    index: usize,
}

impl<'a> Iterator for AabbAxisIterator<'a> {
    type Item = Range<f64>;

    fn next(&mut self) -> Option<Range<f64>> {
        let result = self.aabb.axis(self.index);
        self.index += 1;
        result
    }
}
