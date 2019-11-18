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

//! Pseudorandom number generator using the Mersenne Twister or Complementary Multiply With Carry
//! algorithms.
//!
//! This module used to be named `mersenne` in libtcod.

mod algorithms;

use crate::random::algorithms::{Algorithm, ComplementaryMultiplyWithCarry, MersenneTwister};
use std::time::SystemTime;

/// Trait providing methods for generating random numbers.
pub trait Rng {
    /// Get an `i32` between `min` and `max`.
    fn get_i32(&mut self, min: i32, max: i32) -> i32;

    /// Get an `f32` between `min` and `max`.
    fn get_f32(&mut self, min: f32, max: f32) -> f32;

    /// Get an `f64` between `min` and `max`.
    fn get_f64(&mut self, min: f64, max: f64) -> f64;

    /// Get an `i32` between `min` and `max`, using gaussian distribution with the given `mean`.
    fn get_i32_mean(&mut self, min: i32, max: i32, mean: i32) -> i32;

    /// Get an `f32` between `min` and `max`, using gaussian distribution with the given `mean`.
    fn get_f32_mean(&mut self, min: f32, max: f32, mean: f32) -> f32;

    /// Get an `f64` between `min` and `max`, using gaussian distribution with the given `mean`.
    fn get_f64_mean(&mut self, min: f64, max: f64, mean: f64) -> f64;
}

/// pseudorandom number generator toolkit
#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct Random<A: Algorithm> {
    /* algorithm identifier */
    algo: A,
    /* distribution */
    /// Decides the distribution used for generating random numbers
    pub distribution: Distribution,

    // Used for gaussian result caching
    y2: Option<f64>,
}

impl<A: Algorithm> Random<A> {
    fn default_seed() -> u64 {
        let now = SystemTime::now();
        let duration_since = now.duration_since(SystemTime::UNIX_EPOCH).unwrap();
        duration_since.as_secs()
    }

    fn get_i(&mut self, mut min: i32, mut max: i32) -> i32 {
        if max == min {
            return min;
        } else if max < min {
            std::mem::swap(&mut min, &mut max);
        }

        let delta = max - min + 1; // + 1 because it's used for modulo

        (self.algo.get_int() % delta as u32) as i32 + min
    }

    fn get_f(&mut self, mut min: f32, mut max: f32) -> f32 {
        if (max - min).abs() < 0.000_001 {
            return min;
        } else if max < min {
            std::mem::swap(&mut min, &mut max);
        }

        let delta = max - min;

        self.algo.get_float() * delta + min
    }

    fn get_d(&mut self, mut min: f64, mut max: f64) -> f64 {
        if (max - min).abs() < 0.000_001 {
            return min;
        } else if max < min {
            std::mem::swap(&mut min, &mut max);
        }

        let delta = max - min;

        self.algo.get_double() * delta + min
    }

    /* Box-Muller transform (Gaussian distribution) */

    fn get_gaussian_double(&mut self, mean: f64, std_deviation: f64) -> f64 {
        if let Some(y2) = self.y2.take() {
            return mean + y2 * std_deviation;
        }

        let (x1, x2, w) = loop {
            let x1 = self.algo.get_double() * 2.0 - 1.0;
            let x2 = self.algo.get_double() * 2.0 - 1.0;
            let w = x1.powi(2) + x2.powi(2);
            if w < 1.0 {
                break (x1, x2, (-2.0 * w.ln() / w).sqrt());
            }
        };

        let y1 = x1 * w;
        self.y2 = Some(x2 * w);

        mean + y1 * std_deviation
    }

    fn get_gaussian_float(&mut self, mean: f32, std_deviation: f32) -> f32 {
        self.get_gaussian_double(f64::from(mean), f64::from(std_deviation)) as f32
    }

    fn get_gaussian_int(&mut self, mean: i32, std_deviation: i32) -> i32 {
        self.get_gaussian_double(f64::from(mean), f64::from(std_deviation))
            .round() as i32
    }

    /* Box-Muller, ranges */

    fn get_gaussian_double_range(&mut self, mut min: f64, mut max: f64) -> f64 {
        if max < min {
            std::mem::swap(&mut min, &mut max);
        }

        let mean = (min + max) / 2.0;
        let std_deviation = (max - min) / 6.0; /* 6.0 is used because of the three-sigma rule */

        self.get_gaussian_double(mean, std_deviation)
            .max(min)
            .min(max)
    }

