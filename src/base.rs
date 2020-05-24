/* BSD 3-Clause License
 *
 * Copyright © 2019, Alexander Krivács Schrøder <alexschrod@gmail.com>.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 * 1. Redistributions of source code must retain the above copyright notice,
 *    this list of conditions and the following disclaimer.
 *
 * 2. Redistributions in binary form must reproduce the above copyright notice,
 *    this list of conditions and the following disclaimer in the documentation
 *    and/or other materials provided with the distribution.
 *
 * 3. Neither the name of the copyright holder nor the names of its
 *    contributors may be used to endorse or promote products derived from
 *    this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
 * AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE
 * LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
 * CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
 * SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 * INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 * CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
 * POSSIBILITY OF SUCH DAMAGE.
 */

use std::convert::TryFrom;
use std::num::TryFromIntError;

#[macro_use]
mod def_macro;

define_two_property_arithmetic_struct!(Position, UPosition, FPosition, x, y, ORIGIN, "({}, {})");
define_two_property_arithmetic_struct!(Size, USize, FSize, width, height, ZERO, "{}x{}");

impl FPosition {
    /// Returns a non-floating point position where the decimal parts of the width and height
    /// have been rounded.
    pub fn round(self) -> Position {
        Position::new(self.x.round() as i32, self.y.round() as i32)
    }

    /// Returns a non-floating point position where the decimal parts of the width and height
    /// have been truncated.
    pub fn trunc(self) -> Position {
        Position::new(self.x.trunc() as i32, self.y.trunc() as i32)
    }

    /// Returns a non-floating point position where the decimal parts of the width and height
    /// have been truncated.
    pub fn trunc_u(self) -> UPosition {
        assert!(self.x >= 0.);
        assert!(self.y >= 0.);

        UPosition::new(self.x.trunc() as u32, self.y.trunc() as u32)
    }
}

impl Size {
    /// Returns the area represented by this size
    pub fn area(self) -> i32 {
        self.width * self.height
    }
}

impl USize {
    /// Returns the area represented by this size
    pub fn area(self) -> u32 {
        self.width * self.height
    }
}

impl TryFrom<Size> for USize {
    type Error = TryFromIntError;

    fn try_from(value: Size) -> Result<Self, Self::Error> {
        Ok(Self::new(
            TryFrom::try_from(value.width)?,
            TryFrom::try_from(value.height)?,
        ))
    }
}

impl FSize {
    /// Returns the area represented by this size
    pub fn area(self) -> f32 {
        self.width * self.height
    }
}

/// Represents a rectangle, using a position and size.
#[derive(Copy, Clone, Default, PartialEq, Eq, Debug)]
#[cfg_attr(
    feature = "serialization",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
pub struct Rectangle {
    /// The location of the rectangle's upper-left corner
    pub position: Position,
    /// The width and height of the rectangle
    pub size: USize,
}

impl Rectangle {
    /// Returns a new rectangle with the given position and size
    pub fn new(position: Position, size: USize) -> Self {
        Self { position, size }
    }

    /// Returns a new rectangle with the given raw position and size values
    pub fn new_from_raw(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            position: Position::new(x, y),
            size: USize::new(width, height),
        }
    }

    /// Returns whether a given position is within the rectangle or not
    pub fn contains_position(&self, position: Position) -> bool {
        position.x >= self.position.x
            && position.x <= self.position.x + self.size.width as i32
            && position.y >= self.position.y
            && position.y <= self.position.y + self.size.height as i32
    }

    /// Returns whether a given position is within the rectangle or not
    pub fn contains_fposition(&self, position: FPosition) -> bool {
        position.x >= self.position.x as f32
            && position.x <= self.position.x as f32 + self.size.width as f32
            && position.y >= self.position.y as f32
            && position.y <= self.position.y as f32 + self.size.height as f32
    }
}

/// Represents a floating-point rectangle, using a position and size.
#[derive(Copy, Clone, Default, PartialEq, Debug)]
#[cfg_attr(
    feature = "serialization",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
pub struct FRectangle {
    /// The location of the rectangle's upper-left corner
    pub position: FPosition,
    /// The width and height of the rectangle
    pub size: FSize,
}

impl FRectangle {
    /// Returns a new rectangle with the given position and size
    ///
    /// # Panics
    /// This function may panic if the width or height is < 0.
    pub fn new(position: FPosition, size: FSize) -> Self {
        assert!(size.width >= 0.0);
        assert!(size.height >= 0.0);

        Self { position, size }
    }

