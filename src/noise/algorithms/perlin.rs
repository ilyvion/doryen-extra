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

use crate::noise::algorithms::AlgorithmInitializer;
use crate::noise::{Algorithm, MAX_DIMENSIONS};
use crate::random::Algorithm as RandomAlgorithm;
use derivative::Derivative;
use ilyvion_util::multi_dimensional::Window2D;

#[derive(Clone, Copy, Derivative)]
#[derivative(Debug)]
pub struct Perlin {
    dimensions: usize,
    /** Randomized map of indexes into buffer */
    #[derivative(Debug = "ignore")]
    pub map: [u8; 256],
    /** Random 256 x ndim buffer */
    #[derivative(Debug = "ignore")]
    pub buffer: [f32; MAX_DIMENSIONS * 256],
}

impl Perlin {
    #[allow(clippy::too_many_arguments)]
    fn lattice(
        &self,
        ix: i32,
        fx: f32,
        iy: i32,
        fy: f32,
        iz: i32,
        fz: f32,
        iw: i32,
        fw: f32,
    ) -> f32 {
        let n: [i32; 4] = [ix, iy, iz, iw];
        let f: [f32; 4] = [fx, fy, fz, fw];
        let mut n_index = 0;
        for &ni in n.iter().take(self.dimensions) {
            n_index = i32::from(self.map[((n_index + ni) & 0xFF) as usize]);
        }
        let buffer_window = Window2D::new_ref_unchecked(&self.buffer, 256, MAX_DIMENSIONS);

        Iterator::zip(buffer_window[n_index as usize].iter(), f.iter())
            .take(self.dimensions)
            .map(|(b, f)| b * f)
            .sum()
    }

    fn perlin_1d(
        &self,
        n: [i32; MAX_DIMENSIONS],
        r: [f32; MAX_DIMENSIONS],
        w: [f32; MAX_DIMENSIONS],
    ) -> f32 {
        lerp!(
            self.lattice(n[0], r[0], 0, 0.0, 0, 0.0, 0, 0.0),
            self.lattice(n[0] + 1, r[0] - 1.0, 0, 0.0, 0, 0.0, 0, 0.0),
            w[0]
        )
    }

    fn perlin_2d(
        &self,
        n: [i32; MAX_DIMENSIONS],
        r: [f32; MAX_DIMENSIONS],
        w: [f32; MAX_DIMENSIONS],
    ) -> f32 {
        lerp!(
            lerp!(
                self.lattice(n[0], r[0], n[1], r[1], 0, 0.0, 0, 0.0),
                self.lattice(n[0] + 1, r[0] - 1.0, n[1], r[1], 0, 0.0, 0, 0.0),
                w[0]
            ),
            lerp!(
                self.lattice(n[0], r[0], n[1] + 1, r[1] - 1.0, 0, 0.0, 0, 0.0),
                self.lattice(n[0] + 1, r[0] - 1.0, n[1] + 1, r[1] - 1.0, 0, 0.0, 0, 0.0),
                w[0]
            ),
            w[1]
        )
    }

    fn perlin_3d(
        &self,
        n: [i32; MAX_DIMENSIONS],
        r: [f32; MAX_DIMENSIONS],
        w: [f32; MAX_DIMENSIONS],
    ) -> f32 {
        lerp!(
            lerp!(
                lerp!(
                    self.lattice(n[0], r[0], n[1], r[1], n[2], r[2], 0, 0.0),
                    self.lattice(n[0] + 1, r[0] - 1.0, n[1], r[1], n[2], r[2], 0, 0.0),
                    w[0]
                ),
                lerp!(
                    self.lattice(n[0], r[0], n[1] + 1, r[1] - 1.0, n[2], r[2], 0, 0.0),
                    self.lattice(
                        n[0] + 1,
                        r[0] - 1.0,
                        n[1] + 1,
                        r[1] - 1.0,
                        n[2],
                        r[2],
                        0,
                        0.0
                    ),
                    w[0]
                ),
                w[1]
            ),
            lerp!(
                lerp!(
                    self.lattice(n[0], r[0], n[1], r[1], n[2] + 1, r[2] - 1.0, 0, 0.0),
                    self.lattice(
                        n[0] + 1,
                        r[0] - 1.0,
                        n[1],
                        r[1],
                        n[2] + 1,
                        r[2] - 1.0,
                        0,
                        0.0
                    ),
                    w[0]
                ),
                lerp!(
                    self.lattice(
                        n[0],
                        r[0],
                        n[1] + 1,
                        r[1] - 1.0,
                        n[2] + 1,
                        r[2] - 1.0,
                        0,
                        0.0
                    ),
                    self.lattice(
                        n[0] + 1,
                        r[0] - 1.0,
                        n[1] + 1,
                        r[1] - 1.0,
                        n[2] + 1,
                        r[2] - 1.0,
                        0,
                        0.0
                    ),
                    w[0]
                ),
                w[1]
            ),
            w[2]
        )
    }