    fn get_gaussian_float_range(&mut self, min: f32, max: f32) -> f32 {
        self.get_gaussian_double_range(f64::from(min), f64::from(max)) as f32
    }

    fn get_gaussian_int_range(&mut self, min: i32, max: i32) -> i32 {
        self.get_gaussian_double_range(f64::from(min), f64::from(max))
            .round() as i32
    }

    /* Box-Muller, ranges with a custom mean */

    fn get_gaussian_double_range_custom(&mut self, mut min: f64, mut max: f64, mean: f64) -> f64 {
        if max < min {
            std::mem::swap(&mut min, &mut max);
        }
        let d1 = max - mean;
        let d2 = mean - min;
        let std_deviation = d1.max(d2) / 3.0;

        self.get_gaussian_double(mean, std_deviation)
            .max(min)
            .min(max)
    }

    fn get_gaussian_float_range_custom(&mut self, min: f32, max: f32, mean: f32) -> f32 {
        self.get_gaussian_double_range_custom(f64::from(min), f64::from(max), f64::from(mean))
            as f32
    }

    fn get_gaussian_int_range_custom(&mut self, min: i32, max: i32, mean: i32) -> i32 {
        (self
            .get_gaussian_double_range_custom(f64::from(min), f64::from(max), f64::from(mean))
            .round() as i32)
            .max(min)
            .min(max)
    }

    /* Box-Muller, inverted distribution */

    fn get_gaussian_double_inv(&mut self, mean: f64, std_deviation: f64) -> f64 {
        let num = self.get_gaussian_double(mean, std_deviation);
        if num >= mean {
            num - 3.0 * std_deviation
        } else {
            num + 3.0 * std_deviation
        }
    }

    fn get_gaussian_float_inv(&mut self, mean: f32, std_deviation: f32) -> f32 {
        let num = self.get_gaussian_double(f64::from(mean), f64::from(std_deviation));
        if num >= f64::from(mean) {
            (num - 3.0 * f64::from(std_deviation)) as f32
        } else {
            (num + 3.0 * f64::from(std_deviation)) as f32
        }
    }

    fn get_gaussian_int_inv(&mut self, mean: i32, std_deviation: i32) -> i32 {
        let num = self.get_gaussian_double(f64::from(mean), f64::from(std_deviation));
        let integer = num.round() as i32;
        if num >= f64::from(mean) {
            (integer - 3 * std_deviation)
        } else {
            (integer + 3 * std_deviation)
        }
    }

    /* Box-Muller, ranges, inverted distribution */

    fn get_gaussian_double_range_inv(&mut self, mut min: f64, mut max: f64) -> f64 {
        if max < min {
            std::mem::swap(&mut min, &mut max);
        }
        let mean = (min + max) / 2.0;
        let std_deviation = (max - min) / 6.0; /* 6.0 is used because of the three-sigma rule */

        self.get_gaussian_double_inv(mean, std_deviation)
            .max(min)
            .min(max)
    }

    fn get_gaussian_float_range_inv(&mut self, min: f32, max: f32) -> f32 {
        self.get_gaussian_double_range_inv(f64::from(min), f64::from(max)) as f32
    }

    fn get_gaussian_int_range_inv(&mut self, min: i32, max: i32) -> i32 {
        (self
            .get_gaussian_double_range_inv(f64::from(min), f64::from(max))
            .round() as i32)
            .max(min)
            .min(max)
    }

    /* Box-Muller, ranges with a custom mean, inverted distribution */

    fn get_gaussian_double_range_custom_inv(
        &mut self,
        mut min: f64,
        mut max: f64,
        mean: f64,
    ) -> f64 {
        if max < min {
            std::mem::swap(&mut min, &mut max);
        }

        let d1 = max - mean;
        let d2 = mean - min;
        let std_deviation = d1.max(d2) / 3.0;

        self.get_gaussian_double_inv(mean, std_deviation)
            .max(min)
            .min(max)
    }

    fn get_gaussian_float_range_custom_inv(&mut self, min: f32, max: f32, mean: f32) -> f32 {
        self.get_gaussian_double_range_custom_inv(f64::from(min), f64::from(max), f64::from(mean))
            as f32
    }

    fn get_gaussian_int_range_custom_inv(&mut self, min: i32, max: i32, mean: i32) -> i32 {
        (self
            .get_gaussian_double_range_custom_inv(f64::from(min), f64::from(max), f64::from(mean))
            .round() as i32)
            .max(min)
            .min(max)
    }
}

