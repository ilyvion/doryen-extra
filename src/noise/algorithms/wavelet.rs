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
use crate::random::{Algorithm as RandomAlgorithm, Random, Rng};
use crate::util::FloorRem;
#[cfg(feature = "debug")]
use derivative::Derivative;
use std::mem::MaybeUninit;

#[allow(dead_code)]
const WAVELET_TILE_SIZE: usize = 32;
const WAVELET_TILE_SIZE_SQUARED: usize = WAVELET_TILE_SIZE * WAVELET_TILE_SIZE;
const WAVELET_TILE_SIZE_CUBED: usize = WAVELET_TILE_SIZE_SQUARED * WAVELET_TILE_SIZE;
const WAVELET_ARAD: usize = 16;

const WAVELET_SCALE: f32 = 2.0;

/* wavelet noise, adapted from Robert L. Cook and Tony Derose 'Wavelet noise' paper */

#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Derivative))]
#[cfg_attr(feature = "debug", derivative(Debug))]
pub struct Wavelet {
    dimensions: usize,
    #[cfg_attr(feature = "debug", derivative(Debug = "ignore"))]
    tile_data: Box<[f32; WAVELET_TILE_SIZE_CUBED]>,
}

impl Algorithm for Wavelet {
    fn new<R: RandomAlgorithm>(dimensions: usize, initializer: AlgorithmInitializer<R>) -> Self {
        let mut random = initializer.random;
        let tile_data = WaveletTileData::initialize(&mut random);
        Self {
            dimensions,
            tile_data,
        }
    }

    #[allow(clippy::many_single_char_names)]
    fn generate(&self, f: &[f32]) -> f32 {
        if self.dimensions > 3 {
            panic!("Wavelet noise only supports up to 3 dimensions");
        }

        let mut pf = [0.0; 3];
        for (pfe, &fe) in Iterator::zip(
            pf.iter_mut().take(self.dimensions),
            f.iter().take(self.dimensions),
        ) {
            *pfe = fe * WAVELET_SCALE;
        }

        let mut mid = [0; 3];
        let mut w = [[0.0; 3]; 3];
        let mut t;
        for i in 0..3 {
            mid[i] = (pf[i] - 0.5).ceil() as i32;
            t = mid[i] as f32 - (pf[i] - 0.5);
            w[i][0] = t * t * 0.5;
            w[i][2] = (1.0 - t) * (1.0 - t) * 0.5;
            w[i][1] = 1.0 - w[i][0] - w[i][2];
        }

        let mut c = [0; 3];
        let mut result = 0.0;
        let mid = mid;
        for p2 in -1..=1 {
            for p1 in -1..=1 {
                for p0 in -1..=1 {
                    let mut weight = 1.0;
                    for i in 0..3 {
                        let p = match i {
                            0 => p0,
                            1 => p1,
                            2 => p2,
                            _ => unreachable!(),
                        };

                        c[i] = (mid[i].wrapping_add(p)).floor_modulo(WAVELET_TILE_SIZE as i32);
                        weight *= w[i][(p + 1) as usize];
                    }
                    result += weight
                        * self.tile_data[c[2] as usize * WAVELET_TILE_SIZE_SQUARED
                            + c[1] as usize * WAVELET_TILE_SIZE
                            + c[0] as usize];
                }
            }
        }

        result.max(-1.0).min(1.0)
    }
}

pub struct WaveletTileData;

