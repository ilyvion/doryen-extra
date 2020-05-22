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

//! # Height map generation.
//!
//! This module provides a way to create a 2D grid of float values using various algorithms.

use crate::noise::{Algorithm as NoiseAlgorithm, Noise};
use crate::random::{Algorithm as RandomAlgorithm, Random, Rng};
use crate::{FPosition, Position, UPosition};
use ilyvion_util::non_nan::NonNan;
use impl_ops::*;
use std::ops::{self, AddAssign, MulAssign};

/// A struct representing a height map.
#[derive(Clone, Debug)]
pub struct HeightMap {
    width: usize,
    height: usize,
    values: Vec<f32>,
}

impl HeightMap {
    /// Returns a new height map with the given width and height. Initially, all the values of the
    /// height map are `0.0`.
    ///
    /// # Panics
    ///
    /// If the `width` or the `height` is 0.
    pub fn new(width: usize, height: usize) -> Self {
        assert!(width > 0 && height > 0);

        Self {
            width,
            height,
            values: vec![0.0; width * height],
        }
    }

    /// Returns a new height map with the given width and height, and a set of values.
    ///
    /// # Panics
    ///
    /// * If the `width` or the `height` is 0.
    /// * If the length of `values` is not `width * height`.
    pub fn new_with_values(width: usize, height: usize, values: &[f32]) -> Self {
        assert!(width > 0 && height > 0);
        assert_eq!(values.len(), width * height);

        Self {
            width,
            height,
            values: values.to_vec(),
        }
    }

    /// Returns the width of the height map.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns the height of the height map.
    pub fn height(&self) -> usize {
        self.height
    }

    /// Returns the values of the height map.
    pub fn values(&self) -> &[f32] {
        &self.values
    }

    /// Returns the values of the height map.
    pub fn values_mut(&mut self) -> &mut [f32] {
        &mut self.values
    }

    /// Returns the value of the height map at the given position.
    ///
    /// # Panics
    ///
    /// If the position is outside the range of the height map.
    pub fn value(&self, position: UPosition) -> f32 {
        self.get_value(position.x as usize, position.y as usize)
    }

    /// Sets the value of the height map at the given position.
    ///
    /// # Panics
    ///
    /// If the position is outside the range of the height map.
    pub fn set_value(&mut self, position: UPosition, value: f32) {
        self.values[position.x as usize + position.y as usize * self.width] = value;
    }

    /// Interpolates the value of the height map at the given position.
    ///
    /// # Panics
    ///
    /// If the position is outside the range of the height map.
    pub fn interpolated_value(&self, position: FPosition) -> f32 {
        let i_position = position.truncate_u();
        if i_position.x as usize >= self.width - 1 || i_position.y as usize >= self.height - 1 {
            self.value(i_position)
        } else {
            let dx = position.x - i_position.x as f32;
            let dy = position.y - i_position.y as f32;
            let c1 = self.value(i_position);
            let c2 = self.value(i_position + (1, 0));
            let c3 = self.value(i_position + (0, 1));
            let c4 = self.value(i_position + (1, 1));
            let top = (1.0 - dx) * c1 + dx * c2;
            let bottom = (1.0 - dx) * c3 + dx * c4;

            (1.0 - dy) * top + dy * bottom
        }
    }

    /// Calculates the slope at the given position.
    ///
    /// # Panics
    ///
    /// If the position is outside the range of the height map.
    pub fn slope(&self, position: UPosition) -> f32 {
        const DIX: [i32; 8] = [-1, 0, 1, -1, 1, -1, 0, 1];
        const DIY: [i32; 8] = [-1, -1, -1, 0, 0, 1, 1, 1];

        let mut min_dy = 0.0;
        let mut max_dy = 0.0;
        let v = self.value(position);
        for (nx, ny) in Iterator::zip(DIX.iter(), DIY.iter())
            .map(|(&dx, &dy)| (position.x as i32 + dx, position.y as i32 + dy))
        {
            if nx >= 0 && nx < self.width as i32 && ny >= 0 && ny <= self.height as i32 {
                let n_slope = self.get_value(nx as usize, ny as usize) - v;
                if n_slope > max_dy {
                    max_dy = n_slope;
                } else if n_slope < min_dy {
                    min_dy = n_slope;
                }
            }
        }

        (max_dy + min_dy).atan2(1.0)
    }

