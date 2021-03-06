use crate::ray::RT;
use image::Rgb;
use std::ops;

pub(crate) type CT = u8;

#[derive(Clone)]
pub(crate) struct RRgb {
    r: f64,
    g: f64,
    b: f64,
}

impl ops::Add<RRgb> for RRgb {
    type Output = RRgb;

    fn add(self, rhs: RRgb) -> Self::Output {
        RRgb {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl ops::Mul<f32> for RRgb {
    type Output = RRgb;

    fn mul(self, rhs: f32) -> Self::Output {
        RRgb {
            r: (self.r as RT * rhs) as f64,
            g: (self.g as RT * rhs) as f64,
            b: (self.b as RT * rhs) as f64,
        }
    }
}

impl ops::Mul<RRgb> for RRgb {
    type Output = RRgb;

    fn mul(self, rhs: RRgb) -> Self::Output {
        RRgb {
            r: (self.r * rhs.r),
            g: (self.g * rhs.g),
            b: (self.b * rhs.b),
        }
    }
}

impl std::iter::Sum for RRgb {
    fn sum<I: Iterator<Item = RRgb>>(iter: I) -> Self {
        iter.fold(
            RRgb {
                r: 0f64,
                g: 0f64,
                b: 0f64,
            },
            |rc, acc| rc + acc,
        )
    }
}

impl RRgb {
    pub(crate) fn new(r: f64, g: f64, b: f64) -> Self {
        RRgb { r, g, b }
    }
}

impl From<RRgb> for Rgb<CT> {
    fn from(rrgb: RRgb) -> Self {
        let r = if rrgb.r > u8::MAX as f64 {
            u8::MAX
        } else {
            rrgb.r as u8
        };
        let g = if rrgb.g > u8::MAX as f64 {
            u8::MAX
        } else {
            rrgb.g as u8
        };
        let b = if rrgb.b > u8::MAX as f64 {
            u8::MAX
        } else {
            rrgb.b as u8
        };
        Rgb([r, g, b])
    }
}
