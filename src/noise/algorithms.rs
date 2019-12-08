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

pub(crate) mod perlin;
pub(crate) mod simplex;
pub(crate) mod wavelet;

use crate::noise::MAX_DIMENSIONS;
use crate::random::{Algorithm as RandomAlgorithm, Random, Rng};
use ilyvion_util::multi_dimensional::Window2D;

pub trait Algorithm {
    fn new<R: RandomAlgorithm>(dimensions: usize, initializer: AlgorithmInitializer<R>) -> Self;

    fn generate(&self, f: &[f32]) -> f32;
}

pub struct AlgorithmInitializer<R: RandomAlgorithm> {
    random: Random<R>,
}

impl<R: RandomAlgorithm> AlgorithmInitializer<R> {
    pub fn new(random: Random<R>) -> Self {
        Self { random }
    }

    pub fn map(&mut self) -> [u8; 256] {
        let mut map = [0; 256];
        for i in 0_u8..=255 {
            map[i as usize] = i;
        }

        for i in (0..255).rev() {
            let j = self.random.get_i32(0, 255) as usize;
            if i == j {
                continue;
            }
            map.swap(i, j);
        }

        map
    }

    pub fn buffer(&mut self, dimensions: usize) -> [f32; MAX_DIMENSIONS * 256] {
        let mut buffer = [0.0; MAX_DIMENSIONS * 256];
        let mut buffer_window = Window2D::new_mut_unchecked(&mut buffer, 256, MAX_DIMENSIONS);
        for i in 0_u8..=255 {
            for j in 0..dimensions {
                buffer_window[i as usize][j] = self.random.get_f32(-0.5, 0.5);
            }
            Self::normalize(dimensions, &mut buffer_window[i as usize]);
        }

        buffer
    }

    fn normalize(dimensions: usize, f: &mut [f32]) {
        let mut magnitude = 0.0;
        for &i in f.iter().take(dimensions) {
            magnitude += i * i;
        }
        magnitude = 1.0 / magnitude.sqrt();
        for i in f.iter_mut().take(dimensions) {
            *i *= magnitude;
        }
    }
}