impl<A: Algorithm> Rng for Random<A> {
    fn get_i32(&mut self, min: i32, max: i32) -> i32 {
        match self.distribution {
            Distribution::Linear => self.get_i(min, max),
            Distribution::Gaussian => self.get_gaussian_int(min, max),
            Distribution::GaussianRange => self.get_gaussian_int_range(min, max),
            Distribution::GaussianInverse => self.get_gaussian_int_inv(min, max),
            Distribution::GaussianRangeInverse => self.get_gaussian_int_range_inv(min, max),
        }
    }

    fn get_f32(&mut self, min: f32, max: f32) -> f32 {
        match self.distribution {
            Distribution::Linear => self.get_f(min, max),
            Distribution::Gaussian => self.get_gaussian_float(min, max),
            Distribution::GaussianRange => self.get_gaussian_float_range(min, max),
            Distribution::GaussianInverse => self.get_gaussian_float_inv(min, max),
            Distribution::GaussianRangeInverse => self.get_gaussian_float_range_inv(min, max),
        }
    }

    fn get_f64(&mut self, min: f64, max: f64) -> f64 {
        match self.distribution {
            Distribution::Linear => self.get_d(min, max),
            Distribution::Gaussian => self.get_gaussian_double(min, max),
            Distribution::GaussianRange => self.get_gaussian_double_range(min, max),
            Distribution::GaussianInverse => self.get_gaussian_double_inv(min, max),
            Distribution::GaussianRangeInverse => self.get_gaussian_double_range_inv(min, max),
        }
    }

    fn get_i32_mean(&mut self, min: i32, max: i32, mean: i32) -> i32 {
        match self.distribution {
            Distribution::GaussianInverse | Distribution::GaussianRangeInverse => {
                self.get_gaussian_int_range_custom_inv(min, max, mean)
            }
            _ => self.get_gaussian_int_range_custom(min, max, mean),
        }
    }

    fn get_f32_mean(&mut self, min: f32, max: f32, mean: f32) -> f32 {
        match self.distribution {
            Distribution::GaussianInverse | Distribution::GaussianRangeInverse => {
                self.get_gaussian_float_range_custom_inv(min, max, mean)
            }
            _ => self.get_gaussian_float_range_custom(min, max, mean),
        }
    }

    fn get_f64_mean(&mut self, min: f64, max: f64, mean: f64) -> f64 {
        match self.distribution {
            Distribution::GaussianInverse | Distribution::GaussianRangeInverse => {
                self.get_gaussian_double_range_custom_inv(min, max, mean)
            }
            _ => self.get_gaussian_double_range_custom(min, max, mean),
        }
    }
}

impl Random<MersenneTwister> {
    /// Returns a new `Random` using the Mersenne Twister algorithm.
    pub fn new_mt() -> Self {
        Self::new_mt_from_seed(Self::default_seed() as u32)
    }

    /// Returns a new `Random` using the Mersenne Twister algorithm, seeded with the given `seed`.
    pub fn new_mt_from_seed(seed: u32) -> Self {
        Self {
            algo: MersenneTwister::new(seed),
            distribution: Distribution::Linear,

            y2: None,
        }
    }
}

impl Random<ComplementaryMultiplyWithCarry> {
    /// Returns a new `Random` using the Complementary Multiply With Carry algorithm.
    pub fn new_cmwc() -> Self {
        Self::new_cmwc_from_seed(Self::default_seed() as u32)
    }

    /// Returns a new `Random` using the Complementary Multiply With Carry algorithm,
    /// seeded with the given `seed`.
    pub fn new_cmwc_from_seed(seed: u32) -> Self {
        Self {
            algo: ComplementaryMultiplyWithCarry::new(seed),
            distribution: Distribution::Linear,

            y2: None,
        }
    }
}

/// The distribution to use when generating random numbers
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum Distribution {
    /// Linear distribution; all numbers are equally likely.
    Linear,
    /// Gaussian distribution; uses a mean and standard deviation to generate numbers.
    Gaussian,
    /// Gaussian range distribution; uses the given min and max values to derive a mean and
    /// standard deviation for generating numbers.
    GaussianRange,
    /// Gaussian inverse distribution; uses a mean and standard deviation to generate numbers.
    GaussianInverse,
    /// Gaussian inverse range distribution; uses the given min and max values to derive a mean and
    /// standard deviation for generating numbers.
    GaussianRangeInverse,
}