    #[allow(clippy::too_many_lines)]
    fn perlin_4d(
        &self,
        n: [i32; MAX_DIMENSIONS],
        r: [f32; MAX_DIMENSIONS],
        w: [f32; MAX_DIMENSIONS],
    ) -> f32 {
        lerp!(
            lerp!(
                lerp!(
                    lerp!(
                        self.lattice(n[0], r[0], n[1], r[1], n[2], r[2], n[3], r[3]),
                        self.lattice(n[0] + 1, r[0] - 1.0, n[1], r[1], n[2], r[2], n[3], r[3]),
                        w[0]
                    ),
                    lerp!(
                        self.lattice(n[0], r[0], n[1] + 1, r[1] - 1.0, n[2], r[2], n[3], r[3]),
                        self.lattice(
                            n[0] + 1,
                            r[0] - 1.0,
                            n[1] + 1,
                            r[1] - 1.0,
                            n[2],
                            r[2],
                            n[3],
                            r[3]
                        ),
                        w[0]
                    ),
                    w[1]
                ),
                lerp!(
                    lerp!(
                        self.lattice(n[0], r[0], n[1], r[1], n[2] + 1, r[2] - 1.0, n[3], r[3]),
                        self.lattice(
                            n[0] + 1,
                            r[0] - 1.0,
                            n[1],
                            r[1],
                            n[2] + 1,
                            r[2] - 1.0,
                            n[3],
                            r[3]
                        ),
                        w[0]
                    ),
                    lerp!(
                        self.lattice(
                            n[0],
                            r[0],
                            n[1] + 1,
                            r[1] - 1.0,
                            n[2] + 1,
                            r[2] - 1.0,
                            0,
                            0.0
                        ),
                        self.lattice(
                            n[0] + 1,
                            r[0] - 1.0,
                            n[1] + 1,
                            r[1] - 1.0,
                            n[2] + 1,
                            r[2] - 1.0,
                            n[3],
                            r[3]
                        ),
                        w[0]
                    ),
                    w[1]
                ),
                w[2]
            ),
            lerp!(
                lerp!(
                    lerp!(
                        self.lattice(n[0], r[0], n[1], r[1], n[2], r[2], n[3] + 1, r[3] - 1.0),
                        self.lattice(
                            n[0] + 1,
                            r[0] - 1.0,
                            n[1],
                            r[1],
                            n[2],
                            r[2],
                            n[3] + 1,
                            r[3] - 1.0
                        ),
                        w[0]
                    ),
                    lerp!(
                        self.lattice(
                            n[0],
                            r[0],
                            n[1] + 1,
                            r[1] - 1.0,
                            n[2],
                            r[2],
                            n[3] + 1,
                            r[3] - 1.0
                        ),
                        self.lattice(
                            n[0] + 1,
                            r[0] - 1.0,
                            n[1] + 1,
                            r[1] - 1.0,
                            n[2],
                            r[2],
                            n[3] + 1,
                            r[3] - 1.0
                        ),
                        w[0]
                    ),
                    w[1]
                ),
                lerp!(
                    lerp!(
                        self.lattice(
                            n[0],
                            r[0],
                            n[1],
                            r[1],
                            n[2] + 1,
                            r[2] - 1.0,
                            n[3] + 1,
                            r[3] - 1.0
                        ),
                        self.lattice(
                            n[0] + 1,
                            r[0] - 1.0,
                            n[1],
                            r[1],
                            n[2] + 1,
                            r[2] - 1.0,
                            n[3] + 1,
                            r[3] - 1.0
                        ),
                        w[0]
                    ),
                    lerp!(
                        self.lattice(
                            n[0],
                            r[0],
                            n[1] + 1,
                            r[1] - 1.0,
                            n[2] + 1,
                            r[2] - 1.0,
                            0,
                            0.0
                        ),
                        self.lattice(
                            n[0] + 1,
                            r[0] - 1.0,
                            n[1] + 1,
                            r[1] - 1.0,
                            n[2] + 1,
                            r[2] - 1.0,
                            n[3] + 1,
                            r[3] - 1.0
                        ),
                        w[0]
                    ),
                    w[1]
                ),
                w[2]
            ),
            w[3]
        )
    }

    fn cubic_f32(a: f32) -> f32 {
        a * a * (3.0 - 2.0 * a)
    }
}

impl Algorithm for Perlin {
    fn new<R: RandomAlgorithm>(
        dimensions: usize,
        mut initializer: AlgorithmInitializer<R>,
    ) -> Self {
        Self {
            dimensions,
            map: initializer.map(),
            buffer: initializer.buffer(dimensions),
        }
    }

    fn generate(&self, f: &[f32]) -> f32 {
        let mut n: [i32; MAX_DIMENSIONS] = [0; MAX_DIMENSIONS]; /* Indexes to pass to lattice function */
        let mut r: [f32; MAX_DIMENSIONS] = [0.0; MAX_DIMENSIONS]; /* Remainders to pass to lattice function */
        let mut w: [f32; MAX_DIMENSIONS] = [0.0; MAX_DIMENSIONS]; /* Cubic values to pass to interpolation function */
        for i in 0..self.dimensions {
            n[i] = f[i].floor() as i32;
            r[i] = f[i] - n[i] as f32;
            w[i] = Self::cubic_f32(r[i]);
        }

        let value = match self.dimensions {
            1 => self.perlin_1d(n, r, w),
            2 => self.perlin_2d(n, r, w),
            3 => self.perlin_3d(n, r, w),
            4 => self.perlin_4d(n, r, w),
            _ => unreachable!(),
        };

        value.max(-0.99999).min(0.99999)
    }
}