    /// Calculates the normal at the given position.
    ///
    /// # Panics
    ///
    /// If the position is outside the range of the height map.
    pub fn normal(&self, position: FPosition, water_level: f32) -> [f32; 3] {
        let mut n = [0.0, 0.0, 1.0];

        if position.x >= self.width as f32 - 1.0 || position.y >= self.height as f32 - 1.0 {
            return n;
        }

        let mut h0 = self.interpolated_value(position);
        if h0 < water_level {
            h0 = water_level;
        }

        let mut hx = self.interpolated_value(position + (1.0, 0.0));
        if hx < water_level {
            hx = water_level;
        }

        let mut hy = self.interpolated_value(position + (0.0, 1.0));
        if hy < water_level {
            hy = water_level;
        }

        n[0] = 255.0 * (h0 - hx);
        n[1] = 255.0 * (h0 - hy);
        n[2] = 16.0;

        // normalize
        let inv_len = 1.0 / (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
        n[0] *= inv_len;
        n[1] *= inv_len;
        n[2] *= inv_len;

        n
    }

    /// Returns the number of cells that have a height between `min` and `max`, inclusive.
    pub fn count_cells(&self, min: f32, max: f32) -> usize {
        self.values
            .iter()
            .filter(|&&v| v >= min && v <= max)
            .count()
    }

    /// Returns whether there is any land along the edge of the height map. A result of `false`
    /// implies that the map is an island.
    pub fn has_land_on_border(&self, water_level: f32) -> bool {
        for x in 0..self.width {
            if self.get_value(x, 0) > water_level
                || self.get_value(x, self.height - 1) > water_level
            {
                return true;
            }
        }
        for y in 0..self.height {
            if self.get_value(0, y) > water_level || self.get_value(self.width - 1, y) > water_level
            {
                return true;
            }
        }

        false
    }

    /// Returns the lowest and highest height value in the height map.
    pub fn min_max(&self) -> MinMax {
        self.values
            .iter()
            .fold((std::f32::MAX, std::f32::MIN), |(min, max), &v| {
                (min.min(v), max.max(v))
            })
            .into()
    }

    /// Clamps the values in the height map to be between `min` and `max`, inclusive.
    ///
    /// # Panics
    ///
    /// If `max` > `min`.
    pub fn clamp(&mut self, min: f32, max: f32) {
        assert!(min <= max);

        self.values
            .iter_mut()
            .for_each(|v| *v = v.max(min).min(max))
    }

    /// Normalizes the values in the height map by scaling them proportionally such that the map's
    /// current smallest value will be set to `min`, and its largest value will be set to `max`, and
    /// all values in-between will be the same as they were, relative to these new end points.
    ///
    /// # Panics
    ///
    /// If `max` > `min`.
    ///
    /// # Examples
    /// ```
    /// # use doryen_extra::heightmap::HeightMap;
    /// let mut hm = HeightMap::new_with_values(2, 5,
    ///     &[-25.0, -15.0, -10.0, -5.0, 0.0, 10.0, 20.0, 30.0, 40.0, 50.0]);
    /// hm.normalize(-25.0, 20.0);
    /// assert_eq!(hm.values(), [
    ///     -25.0, -19.0, -16.0, -13.0, -10.0, -4.0, 2.0, 8.0, 14.0, 20.0,
    /// ]);
    /// ```
    pub fn normalize(&mut self, min: f32, max: f32) {
        assert!(min <= max);

        let MinMax {
            min: cur_min,
            max: cur_max,
        } = self.min_max();

        let inv_max = if cur_max - cur_min == 0.0 {
            0.0
        } else {
            (f64::from(max) - f64::from(min)) / (f64::from(cur_max) - f64::from(cur_min))
        };

        // normalize
        self.values.iter_mut().for_each(|v| {
            *v = (f64::from(min) + (f64::from(*v) - f64::from(cur_min)) * inv_max) as f32
        });
    }

    /// Resets all the values in the height map to `0.0`.
    pub fn clear(&mut self) {
        for v in &mut self.values {
            *v = 0.0;
        }
    }

    /// Linearly interpolate two height maps together.
    pub fn lerp(&self, other: &HeightMap, coefficient: f32) -> HeightMap {
        assert_eq!(self.width, other.width);
        assert_eq!(self.height, other.height);
        assert!(coefficient >= 0.0 && coefficient <= 1.0);

        let mut result = HeightMap::new(self.width, self.height);
        for (v, (&sv, &ov)) in result
            .values
            .iter_mut()
            .zip(Iterator::zip(self.values.iter(), other.values.iter()))
        {
            *v = sv + (ov - sv) * coefficient;
        }

        result
    }

    /// Adds a hill (a half spheroid) at the given position, with a `radius` and a `height`.
    /// If `height == radius` or `-radius`, the hill will be a half-sphere.
    pub fn add_hill(&mut self, position: FPosition, radius: f32, height: f32) {
        let radius2 = radius * radius;
        let coefficient = height / radius2;

        let min_x = (position.x - radius).max(0.0) as usize;
        let max_x = (position.x + radius).min(self.width as f32) as usize;
        let min_y = (position.y - radius).max(0.0) as usize;
        let max_y = (position.y + radius).min(self.height as f32) as usize;

        for x in min_x..max_x {
            let x_dist = (x as f32 - position.x) * (x as f32 - position.x);
            for y in min_y..max_y {
                let z = radius2 - x_dist - (y as f32 - position.y) * (y as f32 - position.y);
                if z > 0.0 {
                    *self.get_value_mut(x, y) += z * coefficient;
                }
            }
        }
    }

    /// Takes the highest value (if `height > 0`) or the lowest (if `height < 0`) between the map
    /// and the hill. Its main goal is to carve things into maps (like rivers) by digging hills
    /// along a curve.
    pub fn dig_hill(&mut self, position: FPosition, radius: f32, height: f32) {
        let radius2 = radius * radius;
        let coefficient = height / radius2;

        let min_x = (position.x - radius).max(0.0) as usize;
        let max_x = (position.x + radius).min(self.width as f32) as usize;
        let min_y = (position.y - radius).max(0.0) as usize;
        let max_y = (position.y + radius).min(self.height as f32) as usize;

        for x in min_x..max_x {
            let x_dist = (x as f32 - position.x) * (x as f32 - position.x);
            for y in min_y..max_y {
                let dist = x_dist + (y as f32 - position.y) * (y as f32 - position.y);
                if dist < radius2 {
                    let z = (radius2 - dist) * coefficient;
                    let value = self.get_value_mut(x, y);
                    if height > 0.0 {
                        if *value < z {
                            *value = z;
                        }
                    } else if *value > 0.0 {
                        *value = z;
                    }
                }
            }
        }
    }

    /// Carves a path along a cubic Bezier curve using the `dig_hill` method. Could be used for
    /// generating roads, rivers, etc. Both radius and depth can vary linearly along the path. The
    /// four `positions` are the 4 Bezier control points.
    pub fn dig_bezier(
        &mut self,
        positions: [UPosition; 4],
        start_radius: f32,
        start_depth: f32,
        end_radius: f32,
        end_depth: f32,
    ) {
        let mut x_from = positions[0].x as usize;
        let mut y_from = positions[0].y as usize;

        let mut t = 0.0_f32;
        while t <= 1.0 {
            let it = 1.0 - t;

            let x_to = (positions[0].x as f32 * it * it * it
                + 3.0 * positions[1].x as f32 * t * it * it
                + 3.0 * positions[2].x as f32 * t * t * it
                + positions[3].x as f32 * t * t * t) as usize;
            let y_to = (positions[0].y as f32 * it * it * it
                + 3.0 * positions[1].y as f32 * t * it * it
                + 3.0 * positions[2].y as f32 * t * t * it
                + positions[3].y as f32 * t * t * t) as usize;

            if x_to != x_from || y_to != y_from {
                let radius = start_radius + (end_radius - start_radius) * t;
                let depth = start_depth + (end_depth - start_depth) * t;
                self.dig_hill((x_to as f32, y_to as f32).into(), radius, depth);
                x_from = x_to;
                y_from = y_to;
            }

            t += 0.001;
        }
    }

    /// Simulates the effect of rain drops on the terrain, resulting in erosion patterns.
    ///
    /// # Parameters
    /// * `drops` - The number of rain drops to simulate. Should be at least `width * height`.
    /// * `erosion_coefficient` - The amount of ground eroded on the drop's path.
    /// * `aggregation_coefficient` - The amount of ground deposited when the drops stops to flow.
    /// * `random` - The random number generator to use.
    pub fn rain_erosion<A: RandomAlgorithm>(
        &mut self,
        mut drops: u32,
        erosion_coefficient: f32,
        aggregation_coefficient: f32,
        random: &mut Random<A>,
    ) {
        const DX: [i32; 8] = [-1, 0, 1, -1, 1, -1, 0, 1];
        const DY: [i32; 8] = [-1, -1, -1, 0, 0, 1, 1, 1];

        while drops > 0 {
            let mut cur_x = random.get_i32(0, (self.width - 1) as i32);
            let mut cur_y = random.get_i32(0, (self.height - 1) as i32);
            let mut slope;
            let mut sediment = 0.0;

            loop {
                let mut next_x = 0;
                let mut next_y = 0;
                let v = self.get_value(cur_x as usize, cur_y as usize);
                slope = 0.0;
                for (nx, ny) in
                    Iterator::zip(DX.iter(), DY.iter()).map(|(&dx, &dy)| (cur_x + dx, cur_y + dy))
                {
                    if nx >= 0 && nx < self.width as i32 && ny >= 0 && ny < self.height as i32 {
                        let n_slope = v - self.get_value(nx as usize, ny as usize);
                        if n_slope > slope {
                            slope = n_slope;
                            next_x = nx;
                            next_y = ny;
                        }
                    }
                }
                if slope > 0.0 {
                    *self.get_value_mut(cur_x as usize, cur_y as usize) -=
                        erosion_coefficient * slope;
                    cur_x = next_x;
                    cur_y = next_y;
                    sediment += slope;
                } else {
                    *self.get_value_mut(cur_x as usize, cur_y as usize) +=
                        aggregation_coefficient * sediment;
                }

                if slope <= 0.0 {
                    break;
                }
            }
            drops -= 1;
        }
    }

    /// Apply a generic transformation on the height map, so that each resulting cell value is the
    /// weighted sum of several neighbour cells. This can be used to, e.g. smooth/sharpen the map.
    ///
    /// # Examples
    /// Do simple horizontal smoothing with direct neighbor cells.
    /// ```
    /// # use doryen_extra::Position;
    /// # use doryen_extra::heightmap::{HeightMap, NeighborCell};
    /// let mut hm =
    ///     HeightMap::new_with_values(3, 3, &[3.0, 6.0, 9.0, 12.0, 15.0, 18.0, 21.0, 24.0, 27.0]);
    /// let cells = [
    ///     NeighborCell { relative_position: Position::new(-1, 0), weight: 0.33 },
    ///     NeighborCell { relative_position: Position::new(0, 0), weight: 0.33 },
    ///     NeighborCell { relative_position: Position::new(1, 0), weight: 0.33 },
    /// ];
    /// hm.kernel_transform(&cells, 0.0, 100.0);
    /// assert_eq!(hm.values(), &[4.5, 6.5, 7.75, 13.5, 15.5, 16.75, 22.5, 24.5, 25.75])
    /// ```
    pub fn kernel_transform(&mut self, cells: &[NeighborCell], min_level: f32, max_level: f32) {
        for x in 0..self.width {
            let mut offset = x;
            for y in 0..self.height {
                if self.values[offset] >= min_level && self.values[offset] <= max_level {
                    let mut val = 0.0;
                    let mut total_weight = 0.0;
                    for cell in cells {
                        let nx = x as i32 + cell.relative_position.x;
                        let ny = y as i32 + cell.relative_position.y;
                        if nx >= 0 && nx < self.width as i32 && ny >= 0 && ny < self.height as i32 {
                            val += f64::from(cell.weight)
                                * f64::from(self.get_value(nx as usize, ny as usize));
                            total_weight += f64::from(cell.weight);
                        }
                    }
                    self.values[offset] = (val / total_weight) as f32;
                }
                offset += self.width;
            }
        }
    }

    /// Adds values from a Voronoi diagram to the height map.
    pub fn add_voronoi<A: RandomAlgorithm>(
        &mut self,
        sites: usize,
        coefficients: &[f32],
        random: &mut Random<A>,
    ) {
        struct Point {
            x: i32,
            y: i32,
            dist: NonNan<f32>,
        }

        assert!(sites >= coefficients.len());

        let mut points = Vec::with_capacity(sites);
        for _ in 0..sites {
            points.push(Point {
                x: random.get_i32(0, (self.width - 1) as i32),
                y: random.get_i32(0, (self.height - 1) as i32),
                dist: 0.0.into(),
            });
        }
        for x in 0..self.width {
            let mut offset = x;
            for y in 0..self.height {
                // calculate distance to voronoi points
                for point in &mut points {
                    point.dist = ((point.x - x as i32) as f32 * (point.x - x as i32) as f32
                        + (point.y - y as i32) as f32 * (point.y - y as i32) as f32)
                        .into();
                }
                for coefficient in coefficients {
                    let min_dist_point = points.iter_mut().min_by_key(|p| p.dist).unwrap();
                    self.values[offset] += coefficient * *min_dist_point.dist;
                    min_dist_point.dist = std::f32::MAX.into();
                }
                offset += self.width;
            }
        }
    }

    /// Generates a height map with mid-point displacement.
    ///
    /// The mid-point displacement algorithm generates a realistic fractal height map using the
    /// diamond-square (aka random midpoint displacement) algorithm.
    ///
    /// The roughness range should be comprised between `0.4` and `0.6`.
    ///
    /// # Panics
    ///
    /// If the `width` or the `height` is 0.
    pub fn mid_point_displacement<A: RandomAlgorithm>(
        &mut self,
        random: &mut Random<A>,
        roughness: f32,
    ) {
        let mut step = 1;
        let mut offset = 1.0;
        let init_sz = self.width.min(self.height);
        let mut sz = init_sz;
        self.values[0] = random.get_f32(0.0, 1.0);
        self.values[sz - 1] = random.get_f32(0.0, 1.0);
        self.values[(sz - 1) * sz] = random.get_f32(0.0, 1.0);
        self.values[sz * sz - 1] = random.get_f32(0.0, 1.0);
        while sz > 0 {
            // diamond step
            for x in 0..step {
                for y in 0..step {
                    let diamond_x = sz / 2 + x * sz;
                    let diamond_y = sz / 2 + y * sz;

                    let mut z = self.get_value(x * sz, y * sz);
                    z += self.get_value((x + 1) * sz, y * sz);
                    z += self.get_value((x + 1) * sz, (y + 1) * sz);
                    z += self.get_value(x * sz, (y + 1) * sz);
                    z *= 0.25;

                    self.set_mpd_height(random, diamond_x, diamond_y, z, offset);
                }
            }
            offset *= roughness;

            // square step
            for x in 0..step {
                for y in 0..step {
                    let diamond_x = sz / 2 + x * sz;
                    let diamond_y = sz / 2 + y * sz;

                    // north
                    self.set_mdp_height_square(
                        random,
                        diamond_x,
                        diamond_y - sz / 2,
                        init_sz,
                        sz / 2,
                        offset,
                    );
                    // south
                    self.set_mdp_height_square(
                        random,
                        diamond_x,
                        diamond_y + sz / 2,
                        init_sz,
                        sz / 2,
                        offset,
                    );
                    // west
                    self.set_mdp_height_square(
                        random,
                        diamond_x - sz / 2,
                        diamond_y,
                        init_sz,
                        sz / 2,
                        offset,
                    );
                    // east
                    self.set_mdp_height_square(
                        random,
                        diamond_x + sz / 2,
                        diamond_y,
                        init_sz,
                        sz / 2,
                        offset,
                    );
                }
            }
            sz /= 2;
            step *= 2;
        }
    }

    /// Add an FBM to the height map.
    ///
    /// The noise value for map cell `(x, y)` is `(x + add_x) * mul_x / width` and
    /// `(y + add_y) * mul_y / height`, respectively. Those values allow you to scale and translate
    /// the noise function over the height map.
    ///
    /// # Panics
    ///
    /// If the `noise` provided isn't 2D.
    pub fn add_fbm<A: NoiseAlgorithm>(
        &mut self,
        noise: &mut Noise<A>,
        octaves: f32,
        coordinates: FbmCoordinateParameters,
        delta: f32,
        scale: f32,
    ) {
        assert_eq!(
            noise.dimensions, 2,
            "add_fbm requires a 2D noise generator."
        );

        let x_coefficient = coordinates.mul_x / self.width as f32;
        let y_coefficient = coordinates.mul_y / self.height as f32;

        for x in 0..self.width {
            let mut f = [0.0; 2];
            let mut offset = x;
            f[0] = (x as f32 + coordinates.add_x) * x_coefficient;
            for y in 0..self.height {
                f[1] = (y as f32 + coordinates.add_y) * y_coefficient;
                let value = delta + noise.fbm(&f, octaves) * scale;
                self.values[offset] += value;
                offset += self.width;
            }
        }
    }

    /// Scale the map by an FBM.
    ///
    /// The noise coordinate for map cell `(x, y)` is `(x + add_x) * mul_x / width` and
    /// `(y + add_y) * mul_y / height`, respectively. Those values allow you to scale and translate
    /// the noise function over the height map.
    ///
    /// The value multiplied with the height map is `delta + noise * scale`.
    ///
    /// # Panics
    ///
    /// If the `noise` generator provided isn't 2D.
    pub fn scale_fbm<A: NoiseAlgorithm>(
        &mut self,
        noise: &mut Noise<A>,
        coordinates: FbmCoordinateParameters,
        octaves: f32,
        delta: f32,
        scale: f32,
    ) {
        assert_eq!(
            noise.dimensions, 2,
            "scale_fbm requires a 2D noise generator."
        );

        let x_coefficient = coordinates.mul_x / self.width as f32;
        let y_coefficient = coordinates.mul_y / self.height as f32;

        for x in 0..self.width {
            let mut f = [0.0; 2];
            let mut offset = x;
            f[0] = (x as f32 + coordinates.add_x) * x_coefficient;
            for y in 0..self.height {
                f[1] = (y as f32 + coordinates.add_y) * y_coefficient;
                let value = delta + noise.fbm(&f, octaves) * scale;
                self.values[offset] *= value;
                offset += self.width;
            }
        }
    }

    #[inline]
    fn get_value(&self, x: usize, y: usize) -> f32 {
        assert!(x < self.width);
        assert!(y < self.height);

        self.values[x + y * self.width]
    }

    #[inline]
    fn get_value_mut(&mut self, x: usize, y: usize) -> &mut f32 {
        assert!(x < self.width);
        assert!(y < self.height);

        &mut self.values[x + y * self.width]
    }

    fn set_mdp_height_square<A: RandomAlgorithm>(
        &mut self,
        random: &mut Random<A>,
        x: usize,
        y: usize,
        init_sz: usize,
        sz: usize,
        offset: f32,
    ) {
        let mut z = 0.0;
        let mut count = 0;
        if y >= sz {
            z += self.get_value(x, y - sz);
            count += 1;
        }
        if x >= sz {
            z += self.get_value(x - sz, y);
            count += 1;
        }
        if y + sz < init_sz {
            z += self.get_value(x, y + sz);
            count += 1;
        }
        if x + sz < init_sz {
            z += self.get_value(x + sz, y);
            count += 1;
        }
        z /= count as f32;
        self.set_mpd_height(random, x, y, z, offset);
    }

    fn set_mpd_height<A: RandomAlgorithm>(
        &mut self,
        random: &mut Random<A>,
        x: usize,
        y: usize,
        mut z: f32,
        offset: f32,
    ) {
        z += random.get_f32(-offset, offset);
        *self.get_value_mut(x, y) = z;
    }
}

impl_op_ex!(+ |a: &HeightMap, b: &HeightMap| -> HeightMap {
    assert_eq!(a.width, b.width);
    assert_eq!(a.height, b.height);

    let mut result = a.clone();
    for (r, &o) in result.values.iter_mut().zip(b.values.iter()) {
        *r += o
    }

    result
});

impl AddAssign<f32> for HeightMap {
    fn add_assign(&mut self, rhs: f32) {
        self.values.iter_mut().for_each(|v| *v += rhs);
    }
}

impl_op_ex!(*|a: &HeightMap, b: &HeightMap| -> HeightMap {
    assert_eq!(a.width, b.width);
    assert_eq!(a.height, b.height);

    let mut result = a.clone();
    for (r, &o) in result.values.iter_mut().zip(b.values.iter()) {
        *r *= o
    }

    result
});

impl MulAssign<f32> for HeightMap {
    fn mul_assign(&mut self, rhs: f32) {
        self.values.iter_mut().for_each(|v| *v *= rhs);
    }
}

/// Represents a result of minimum and maximum values in a height map.
#[derive(Copy, Clone, Debug)]
pub struct MinMax {
    /// The minimum value.
    pub min: f32,
    /// The maximum value.
    pub max: f32,
}

impl From<(f32, f32)> for MinMax {
    fn from((min, max): (f32, f32)) -> Self {
        Self { min, max }
    }
}

/// Represents a neighbor cell in the kernel transformation method.
#[derive(Copy, Clone, Debug)]
pub struct NeighborCell {
    /// Which map cell this transform takes its value from, relative to the current cell. That is to
    /// say, if the transform is currently working on cell (5, 2) and this field is (-1, 1), it will
    /// get its value from (4, 3).
    pub relative_position: Position,

    /// What to scale this neighbor cell's value by when calculating the new value of the
    /// current cell.
    pub weight: f32,
}

/// Represents the coordinates used in the `*_fbm` methods.
#[derive(Copy, Clone, Debug)]
pub struct FbmCoordinateParameters {
    /// See the `*_fbm` methods for details on how this parameter is used.
    pub mul_x: f32,
    /// See the `*_fbm` methods for details on how this parameter is used.
    pub mul_y: f32,
    /// See the `*_fbm` methods for details on how this parameter is used.
    pub add_x: f32,
    /// See the `*_fbm` methods for details on how this parameter is used.
    pub add_y: f32,
}