    /// Returns a new rectangle with the given raw position and size values
    ///
    /// # Panics
    /// This function may panic if the width or height is < 0.
    pub fn new_from_raw(x: f32, y: f32, width: f32, height: f32) -> Self {
        assert!(width >= 0.0);
        assert!(height >= 0.0);

        Self {
            position: FPosition::new(x, y),
            size: FSize::new(width, height),
        }
    }

    /// Returns whether a given position is within the rectangle or not
    pub fn contains_position(&self, position: FPosition) -> bool {
        position.x >= self.position.x
            && position.x <= self.position.x + self.size.width
            && position.y >= self.position.y
            && position.y <= self.position.y + self.size.height
    }
}

impl std::ops::Add<USize> for Position {
    type Output = Rectangle;

    fn add(self, rhs: USize) -> Self::Output {
        Rectangle::new(self, rhs)
    }
}

impl std::ops::Add<FSize> for FPosition {
    type Output = FRectangle;

    fn add(self, rhs: FSize) -> Self::Output {
        FRectangle::new(self, rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::float_cmp)]
    fn new_sets_values() {
        let p = Position::new(-1, -2);
        assert_eq!(p.x, -1);
        assert_eq!(p.y, -2);

        let up = UPosition::new(1, 2);
        assert_eq!(up.x, 1);
        assert_eq!(up.y, 2);

        let fp = FPosition::new(1., -2.);
        assert_eq!(fp.x, 1.);
        assert_eq!(fp.y, -2.);

        let s = Size::new(-1, -2);
        assert_eq!(s.width, -1);
        assert_eq!(s.height, -2);

        let us = USize::new(1, 2);
        assert_eq!(us.width, 1);
        assert_eq!(us.height, 2);

        let fs = FSize::new(1., -2.);
        assert_eq!(fs.width, 1.);
        assert_eq!(fs.height, -2.);

        let r = Rectangle::new(p, us);
        assert_eq!(r.position, p);
        assert_eq!(r.size, us);

        let r2 = Rectangle::new_from_raw(1, 2, 3, 4);
        assert_eq!(r2.position, Position::new(1, 2));
        assert_eq!(r2.size, USize::new(3, 4));
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn from_sets_values() {
        let p: Position = From::from((-1, -2));
        assert_eq!(p.x, -1);
        assert_eq!(p.y, -2);

        let up: UPosition = From::from((1, 2));
        assert_eq!(up.x, 1);
        assert_eq!(up.y, 2);

        let fp: FPosition = From::from((1., -2.));
        assert_eq!(fp.x, 1.);
        assert_eq!(fp.y, -2.);

        let s: Size = From::from((-1, -2));
        assert_eq!(s.width, -1);
        assert_eq!(s.height, -2);

        let us: USize = From::from((1, 2));
        assert_eq!(us.width, 1);
        assert_eq!(us.height, 2);

        let fs: FSize = From::from((1., -2.));
        assert_eq!(fs.width, 1.);
        assert_eq!(fs.height, -2.);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn from_gets_values() {
        let p = Position::new(-1, -2);
        let (px, py): (i32, i32) = From::from(p);
        assert_eq!(px, -1);
        assert_eq!(py, -2);

        let up = UPosition::new(1, 2);
        let (upx, upy): (u32, u32) = From::from(up);
        assert_eq!(upx, 1);
        assert_eq!(upy, 2);

        let fp = FPosition::new(1., -2.);
        let (fpx, fpy): (f32, f32) = From::from(fp);
        assert_eq!(fpx, 1.);
        assert_eq!(fpy, -2.);

        let s = Size::new(-1, -2);
        let (swidth, sheight): (i32, i32) = From::from(s);
        assert_eq!(swidth, -1);
        assert_eq!(sheight, -2);

        let us = USize::new(1, 2);
        let (uswidth, usheight): (u32, u32) = From::from(us);
        assert_eq!(uswidth, 1);
        assert_eq!(usheight, 2);

        let fs = FSize::new(1., -2.);
        let (fswidth, fsheight): (f32, f32) = From::from(fs);
        assert_eq!(fswidth, 1.);
        assert_eq!(fsheight, -2.);
    }

    #[test]
    fn display_is_correct() {
        let p = Position::new(-1, -2);
        assert_eq!(p.to_string(), "(-1, -2)");

        let up = UPosition::new(1, 2);
        assert_eq!(up.to_string(), "(1, 2)");

        let fp = FPosition::new(1.5, -2.7);
        assert_eq!(fp.to_string(), "(1.5, -2.7)");

        let s = Size::new(-1, -2);
        assert_eq!(s.to_string(), "-1x-2");

        let us = USize::new(1, 2);
        assert_eq!(us.to_string(), "1x2");

        let fs = FSize::new(1.5, -2.7);
        assert_eq!(fs.to_string(), "1.5x-2.7");
    }

    #[test]
    fn addition() {
        let p = Position::new(-1, -2);
        let p2 = Position::new(3, 4);
        assert_eq!(p + p2, Position::new(2, 2));

        let up = UPosition::new(1, 2);
        let up2 = UPosition::new(3, 4);
        assert_eq!(up + up2, UPosition::new(4, 6));

        let fp = FPosition::new(-1.5, -3.0);
        let fp2 = FPosition::new(4.5, 6.0);
        assert_eq!(fp + fp2, FPosition::new(3., 3.));

        let s = Size::new(-1, -2);
        let s2 = Size::new(3, 4);
        assert_eq!(s + s2, Size::new(2, 2));

        let us = USize::new(1, 2);
        let us2 = USize::new(3, 4);
        assert_eq!(us + us2, USize::new(4, 6));

        let fs = FSize::new(-1.5, -3.0);
        let fs2 = FSize::new(4.5, 6.0);
        assert_eq!(fs + fs2, FSize::new(3., 3.));

        assert_eq!(p + us, Rectangle::new(p, us));
        assert_eq!(fp + fs2, FRectangle::new(fp, fs2));
    }

    #[test]
    fn addition_scalar() {
        let p = Position::new(-1, -2);
        let p2 = 2;
        assert_eq!(p + p2, Position::new(1, 0));

        let up = UPosition::new(1, 2);
        let up2 = 3;
        assert_eq!(up + up2, UPosition::new(4, 5));

        let fp = FPosition::new(-1.5, -3.0);
        let fp2 = 1.5;
        assert_eq!(fp + fp2, FPosition::new(0., -1.5));

        let s = Size::new(-1, -2);
        let s2 = 2;
        assert_eq!(s + s2, Size::new(1, 0));

        let us = USize::new(1, 2);
        let us2 = 3;
        assert_eq!(us + us2, USize::new(4, 5));

        let fs = FSize::new(-1.5, -3.0);
        let fs2 = 1.5;
        assert_eq!(fs + fs2, FSize::new(0., -1.5));
    }

    #[test]
    fn addition_tuple() {
        let p = Position::new(-1, -2);
        let p2 = (3, 4);
        assert_eq!(p + p2, Position::new(2, 2));

        let up = UPosition::new(1, 2);
        let up2 = (3, 4);
        assert_eq!(up + up2, UPosition::new(4, 6));

        let fp = FPosition::new(-1.5, -3.0);
        let fp2 = (4.5, 6.0);
        assert_eq!(fp + fp2, FPosition::new(3., 3.));

        let s = Size::new(-1, -2);
        let s2 = (3, 4);
        assert_eq!(s + s2, Size::new(2, 2));

        let us = USize::new(1, 2);
        let us2 = (3, 4);
        assert_eq!(us + us2, USize::new(4, 6));

        let fs = FSize::new(-1.5, -3.0);
        let fs2 = (4.5, 6.0);
        assert_eq!(fs + fs2, FSize::new(3., 3.));
    }

    #[test]
    fn add_assign_scalar() {
        let mut p = Position::new(-1, -2);
        p += 2;
        assert_eq!(p, Position::new(1, 0));

        let mut up = UPosition::new(1, 2);
        up += 3;
        assert_eq!(up, UPosition::new(4, 5));

        let mut fp = FPosition::new(-1.5, -3.0);
        fp += 1.5;
        assert_eq!(fp, FPosition::new(0., -1.5));

        let mut s = Size::new(-1, -2);
        s += 2;
        assert_eq!(s, Size::new(1, 0));

        let mut us = USize::new(1, 2);
        us += 3;
        assert_eq!(us, USize::new(4, 5));

        let mut fs = FSize::new(-1.5, -3.0);
        fs += 1.5;
        assert_eq!(fs, FSize::new(0., -1.5));
    }

    #[test]
    fn add_assign_tuple() {
        let mut p = Position::new(-1, -2);
        p += (3, 4);
        assert_eq!(p, Position::new(2, 2));

        let mut up = UPosition::new(1, 2);
        up += (3, 4);
        assert_eq!(up, UPosition::new(4, 6));

        let mut fp = FPosition::new(-1.5, -3.0);
        fp += (4.5, 6.0);
        assert_eq!(fp, FPosition::new(3., 3.));

        let mut s = Size::new(-1, -2);
        s += (3, 4);
        assert_eq!(s, Size::new(2, 2));

        let mut us = USize::new(1, 2);
        us += (3, 4);
        assert_eq!(us, USize::new(4, 6));

        let mut fs = FSize::new(-1.5, -3.0);
        fs += (4.5, 6.0);
        assert_eq!(fs, FSize::new(3., 3.));
    }

    #[test]
    fn subtraction() {
        let p = Position::new(-1, -2);
        let p2 = Position::new(3, 4);
        assert_eq!(p - p2, Position::new(-4, -6));

        let up = UPosition::new(3, 4);
        let up2 = UPosition::new(1, 2);
        assert_eq!(up - up2, UPosition::new(2, 2));

        let fp = FPosition::new(-1.5, -3.0);
        let fp2 = FPosition::new(4.5, 6.0);
        assert_eq!(fp - fp2, FPosition::new(-6., -9.));

        let s = Size::new(-1, -2);
        let s2 = Size::new(3, 4);
        assert_eq!(s - s2, Size::new(-4, -6));

        let us = USize::new(3, 4);
        let us2 = USize::new(1, 2);
        assert_eq!(us - us2, USize::new(2, 2));

        let fs = FSize::new(-1.5, -3.0);
        let fs2 = FSize::new(4.5, 6.0);
        assert_eq!(fs - fs2, FSize::new(-6., -9.));
    }

    #[test]
    fn subtraction_scalar() {
        let mut p = Position::new(-1, -2);
        p -= 2;
        assert_eq!(p, Position::new(-3, -4));

        let mut up = UPosition::new(1, 2);
        up -= 1;
        assert_eq!(up, UPosition::new(0, 1));

        let mut fp = FPosition::new(-1.5, -3.0);
        fp -= 1.5;
        assert_eq!(fp, FPosition::new(-3., -4.5));

        let mut s = Size::new(-1, -2);
        s -= 2;
        assert_eq!(s, Size::new(-3, -4));

        let mut us = USize::new(1, 2);
        us -= 1;
        assert_eq!(us, USize::new(0, 1));

        let mut fs = FSize::new(-1.5, -3.0);
        fs -= 1.5;
        assert_eq!(fs, FSize::new(-3., -4.5));
    }

    #[test]
    fn subtraction_tuple() {
        let p = Position::new(-1, -2);
        let p2 = (3, 4);
        assert_eq!(p - p2, Position::new(-4, -6));

        let up = UPosition::new(3, 4);
        let up2 = (1, 2);
        assert_eq!(up - up2, UPosition::new(2, 2));

        let fp = FPosition::new(-1.5, -3.0);
        let fp2 = (4.5, 6.0);
        assert_eq!(fp - fp2, FPosition::new(-6., -9.));

        let s = Size::new(-1, -2);
        let s2 = (3, 4);
        assert_eq!(s - s2, Size::new(-4, -6));

        let us = USize::new(3, 4);
        let us2 = (1, 2);
        assert_eq!(us - us2, USize::new(2, 2));

        let fs = FSize::new(-1.5, -3.0);
        let fs2 = (4.5, 6.0);
        assert_eq!(fs - fs2, FSize::new(-6., -9.));
    }

    #[test]
    fn sub_assign_scalar() {
        let mut p = Position::new(-1, -2);
        p -= 2;
        assert_eq!(p, Position::new(-3, -4));

        let mut up = UPosition::new(6, 3);
        up -= 3;
        assert_eq!(up, UPosition::new(3, 0));

        let mut fp = FPosition::new(-1.5, -3.0);
        fp -= 1.5;
        assert_eq!(fp, FPosition::new(-3.0, -4.5));

        let mut s = Size::new(-1, -2);
        s -= 2;
        assert_eq!(s, Size::new(-3, -4));

        let mut us = USize::new(6, 3);
        us -= 3;
        assert_eq!(us, USize::new(3, 0));

        let mut fs = FSize::new(-1.5, -3.0);
        fs -= 1.5;
        assert_eq!(fs, FSize::new(-3., -4.5));
    }

    #[test]
    fn sub_assign_tuple() {
        let mut p = Position::new(-1, -2);
        p -= (3, 4);
        assert_eq!(p, Position::new(-4, -6));

        let mut up = UPosition::new(3, 4);
        up -= (1, 2);
        assert_eq!(up, UPosition::new(2, 2));

        let mut fp = FPosition::new(-1.5, -3.0);
        fp -= (4.5, 6.0);
        assert_eq!(fp, FPosition::new(-6., -9.));

        let mut s = Size::new(-1, -2);
        s -= (3, 4);
        assert_eq!(s, Size::new(-4, -6));

        let mut us = USize::new(3, 4);
        us -= (1, 2);
        assert_eq!(us, USize::new(2, 2));

        let mut fs = FSize::new(-1.5, -3.0);
        fs -= (4.5, 6.0);
        assert_eq!(fs, FSize::new(-6., -9.));
    }

    #[test]
    fn multiplication_scalar() {
        let p = Position::new(-1, -2);
        let p2 = 2;
        assert_eq!(p * p2, Position::new(-2, -4));

        let up = UPosition::new(1, 2);
        let up2 = 3;
        assert_eq!(up * up2, UPosition::new(3, 6));

        let fp = FPosition::new(-1.5, -3.0);
        let fp2 = 1.5;
        assert_eq!(fp * fp2, FPosition::new(-2.25, -4.5));

        let s = Size::new(-1, -2);
        let s2 = 2;
        assert_eq!(s * s2, Size::new(-2, -4));

        let us = USize::new(1, 2);
        let us2 = 3;
        assert_eq!(us * us2, USize::new(3, 6));

        let fs = FSize::new(-1.5, -3.0);
        let fs2 = 1.5;
        assert_eq!(fs * fs2, FSize::new(-2.25, -4.5));
    }

    #[test]
    fn mul_assign_scalar() {
        let mut p = Position::new(-1, -2);
        p *= 2;
        assert_eq!(p, Position::new(-2, -4));

        let mut up = UPosition::new(6, 3);
        up *= 3;
        assert_eq!(up, UPosition::new(18, 9));

        let mut fp = FPosition::new(-1.5, -3.0);
        fp *= 1.5;
        assert_eq!(fp, FPosition::new(-2.25, -4.5));

        let mut s = Size::new(-1, -2);
        s *= 2;
        assert_eq!(s, Size::new(-2, -4));

        let mut us = USize::new(6, 3);
        us *= 3;
        assert_eq!(us, USize::new(18, 9));

        let mut fs = FSize::new(-1.5, -3.0);
        fs *= 1.5;
        assert_eq!(fs, FSize::new(-2.25, -4.5));
    }

    #[test]
    fn division_scalar() {
        let p = Position::new(-2, -4);
        let p2 = 2;
        assert_eq!(p / p2, Position::new(-1, -2));

        let up = UPosition::new(18, 9);
        let up2 = 3;
        assert_eq!(up / up2, UPosition::new(6, 3));

        let fp = FPosition::new(-1.5, -3.0);
        let fp2 = 1.5;
        assert_eq!(fp / fp2, FPosition::new(-1., -2.));

        let s = Size::new(-2, -4);
        let s2 = 2;
        assert_eq!(s / s2, Size::new(-1, -2));

        let us = USize::new(18, 9);
        let us2 = 3;
        assert_eq!(us / us2, USize::new(6, 3));

        let fs = FSize::new(-1.5, -3.0);
        let fs2 = 1.5;
        assert_eq!(fs / fs2, FSize::new(-1., -2.));
    }

    #[test]
    fn div_assign_scalar() {
        let mut p = Position::new(-2, -4);
        p /= 2;
        assert_eq!(p, Position::new(-1, -2));

        let mut up = UPosition::new(6, 3);
        up /= 3;
        assert_eq!(up, UPosition::new(2, 1));

        let mut fp = FPosition::new(-1.5, -3.0);
        fp /= 1.5;
        assert_eq!(fp, FPosition::new(-1., -2.));

        let mut s = Size::new(-2, -4);
        s /= 2;
        assert_eq!(s, Size::new(-1, -2));

        let mut us = USize::new(6, 3);
        us /= 3;
        assert_eq!(us, USize::new(2, 1));

        let mut fs = FSize::new(-1.5, -3.0);
        fs /= 1.5;
        assert_eq!(fs, FSize::new(-1., -2.));
    }

    #[test]
    fn rem_scalar() {
        let p = Position::new(-5, -6);
        let p2 = 4;
        assert_eq!(p % p2, Position::new(-1, -2));

        let up = UPosition::new(18, 9);
        let up2 = 4;
        assert_eq!(up % up2, UPosition::new(2, 1));

        let fp = FPosition::new(-2., -4.);
        let fp2 = 1.5;
        assert_eq!(fp % fp2, FPosition::new(-0.5, -1.));

        let s = Size::new(-2, -4);
        let s2 = 2;
        assert_eq!(s % s2, Size::new(0, 0));

        let us = USize::new(18, 9);
        let us2 = 5;
        assert_eq!(us % us2, USize::new(3, 4));

        let fs = FSize::new(-5., -7.);
        let fs2 = 1.5;
        assert_eq!(fs % fs2, FSize::new(-0.5, -1.));
    }

    #[test]
    fn rem_assign_scalar() {
        let mut p = Position::new(-3, -5);
        p %= 2;
        assert_eq!(p, Position::new(-1, -1));

        let mut up = UPosition::new(6, 3);
        up %= 4;
        assert_eq!(up, UPosition::new(2, 3));

        let mut fp = FPosition::new(-5.5, -7.0);
        fp %= 1.5;
        assert_eq!(fp, FPosition::new(-1., -1.));

        let mut s = Size::new(-2, -4);
        s %= 2;
        assert_eq!(s, Size::new(0, 0));

        let mut us = USize::new(16, 13);
        us %= 7;
        assert_eq!(us, USize::new(2, 6));

        let mut fs = FSize::new(-3.5, -3.0);
        fs %= 1.5;
        assert_eq!(fs, FSize::new(-0.5, -0.));
    }

    #[test]
    fn negate() {
        let p = Position::new(-5, -6);
        assert_eq!(-p, Position::new(5, 6));

        let fp = FPosition::new(-2., -4.);
        assert_eq!(-fp, FPosition::new(2., 4.));

        let s = Size::new(-2, -4);
        assert_eq!(-s, Size::new(2, 4));

        let fs = FSize::new(-5., -7.);
        assert_eq!(-fs, FSize::new(5., 7.));
    }

    #[test]
    fn round() {
        let fp = FPosition::new(-2.5, 2.5);
        assert_eq!(fp.round(), Position::new(-3, 3));
    }

    #[test]
    fn trunc() {
        let fp = FPosition::new(-2.5, 2.5);
        assert_eq!(fp.trunc(), Position::new(-2, 2));
    }

    #[test]
    fn trunc_u() {
        let fp = FPosition::new(3.5, 2.5);
        assert_eq!(fp.trunc_u(), UPosition::new(3, 2));
    }

    #[test]
    #[should_panic]
    fn trunc_u_less_than_zero_panics() {
        let fp = FPosition::new(-3.5, 2.5);
        fp.trunc_u();
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn area() {
        let s = Size::new(3, 2);
        assert_eq!(s.area(), 6);

        let us = USize::new(3, 2);
        assert_eq!(us.area(), 6);

        let fs = FSize::new(3.5, 2.5);
        assert_eq!(fs.area(), 8.75);
    }

    #[test]
    fn contains_position() {
        let r = Rectangle::new_from_raw(-5, -10, 10, 20);
        let fr = FRectangle::new_from_raw(-5., -10., 10., 20.);

        // All (integer) points inside, including the corners and along the edges.
        for x in -5..=5 {
            for y in -10..=10 {
                assert!(r.contains_position(Position::new(x, y)));
                assert!(r.contains_fposition(FPosition::new(x as f32, y as f32)));
                assert!(fr.contains_position(FPosition::new(x as f32, y as f32)));
            }
        }

        // All points along the outside
        for &x in &[-6, 6] {
            for y in -11..=11 {
                assert!(!r.contains_position(Position::new(x, y)));
                assert!(!r.contains_fposition(FPosition::new(x as f32, y as f32)));
                assert!(!fr.contains_position(FPosition::new(x as f32, y as f32)));
            }
        }
        for x in -6..=6 {
            for &y in &[-11, 11] {
                assert!(!r.contains_position(Position::new(x, y)));
                assert!(!r.contains_fposition(FPosition::new(x as f32, y as f32)));
                assert!(!fr.contains_position(FPosition::new(x as f32, y as f32)));
            }
        }
    }
}
