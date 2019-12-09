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
    pub fn truncate(self) -> Position {
        Position::new(self.x.trunc() as i32, self.y.trunc() as i32)
    }

    /// Returns a non-floating point position where the decimal parts of the width and height
    /// have been truncated.
    pub fn truncate_u(self) -> UPosition {
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

impl FSize {
    /// Returns the area represented by this size
    pub fn area(self) -> f32 {
        self.width * self.height
    }
}

/// Represents a rectangle, using a position and size.
#[derive(Copy, Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "debug", derive(Debug))]
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
#[derive(Copy, Clone, Default, PartialEq)]
#[cfg_attr(feature = "debug", derive(Debug))]
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
    pub fn new(position: FPosition, size: FSize) -> Self {
        assert!(size.width >= 0.0);
        assert!(size.height >= 0.0);

        Self { position, size }
    }

    /// Returns a new rectangle with the given raw position and size values
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

impl std::ops::Add<Size> for Position {
    type Output = Rectangle;

    fn add(self, rhs: Size) -> Self::Output {
        Rectangle::new(self, USize::new(rhs.width as u32, rhs.height as u32))
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
