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

// These are needed due to a strange interaction between clippy and the output
// produced by the derivative crate on the Noise struct.
#![allow(clippy::match_single_binding)]
#![allow(clippy::use_self)]

//! # Noise generation.
//!
//! This module provides several ways to generate Perlin noise and other derived noises.
//! It can handle noise functions from 1 to 4 dimensions.
//!
//! ## Usage examples:
//! * 1D noise : the variation of a torch intensity
//! * 2D fbm : height field generation or clouds
//! * 3D fbm : animated smoke
//!
//! ## Choosing a noise type
//! The default choice should be Simplex. It's much faster than Perlin, especially in 4 dimensions.
//! It has a better contrast too.

pub mod algorithms;

use crate::noise::algorithms::Algorithm;
use crate::noise::algorithms::AlgorithmInitializer;
use crate::noise::algorithms::Perlin;
use crate::noise::algorithms::Simplex;
use crate::noise::algorithms::Wavelet;
use crate::random::algorithms::Algorithm as RandomAlgorithm;
use crate::random::Random;
use derivative::Derivative;

/// The maximum number of octaves supported.
pub const MAX_OCTAVES: usize = 128;
/// The maximum number of dimensions supported.
pub const MAX_DIMENSIONS: usize = 4;
//pub const DEFAULT_HURST: f32 = 0.5;
/// The default lacunarity value.
pub const DEFAULT_LACUNARITY: f32 = 2.0;

const DELTA: f32 = 1.0e-6;

/// A struct representing a noise generator algorithm and its parameters.

#[derive(Derivative)]
#[derivative(Debug)]
pub struct Noise<A: Algorithm> {
    pub(crate) dimensions: usize,
    algorithm: A,
    #[derivative(Debug = "ignore")]
    exponent: [f32; MAX_OCTAVES],
    lacunarity: f32,
}

impl<A: Algorithm> Noise<A> {
    /// Returns the noise function value between -1.0 and 1.0 at the given coordinates.
    /// The same array of coordinates will always return the same value.
    ///
    /// # Panics
    /// If the `f` slice's length isn't equal to the `Noise`'s dimensions.
    pub fn flat(&self, f: &[f32]) -> f32 {
        assert_eq!(
            self.dimensions,
            f.len(),
            "Number of coordinates given in 'f' must match the dimensions."
        );

        self.algorithm.generate(f)
    }

    /// Returns the Fractal Brownian Motion function value between -1.0 and 1.0 at the given
    /// coordinates, using the lacunarity defined when the noise generator was created.
    /// The same array of coordinates will always return the same value.
    ///
    /// The octaves decide the number of iterations. Must be < `MAX_OCTAVES`, i.e. 128.
    ///
    /// # Panics
    /// If the `f` slice's length isn't equal to the `Noise`'s dimensions.
    pub fn fbm(&self, f: &[f32], mut octaves: f32) -> f32 {
        assert_eq!(
            self.dimensions,
            f.len(),
            "Number of coordinates given in 'f' must match the dimensions."
        );

        let mut tf = [0.0_f32; MAX_DIMENSIONS];
        tf[0..self.dimensions].copy_from_slice(f);

        let mut value: f64 = 0.0;
        /* Inner loop of spectral construction, where the fractal is built */
        for &e in self.exponent.iter().take(octaves.trunc() as usize) {
            value += f64::from(self.algorithm.generate(&tf)) * f64::from(e);
            for tfe in tf.iter_mut().take(f.len()) {
                *tfe *= self.lacunarity;
            }
        }

        /* Take care of remainder in octaves */
        let exp_i = octaves.trunc() as usize;
        octaves -= octaves.trunc();
        if octaves > DELTA {
            value +=
                f64::from(octaves * self.algorithm.generate(&tf)) * f64::from(self.exponent[exp_i]);
        }

        value.max(-0.99999).min(0.99999) as f32
    }

    /// Returns the turbulence function value between -1.0 and 1.0 at the given
    /// coordinates, using the lacunarity defined when the noise generator was created.
    /// The same array of coordinates will always return the same value.
    ///
    /// The octaves decide the number of iterations. Must be < `MAX_OCTAVES`, i.e. 128.
    ///
    /// # Panics
    /// If the `f` slice's length isn't equal to the `Noise`'s dimensions.
    pub fn turbulence(&self, f: &[f32], mut octaves: f32) -> f32 {
        assert_eq!(
            self.dimensions,
            f.len(),
            "Number of coordinates given in 'f' must match the dimensions."
        );

        let mut tf = [0.0_f32; MAX_DIMENSIONS];
        tf[0..self.dimensions].copy_from_slice(f);

        let mut value: f64 = 0.0;
        /* Inner loop of spectral construction, where the fractal is built */
        for &e in self.exponent.iter().take(octaves.trunc() as usize) {
            value += f64::from(self.algorithm.generate(&tf).abs()) * f64::from(e);
            for tfe in tf.iter_mut().take(f.len()) {
                *tfe *= self.lacunarity;
            }
        }

        /* Take care of remainder in octaves */
        let exp_i = octaves.trunc() as usize;
        octaves -= octaves.trunc();
        if octaves > DELTA {
            value += f64::from(octaves * self.algorithm.generate(&tf).abs())
                * f64::from(self.exponent[exp_i]);
        }

        value.max(-0.99999).min(0.99999) as f32
    }

    fn new<R: RandomAlgorithm>(
        mut dimensions: usize,
        //hurst: f32,
        lacunarity: f32,
        random: Random<R>,
    ) -> Self {
        dimensions = dimensions.min(4);

        let initializer = AlgorithmInitializer::new(random);

        Self {
            dimensions,
            algorithm: A::new(dimensions, initializer),
            exponent: Self::exponent(lacunarity),
            lacunarity,
        }
    }

    fn exponent(lacunarity: f32) -> [f32; MAX_OCTAVES] {
        let mut exponent = [0.0; MAX_OCTAVES];
        let mut f = 1.0;
        for e in exponent.iter_mut() {
            *e = 1.0 / f;
            f *= lacunarity;
        }

        exponent
    }
}

impl Noise<Perlin> {
    /// Initializes a Perlin noise generator with the given number of dimensions (from 1 to 4),
    /// the lacunarity parameter and a random number generator.
    pub fn new_perlin<R: RandomAlgorithm>(
        dimensions: usize,
        lacunarity: f32,
        random: Random<R>,
    ) -> Self {
        Self::new(dimensions, lacunarity, random)
    }
}

impl Noise<Simplex> {
    /// Initializes a Simplex noise generator with the given number of dimensions (from 1 to 4),
    /// the lacunarity parameter and a random number generator.
    pub fn new_simplex<R: RandomAlgorithm>(
        dimensions: usize,
        lacunarity: f32,
        random: Random<R>,
    ) -> Self {
        Self::new(dimensions, lacunarity, random)
    }
}

impl Noise<Wavelet> {
    /// Initializes a Wavelet noise generator with the given number of dimensions (from 1 to 4),
    /// the lacunarity parameter and a random number generator.
    pub fn new_wavelet<R: RandomAlgorithm>(
        dimensions: usize,
        lacunarity: f32,
        random: Random<R>,
    ) -> Self {
        Self::new(dimensions, lacunarity, random)
    }
}
