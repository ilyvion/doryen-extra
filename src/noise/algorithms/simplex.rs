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
use crate::noise::Algorithm;
use crate::random::Algorithm as RandomAlgorithm;
use crate::util::FloorRem;

use derivative::Derivative;

#[derive(Clone, Copy, Derivative)]
#[derivative(Debug)]
pub struct Simplex {
    dimensions: usize,
    #[derivative(Debug = "ignore")]
    map: [u8; 256],
}

impl Algorithm for Simplex {
    fn new<R: RandomAlgorithm>(
        dimensions: usize,
        mut initializer: AlgorithmInitializer<R>,
    ) -> Self {
        Self {
            dimensions,
            map: initializer.map(),
        }
    }

    fn generate(&self, f: &[f32]) -> f32 {
        assert!(f.len() >= self.dimensions);

        match self.dimensions {
            1 => self.simplex_1d(f[0]),
            2 => self.simplex_2d(f[0], f[1]),
            3 => self.simplex_3d(f[0], f[1], f[2]),
            4 => self.simplex_4d(f[0], f[1], f[2], f[3]),
            _ => unreachable!(),
        }
    }
}

impl Simplex {
    const SIMPLEX_SCALE: f32 = 0.5;
    const SIMPLEX: [[f32; 4]; 64] = [
        [0.0, 1.0, 2.0, 3.0],
        [0.0, 1.0, 3.0, 2.0],
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 2.0, 3.0, 1.0],
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
        [1.0, 2.0, 3.0, 0.0],
        [0.0, 2.0, 1.0, 3.0],
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 3.0, 1.0, 2.0],
        [0.0, 3.0, 2.0, 1.0],
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
        [1.0, 3.0, 2.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
        [1.0, 2.0, 0.0, 3.0],
        [0.0, 0.0, 0.0, 0.0],
        [1.0, 3.0, 0.0, 2.0],
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
        [2.0, 3.0, 0.0, 1.0],
        [2.0, 3.0, 1.0, 0.0],
        [1.0, 0.0, 2.0, 3.0],
        [1.0, 0.0, 3.0, 2.0],
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
        [2.0, 0.0, 3.0, 1.0],
        [0.0, 0.0, 0.0, 0.0],
        [2.0, 1.0, 3.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
        [2.0, 0.0, 1.0, 3.0],
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
        [3.0, 0.0, 1.0, 2.0],
        [3.0, 0.0, 2.0, 1.0],
        [0.0, 0.0, 0.0, 0.0],
        [3.0, 1.0, 2.0, 0.0],
        [2.0, 1.0, 0.0, 3.0],
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
        [3.0, 1.0, 0.0, 2.0],
        [0.0, 0.0, 0.0, 0.0],
        [3.0, 2.0, 0.0, 1.0],
        [3.0, 2.0, 1.0, 0.0],
    ];

    fn simplex_1d(&self, f0: f32) -> f32 {
        let i0 = (f0 * Self::SIMPLEX_SCALE).floor() as i32;
        let i1 = i0 + 1;
        let x0 = f0 * Self::SIMPLEX_SCALE - i0 as f32;
        let x1 = x0 - 1.0;
        let t0 = 1.0 - x0 * x0;
        let t1 = 1.0 - x1 * x1;
        let t0 = t0 * t0;
        let t1 = t1 * t1;
        let i0 = i32::from(self.map[(i0 & 0xFF) as usize]);
        let mut n0 = Self::simplex_gradient_1d(i0, x0);
        n0 *= t0 * t0;
        let i1 = i32::from(self.map[(i1 & 0xFF) as usize]);
        let mut n1 = Self::simplex_gradient_1d(i1, x1);
        n1 *= t1 * t1;

        0.25 * (n0 + n1)
    }

    #[allow(clippy::many_single_char_names)]
    fn simplex_2d(&self, f0: f32, f1: f32) -> f32 {
        const F2: f64 = 0.366_025_403;
        const G2: f64 = 0.211_324_865;

        let s = f64::from(f0 + f1) * F2 * f64::from(Self::SIMPLEX_SCALE);
        let xs = f0 * Self::SIMPLEX_SCALE + s as f32;
        let ys = f1 * Self::SIMPLEX_SCALE + s as f32;
        let i = xs.floor() as i32;
        let j = ys.floor() as i32;
        let t = (f64::from(i) + f64::from(j)) * G2;
        let xo = f64::from(i) - t;
        let yo = f64::from(j) - t;
        let x0 = f0 * Self::SIMPLEX_SCALE - xo as f32;
        let y0 = f1 * Self::SIMPLEX_SCALE - yo as f32;
        let ii = i.floor_modulo(256);
        let jj = j.floor_modulo(256);
        let (i1, j1) = if x0 > y0 { (1, 0) } else { (0, 1) };
        let x1 = x0 - i1 as f32 + G2 as f32;
        let y1 = y0 - j1 as f32 + G2 as f32;
        let x2 = x0 - 1.0 + (2.0 * G2) as f32;
        let y2 = y0 - 1.0 + (2.0 * G2) as f32;
        let mut t0 = 0.5 - x0 * x0 - y0 * y0;

        let n0 = if t0 < 0.0 {
            0.0
        } else {
            let mut idx = (ii + i32::from(self.map[jj as usize])) & 0xFF;
            t0 *= t0;
            idx = i32::from(self.map[idx as usize]);
            Self::simplex_gradient_2d(idx, x0, y0) * t0 * t0
        };
        let mut t1 = 0.5 - x1 * x1 - y1 * y1;
        let n1 = if t1 < 0.0 {
            0.0
        } else {
            let mut idx = (ii + i1 + i32::from(self.map[((jj + j1) & 0xFF) as usize])) & 0xFF;
            t1 *= t1;
            idx = i32::from(self.map[idx as usize]);
            Self::simplex_gradient_2d(idx, x1, y1) * t1 * t1
        };
        let mut t2 = 0.5 - x2 * x2 - y2 * y2;
        let n2 = if t2 < 0.0 {
            0.0
        } else {
            let mut idx = (ii + 1 + i32::from(self.map[((jj + 1) & 0xFF) as usize])) & 0xFF;
            t2 *= t2;
            idx = i32::from(self.map[idx as usize]);
            Self::simplex_gradient_2d(idx, x2, y2) * t2 * t2
        };

        40.0 * (n0 + n1 + n2)
    }

    #[allow(clippy::too_many_lines)]
    #[allow(clippy::many_single_char_names)]
    fn simplex_3d(&self, f0: f32, f1: f32, f2: f32) -> f32 {
        const F3: f64 = 0.333_333_333;
        const G3: f64 = 0.166_666_667;

        let s = f64::from(f0 + f1 + f2) * F3 * f64::from(Self::SIMPLEX_SCALE);
        let xs = f0 * Self::SIMPLEX_SCALE + s as f32;
        let ys = f1 * Self::SIMPLEX_SCALE + s as f32;
        let zs = f2 * Self::SIMPLEX_SCALE + s as f32;
        let i = xs.floor() as i32;
        let j = ys.floor() as i32;
        let k = zs.floor() as i32;
        let t = (f64::from(i) + f64::from(j) + f64::from(k)) * G3;
        let xo = f64::from(i) - t;
        let yo = f64::from(j) - t;
        let zo = f64::from(k) - t;
        let x0 = f0 * Self::SIMPLEX_SCALE - xo as f32;
        let y0 = f1 * Self::SIMPLEX_SCALE - yo as f32;
        let z0 = f2 * Self::SIMPLEX_SCALE - zo as f32;

        let (i1, j1, k1, i2, j2, k2) = if x0 >= y0 {
            if y0 >= z0 {
                (1, 0, 0, 1, 1, 0)
            } else if x0 >= z0 {
                (1, 0, 0, 1, 0, 1)
            } else {
                (0, 0, 1, 1, 0, 1)
            }
        } else if y0 < z0 {
            (0, 0, 1, 0, 1, 1)
        } else if x0 < z0 {
            (0, 1, 0, 0, 1, 1)
        } else {
            (0, 1, 0, 1, 1, 0)
        };

        let x1 = x0 - i1 as f32 + G3 as f32;
        let y1 = y0 - j1 as f32 + G3 as f32;
        let z1 = z0 - k1 as f32 + G3 as f32;
        let x2 = x0 - i2 as f32 + 2.0 * G3 as f32;
        let y2 = y0 - j2 as f32 + 2.0 * G3 as f32;
        let z2 = z0 - k2 as f32 + 2.0 * G3 as f32;
        let x3 = x0 - 1.0 + (3.0 * G3) as f32;
        let y3 = y0 - 1.0 + (3.0 * G3) as f32;
        let z3 = z0 - 1.0 + (3.0 * G3) as f32;
        let ii = i.floor_modulo(256);
        let jj = j.floor_modulo(256);
        let kk = k.floor_modulo(256);

        let mut t0 = 0.6 - x0 * x0 - y0 * y0 - z0 * z0;
        let n0 = if t0 < 0.0 {
            0.0
        } else {
            let idx = i32::from(
                self.map[((ii
                    + i32::from(
                        self.map[((jj + i32::from(self.map[kk as usize])) & 0xFF) as usize],
                    ))
                    & 0xFF) as usize],
            );
            t0 *= t0;
            Self::simplex_gradient_3d(idx, x0, y0, z0) * t0 * t0
        };

        let mut t1 = 0.6 - x1 * x1 - y1 * y1 - z1 * z1;
        let n1 = if t1 < 0.0 {
            0.0
        } else {
            let idx = i32::from(
                self.map[((ii
                    + i1
                    + i32::from(
                        self.map[((jj + j1 + i32::from(self.map[((kk + k1) & 0xFF) as usize]))
                            & 0xFF) as usize],
                    ))
                    & 0xFF) as usize],
            );
            t1 *= t1;
            Self::simplex_gradient_3d(idx, x1, y1, z1) * t1 * t1
        };

        let mut t2 = 0.6 - x2 * x2 - y2 * y2 - z2 * z2;
        let n2 = if t2 < 0.0 {
            0.0
        } else {
            let idx = i32::from(
                self.map[((ii
                    + i2
                    + i32::from(
                        self.map[((jj + j2 + i32::from(self.map[((kk + k2) & 0xFF) as usize]))
                            & 0xFF) as usize],
                    ))
                    & 0xFF) as usize],
            );
            t2 *= t2;
            Self::simplex_gradient_3d(idx, x2, y2, z2) * t2 * t2
        };

        let mut t3 = 0.6 - x3 * x3 - y3 * y3 - z3 * z3;
        let n3 = if t3 < 0.0 {
            0.0
        } else {
            let idx = i32::from(
                self.map[((ii
                    + 1
                    + i32::from(
                        self.map[((jj + 1 + i32::from(self.map[((kk + 1) & 0xFF) as usize])) & 0xFF)
                            as usize],
                    ))
                    & 0xFF) as usize],
            );
            t3 *= t3;
            Self::simplex_gradient_3d(idx, x3, y3, z3) * t3 * t3
        };

        32.0 * (n0 + n1 + n2 + n3)
    }

    #[allow(clippy::too_many_lines)]
    #[allow(clippy::many_single_char_names)]
    fn simplex_4d(&self, f0: f32, f1: f32, f2: f32, f3: f32) -> f32 {
        const F4: f64 = 0.309_016_994;
        const G4: f64 = 0.138_196_601;

        let s = f64::from(f0 + f1 + f2 + f3) * F4 * f64::from(Self::SIMPLEX_SCALE);
        let xs = f0 * Self::SIMPLEX_SCALE + s as f32;
        let ys = f1 * Self::SIMPLEX_SCALE + s as f32;
        let zs = f2 * Self::SIMPLEX_SCALE + s as f32;
        let ws = f3 * Self::SIMPLEX_SCALE + s as f32;

        let i = xs.floor() as i32;
        let j = ys.floor() as i32;
        let k = zs.floor() as i32;
        let l = ws.floor() as i32;

        let t = (f64::from(i) + f64::from(j) + f64::from(k) + f64::from(l)) * G4;
        let xo = f64::from(i) - t;
        let yo = f64::from(j) - t;
        let zo = f64::from(k) - t;
        let wo = f64::from(l) - t;

        let x0 = f0 * Self::SIMPLEX_SCALE - xo as f32;
        let y0 = f1 * Self::SIMPLEX_SCALE - yo as f32;
        let z0 = f2 * Self::SIMPLEX_SCALE - zo as f32;
        let w0 = f3 * Self::SIMPLEX_SCALE - wo as f32;

        let c1 = if x0 > y0 { 32 } else { 0 };
        let c2 = if x0 > z0 { 16 } else { 0 };
        let c3 = if y0 > z0 { 8 } else { 0 };
        let c4 = if x0 > w0 { 4 } else { 0 };
        let c5 = if y0 > w0 { 2 } else { 0 };
        let c6 = if z0 > w0 { 1 } else { 0 };
        let c = c1 + c2 + c3 + c4 + c5 + c6;

        let i1 = if Self::SIMPLEX[c][0] >= 3.0 { 1 } else { 0 };
        let j1 = if Self::SIMPLEX[c][1] >= 3.0 { 1 } else { 0 };
        let k1 = if Self::SIMPLEX[c][2] >= 3.0 { 1 } else { 0 };
        let l1 = if Self::SIMPLEX[c][3] >= 3.0 { 1 } else { 0 };

        let i2 = if Self::SIMPLEX[c][0] >= 2.0 { 1 } else { 0 };
        let j2 = if Self::SIMPLEX[c][1] >= 2.0 { 1 } else { 0 };
        let k2 = if Self::SIMPLEX[c][2] >= 2.0 { 1 } else { 0 };
        let l2 = if Self::SIMPLEX[c][3] >= 2.0 { 1 } else { 0 };

        let i3 = if Self::SIMPLEX[c][0] >= 1.0 { 1 } else { 0 };
        let j3 = if Self::SIMPLEX[c][1] >= 1.0 { 1 } else { 0 };
        let k3 = if Self::SIMPLEX[c][2] >= 1.0 { 1 } else { 0 };
        let l3 = if Self::SIMPLEX[c][3] >= 1.0 { 1 } else { 0 };

        let x1 = x0 - i1 as f32 + (G4) as f32;
        let y1 = y0 - j1 as f32 + (G4) as f32;
        let z1 = z0 - k1 as f32 + (G4) as f32;
        let w1 = w0 - l1 as f32 + (G4) as f32;
        let x2 = x0 - i2 as f32 + (2.0 * G4) as f32;
        let y2 = y0 - j2 as f32 + (2.0 * G4) as f32;
        let z2 = z0 - k2 as f32 + (2.0 * G4) as f32;
        let w2 = w0 - l2 as f32 + (2.0 * G4) as f32;
        let x3 = x0 - i3 as f32 + (3.0 * G4) as f32;
        let y3 = y0 - j3 as f32 + (3.0 * G4) as f32;
        let z3 = z0 - k3 as f32 + (3.0 * G4) as f32;
        let w3 = w0 - l3 as f32 + (3.0 * G4) as f32;
        let x4 = x0 - 1.0 + (4.0 * G4) as f32;
        let y4 = y0 - 1.0 + (4.0 * G4) as f32;
        let z4 = z0 - 1.0 + (4.0 * G4) as f32;
        let w4 = w0 - 1.0 + (4.0 * G4) as f32;

        let ii = i.floor_modulo(256);
        let jj = j.floor_modulo(256);
        let kk = k.floor_modulo(256);
        let ll = l.floor_modulo(256);

        let mut t0 = 0.6 - x0 * x0 - y0 * y0 - z0 * z0 - w0 * w0;
        let n0 = if t0 < 0.0 {
            0.0
        } else {
            let idx = i32::from(
                self.map[((ii
                    + i32::from(
                        self.map[((jj
                            + i32::from(
                                self.map[((kk + i32::from(self.map[ll as usize])) & 0xFF) as usize],
                            ))
                            & 0xFF) as usize],
                    ))
                    & 0xFF) as usize],
            );
            t0 *= t0;
            Self::simplex_gradient_4d(idx, x0, y0, z0, w0) * t0 * t0
        };

        let mut t1 = 0.6 - x1 * x1 - y1 * y1 - z1 * z1 - w1 * w1;
        let n1 = if t1 < 0.0 {
            0.0
        } else {
            let idx = i32::from(
                self.map[((ii
                    + i1
                    + i32::from(
                        self.map[((jj
                            + j1
                            + i32::from(
                                self.map[((kk
                                    + k1
                                    + i32::from(self.map[((ll + l1) & 0xFF) as usize]))
                                    & 0xFF) as usize],
                            ))
                            & 0xFF) as usize],
                    ))
                    & 0xFF) as usize],
            );
            t1 *= t1;
            Self::simplex_gradient_4d(idx, x1, y1, z1, w1) * t1 * t1
        };

        let mut t2 = 0.6 - x2 * x2 - y2 * y2 - z2 * z2 - w2 * w2;
        let n2 = if t2 < 0.0 {
            0.0
        } else {
            let idx = i32::from(
                self.map[((ii
                    + i2
                    + i32::from(
                        self.map[((jj
                            + j2
                            + i32::from(
                                self.map[((kk
                                    + k2
                                    + i32::from(self.map[((ll + l2) & 0xFF) as usize]))
                                    & 0xFF) as usize],
                            ))
                            & 0xFF) as usize],
                    ))
                    & 0xFF) as usize],
            );
            t2 *= t2;
            Self::simplex_gradient_4d(idx, x2, y2, z2, w2) * t2 * t2
        };

        let mut t3 = 0.6 - x3 * x3 - y3 * y3 - z3 * z3 - w3 * w3;
        let n3 = if t3 < 0.0 {
            0.0
        } else {
            let idx = i32::from(
                self.map[((ii
                    + i3
                    + i32::from(
                        self.map[((jj
                            + j3
                            + i32::from(
                                self.map[((kk
                                    + k3
                                    + i32::from(self.map[((ll + l3) & 0xFF) as usize]))
                                    & 0xFF) as usize],
                            ))
                            & 0xFF) as usize],
                    ))
                    & 0xFF) as usize],
            );
            t3 *= t3;
            Self::simplex_gradient_4d(idx, x3, y3, z3, w3) * t3 * t3
        };

        let mut t4 = 0.6 - x4 * x4 - y4 * y4 - z4 * z4 - w4 * w4;
        let n4 = if t4 < 0.0 {
            0.0
        } else {
            let idx = i32::from(
                self.map[((ii
                    + 1
                    + i32::from(
                        self.map[((jj
                            + 1
                            + i32::from(
                                self.map[((kk
                                    + 1
                                    + i32::from(self.map[((ll + 1) & 0xFF) as usize]))
                                    & 0xFF) as usize],
                            ))
                            & 0xFF) as usize],
                    ))
                    & 0xFF) as usize],
            );
            t4 *= t4;
            Self::simplex_gradient_4d(idx, x4, y4, z4, w4) * t4 * t4
        };

        27.0 * (n0 + n1 + n2 + n3 + n4)
    }

    fn simplex_gradient_1d(mut h: i32, x: f32) -> f32 {
        h &= 0xF;
        let mut grad = 1.0 + (h & 7) as f32;
        if h & 8 == 8 {
            grad = -grad;
        }
        grad * x
    }

    #[allow(clippy::many_single_char_names)]
    fn simplex_gradient_2d(mut h: i32, x: f32, y: f32) -> f32 {
        h &= 0x7;
        let (u, v) = if h < 4 { (x, 2.0 * y) } else { (y, 2.0 * x) };
        (if h & 1 == 1 { -u } else { u }) + (if h & 2 == 2 { -v } else { v })
    }

    #[allow(clippy::many_single_char_names)]
    #[allow(unused_parens)] // Bug in stable/nightly at the time of writing.
    fn simplex_gradient_3d(mut h: i32, x: f32, y: f32, z: f32) -> f32 {
        h &= 0xF;
        let u = if h < 8 { x } else { y };
        let v = if h < 4 {
            y
        } else if h == 12 || h == 14 {
            x
        } else {
            z
        };

        (if h & 1 == 1 { -u } else { u } + if h & 2 == 2 { -v } else { v })
    }

    #[allow(clippy::many_single_char_names)]
    #[allow(unused_parens)] // Bug in stable/nightly at the time of writing.
    fn simplex_gradient_4d(mut h: i32, x: f32, y: f32, z: f32, t: f32) -> f32 {
        h &= 0x1F;
        let u = if h < 24 { x } else { y };
        let v = if h < 16 { y } else { z };
        let w = if h < 8 { z } else { t };

        (if h & 1 == 1 { -u } else { u }
            + if h & 2 == 2 { -v } else { v }
            + if h & 4 == 4 { -w } else { w })
    }
}