impl WaveletTileData {
    pub fn initialize<R: RandomAlgorithm>(
        random: &mut Random<R>,
    ) -> Box<[f32; WAVELET_TILE_SIZE_CUBED]> {
        let mut noise = Self::generate_noise(random);
        let mut temp1 = [0.0; WAVELET_TILE_SIZE_CUBED];
        let mut temp2 = [0.0; WAVELET_TILE_SIZE_CUBED];

        for iy in 0..WAVELET_TILE_SIZE {
            for iz in 0..WAVELET_TILE_SIZE {
                let i = iy * WAVELET_TILE_SIZE + iz * WAVELET_TILE_SIZE_SQUARED;
                Self::downsample(&noise[i..], &mut temp1[i..], 1);
                Self::upsample(&temp1[i..], &mut temp2[i..], 1);
            }
        }
        for ix in 0..WAVELET_TILE_SIZE {
            for iz in 0..WAVELET_TILE_SIZE {
                let i = ix + iz * WAVELET_TILE_SIZE_SQUARED;
                Self::downsample(&temp2[i..], &mut temp1[i..], WAVELET_TILE_SIZE);
                Self::upsample(&temp1[i..], &mut temp2[i..], WAVELET_TILE_SIZE);
            }
        }
        for ix in 0..WAVELET_TILE_SIZE {
            for iy in 0..WAVELET_TILE_SIZE {
                let i = ix + iy * WAVELET_TILE_SIZE;
                Self::downsample(&temp2[i..], &mut temp1[i..], WAVELET_TILE_SIZE_SQUARED);
                Self::upsample(&temp1[i..], &mut temp2[i..], WAVELET_TILE_SIZE_SQUARED);
            }
        }
        for (n, &t) in Iterator::zip(noise.iter_mut(), temp2.iter()) {
            *n -= t;
        }
        let mut offset = WAVELET_TILE_SIZE / 2;
        if offset & 1 == 0 {
            offset += 1
        }
        let mut i = 0;
        for ix in 0..WAVELET_TILE_SIZE {
            for iy in 0..WAVELET_TILE_SIZE {
                for iz in 0..WAVELET_TILE_SIZE {
                    temp1[i] = noise[((ix + offset) % (WAVELET_TILE_SIZE))
                        + ((iy + offset) % (WAVELET_TILE_SIZE)) * WAVELET_TILE_SIZE
                        + ((iz + offset) % (WAVELET_TILE_SIZE)) * WAVELET_TILE_SIZE_SQUARED];
                    i += 1;
                }
            }
        }
        for (n, &t) in Iterator::zip(noise.iter_mut(), temp1.iter()) {
            *n += t;
        }

        noise
    }

    #[allow(unsafe_code)]
    fn generate_noise<R: RandomAlgorithm>(
        random: &mut Random<R>,
    ) -> Box<[f32; WAVELET_TILE_SIZE_CUBED]> {
        let mut noise: Box<[MaybeUninit<f32>; WAVELET_TILE_SIZE_CUBED]> =
            Box::new(unsafe { MaybeUninit::uninit().assume_init() });
        for n in noise.iter_mut() {
            unsafe {
                n.as_mut_ptr().write(random.get_f32(-1.0, 1.0));
            }
        }

        unsafe { Box::from_raw(Box::into_raw(noise).cast::<[f32; WAVELET_TILE_SIZE_CUBED]>()) }
    }

    fn downsample(from: &[f32], to: &mut [f32], stride: usize) {
        const A_COEFFICIENTS: [f32; 2 * WAVELET_ARAD] = [
            0.000_334, -0.001_528, 0.000_410, 0.003_545, -0.000_938, -0.008_233, 0.002_172,
            0.019_120, -0.005_040, -0.044_412, 0.011_655, 0.103_311, -0.025_936, -0.243_780,
            0.033_979, 0.655_340, 0.655_340, 0.033_979, -0.243_780, -0.025_936, 0.103_311,
            0.011_655, -0.044_412, -0.005_040, 0.019_120, 0.002_172, -0.008_233, -0.000_938,
            0.003_546, 0.000_410, -0.001_528, 0.000_334,
        ];

        for i in 0..WAVELET_TILE_SIZE as isize / 2 {
            to[i as usize * stride] = 0.0;
            for k in 2 * i - WAVELET_ARAD as isize..2 * i + WAVELET_ARAD as isize {
                to[i as usize * stride] += A_COEFFICIENTS
                    [(16 + k as isize - 2 * i as isize) as usize]
                    * from[k.floor_modulo(WAVELET_TILE_SIZE as isize) as usize * stride];
            }
        }
    }

    fn upsample(from: &[f32], to: &mut [f32], stride: usize) {
        const P_COEFFICIENTS: [f32; 4] = [0.25, 0.75, 0.75, 0.25];

        for i in 0..WAVELET_TILE_SIZE as isize {
            to[i as usize * stride] = 0.0;
            for k in i / 2..=i / 2 {
                to[i as usize * stride] += P_COEFFICIENTS[(2 + i - 2 * k) as usize]
                    * from[k.floor_modulo(WAVELET_TILE_SIZE as isize / 2) as usize * stride];
            }
        }
    }
}
