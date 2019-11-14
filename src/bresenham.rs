/* BSD 3-Clause License
 *
 * Copyright © 2019, Alexander Krivács Schrøder <alexschrod@gmail.com>.
 * Copyright © 2008-2019, Jice and the libtcod contributors.
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

use crate::base::Position;

/// A struct used for computing a bresenham line.
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct Bresenham {
    step_x: i32,
    step_y: i32,
    e: i32,
    delta_x: i32,
    delta_y: i32,
    orig: Position,
    dest: Position,
}

impl Bresenham {
    /// Initialize a Bresenham struct
    ///
    /// # Parameters
    /// * `from` - The starting position.
    /// * `to` - The ending position.
    pub fn init(from: Position, to: Position) -> Self {
        let mut delta_x = to.x - from.x;
        let step_x = if delta_x > 0 {
            1
        } else if delta_x < 0 {
            -1
        } else {
            0
        };

        let mut delta_y = to.y - from.y;
        let step_y = if delta_y > 0 {
            1
        } else if delta_y < 0 {
            -1
        } else {
            0
        };

        let e = if step_x * delta_x > step_y * delta_y {
            step_x * delta_x
        } else {
            step_y * delta_y
        };
        delta_x *= 2;
        delta_y *= 2;

        Self {
            orig: from,
            dest: to,
            delta_x,
            delta_y,
            step_x,
            step_y,
            e,
        }
    }

    /// Get the next point on a line, returns `None` once the line has ended.
    ///
    /// The starting point is excluded by this function.
    /// After the ending point is reached, the next call will return `None`.
    pub fn step(&mut self) -> Option<Position> {
        if self.step_x * self.delta_x > self.step_y * self.delta_y {
            if self.orig.x == self.dest.x {
                return None;
            }
            self.orig.x += self.step_x;
            self.e -= self.step_y * self.delta_y;
            if self.e < 0 {
                self.orig.y += self.step_y;
                self.e += self.step_x * self.delta_x;
            }
        } else {
            if self.orig.y == self.dest.y {
                return None;
            }
            self.orig.y += self.step_y;
            self.e -= self.step_x * self.delta_x;
            if self.e < 0 {
                self.orig.x += self.step_x;
                self.e += self.step_y * self.delta_y;
            }
        }

        Some(self.orig)
    }
}

impl Iterator for Bresenham {
    type Item = Position;

    /// Does the same as calling `step()`, but lets you access the points by iterating
    fn next(&mut self) -> Option<Self::Item> {
        self.step()
    }
}

#[cfg(test)]
mod tests {
    use crate::base::Position;
    use crate::bresenham::Bresenham;

    #[test]
    pub fn calculate_straight_x_line() {
        let sut = Bresenham::init(Position::ORIGIN, Position::new(10, 0));
        for (i, Position { x, y }) in sut.enumerate() {
            assert_eq!(i as i32 + 1, x);
            assert_eq!(0, y);
        }
    }

    #[test]
    pub fn calculate_straight_y_line() {
        let sut = Bresenham::init(Position::ORIGIN, Position::new(0, 10));
        for (i, Position { x, y }) in sut.enumerate() {
            assert_eq!(0, x);
            assert_eq!(i as i32 + 1, y);
        }
    }

    #[test]
    pub fn calculate_diagonal_line() {
        let sut = Bresenham::init(Position::ORIGIN, Position::new(10, 10));
        for (i, Position { x, y }) in sut.enumerate() {
            assert_eq!(i as i32 + 1, x);
            assert_eq!(i as i32 + 1, y);
        }
    }

    #[test]
    pub fn calculate_staggered_diagonal_line() {
        let sut = Bresenham::init(Position::ORIGIN, Position::new(20, 10));
        for (i, Position { x, y }) in sut.enumerate() {
            assert_eq!(i as i32 + 1, x);
            assert_eq!(((i + 1) / 2) as i32, y);
        }
    }
}