/* string hashing function */
/* not used (yet)
fn hash(data: &[u8]) -> u32 {
    let mut hash: u32 = 0;
    for d in data {
        hash = (hash << 4) + *d as u32;
        let x = hash & 0xF000_0000;
        if x != 0 {
            hash ^= x >> 24;
            hash &= !x;
        }
    }

    hash & 0x7FFF_FFFF
}
*/

/// Represents a set of dice and rules for calculating their value when rolled
pub struct Dice {
    nb_rolls: i32,
    nb_faces: i32,
    multiplier: f32,
    add_sub: f32,
}

impl Dice {
    /// Create a new `Dice` with the given dice specification. The specification is as follows:
    /// `[mul*]<rolls>d<faces>[+/-offset]`, where
    /// * `rolls` number of dice is thrown,
    /// * these dice have `faces` number of faces,
    /// * once all the dice have been thrown, `offset` is added to their value,
    /// * and finally, that number is multiplied by `mul`.
    ///
    /// # Example
    /// ```
    /// # use rustcod::random::Dice;
    /// let dice = Dice::new("5*3d6+2");
    /// ```
    pub fn new<S: AsRef<str>>(s: S) -> Self {
        let mut s = s.as_ref();

        /* get multiplier */
        let multiplier = if let Some(m) = s.find(|c| c == '*' || c == 'x') {
            let value = s[0..m].parse::<f32>().unwrap_or_default();
            s = &s[m + 1..];

            value
        } else {
            1.0
        };

        /* get rolls */
        let r = s
            .find(|c| c == 'd' || c == 'D')
            .expect("Incorrect dice specification format");
        let nb_rolls = s[0..r].parse::<i32>().unwrap_or_default();
        s = &s[r + 1..];

        /* get faces */
        let nb_faces = if let Some(f) = s.find(|c| c == '+' || c == '-') {
            let value = s[0..f].parse::<i32>().unwrap_or_default();
            s = &s[f..];

            value
        } else {
            let value = s[0..].parse::<i32>().unwrap_or_default();
            s = &s[s.len()..];

            value
        };

        /* get add_sub */
        let add_sub = if s.is_empty() {
            0.0
        } else {
            s[0..].parse::<f32>().unwrap_or_default()
        };

        Self {
            multiplier,
            nb_rolls,
            nb_faces,
            add_sub,
        }
    }

    /// Roll the dice according to their parameters. See the documentation of `new()` for how these
    /// parameters get used.
    pub fn roll<R: Rng>(&self, mersenne: &mut R) -> i32 {
        let mut result = 0;
        for _ in 0..self.nb_rolls {
            result += mersenne.get_i32(1, self.nb_faces);
        }

        ((result as f32 + self.add_sub) * self.multiplier) as i32
    }

    /// Create a `Dice` and roll these dice once according to the given dice specification. See the
    /// documentation of `new()` for how this specification works. If you intend to use this dice
    /// set more than once, it's generally better to store the `Dice` instance and call `roll()`
    /// rather than to call this method over and over.
    pub fn single_roll<R: Rng, S: AsRef<str>>(mersenne: &mut R, s: S) -> i32 {
        Self::new(s).roll(mersenne)
    }
}

#[cfg(feature = "rng_support")]
impl<A: Algorithm> rand_core::RngCore for Random<A> {
    fn next_u32(&mut self) -> u32 {
        self.algo.get_int()
    }

    fn next_u64(&mut self) -> u64 {
        use rand_core::impls;
        impls::next_u64_via_u32(self)
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        use rand_core::impls;
        impls::fill_bytes_via_next(self, dest)
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

#[cfg(feature = "rng_support")]
impl rand::SeedableRng for Random<MersenneTwister> {
    type Seed = [u8; 4];

    fn from_seed(seed: Self::Seed) -> Self {
        let seed = (seed[0] as u32) << 24
            | (seed[1] as u32) << 16
            | (seed[2] as u32) << 8
            | (seed[3] as u32);
        Self::new_mt_from_seed(seed)
    }
}

#[cfg(feature = "rng_support")]
impl rand::SeedableRng for Random<ComplementaryMultiplyWithCarry> {
    type Seed = [u8; 4];

    fn from_seed(seed: Self::Seed) -> Self {
        let seed = (seed[0] as u32) << 24
            | (seed[1] as u32) << 16
            | (seed[2] as u32) << 8
            | (seed[3] as u32);
        Self::new_cmwc_from_seed(seed)
    }
}
