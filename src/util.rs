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

use std::ops::Rem;

pub(crate) trait FloorRem<Rhs = Self>: Rem<Rhs> {
    /// Returns floor modulo.
    #[must_use]
    fn floor_modulo(self, rhs: Self) -> Self::Output;
}

impl FloorRem for f32 {
    fn floor_modulo(self, rhs: Self) -> Self::Output {
        let m = self % rhs;
        if m < 0.0 {
            m + rhs
        } else {
            m
        }
    }
}

impl FloorRem for i32 {
    fn floor_modulo(self, rhs: Self) -> Self::Output {
        let m = self % rhs;
        if m < 0 {
            m + rhs
        } else {
            m
        }
    }
}

impl FloorRem for isize {
    fn floor_modulo(self, rhs: Self) -> Self::Output {
        let m = self % rhs;
        if m < 0 {
            m + rhs
        } else {
            m
        }
    }
}

macro_rules! lerp {
    ($a:expr, $b:expr, $x:expr) => {{
        let a = $a;
        a + $x * ($b - a)
    }};
}
