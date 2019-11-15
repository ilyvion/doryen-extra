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

//! 24 bit color operations.

use crate::util::FloorRem;
use std::ops::{Add, Mul, Sub};

/// A struct representing a 24-bit RGB color
#[derive(Copy, Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    /// Returns a new Color from RGB values.
    ///
    /// # Parameters
    /// * `r` - The color's amount of red.
    /// * `g` - The color's amount of green.
    /// * `b` - The color's amount of blue.
    ///
    /// # Example
    /// ```
    /// # use rustcod::color::Color;
    /// let white = Color::new(255, 255, 255);
    /// ```
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Returns a new Color from HSV values.
    ///
    /// The saturation and value parameters are automatically clamped to 0 and 1.
    ///
    /// Use `set_hsv()` to fill an existing struct with HSV values.
    ///
    /// # Parameters
    /// * `hue` - The color's hue in degrees.
    /// * `saturation` - The color's saturation, from 0 to 1.
    /// * `value` - The color's value, from 0 to 1.
    ///
    /// # Example
    /// ```
    /// # use rustcod::color::Color;
    /// let light_blue = Color::new_hsv(240.0, 0.75, 1.0);
    /// ```
    pub fn new_hsv(hue: f32, saturation: f32, value: f32) -> Self {
        let mut color = Color::new(0, 0, 0);
        color.set_hsv(hue, saturation, value);

        color
    }

    /// Sets a colors values from HSV values.
    ///
    /// # Parameters
    /// * `hue` - The color's hue in degrees.
    /// * `saturation` - The color's saturation, from 0 to 1.
    /// * `value` - The color's value, from 0 to 1.
    pub fn set_hsv(&mut self, hue: f32, saturation: f32, value: f32) {
        let saturation = saturation.max(0.0).min(1.0);
        let value = value.max(0.0).min(1.0);

        if saturation == 0.0 {
            /* achromatic (grey) */
            let value = (value * 255.0).round() as u8;
            self.r = value;
            self.g = value;
            self.b = value;
            return;
        }

        let mut hue = hue.floor_modulo(360.0);
        hue /= 60.0; /* sector 0 to 5 */
        let hue_section = hue.floor() as i32;
        let hue_fraction = hue - hue_section as f32;

        let p = ((value * (1.0 - saturation)) * 255.0).round() as u8;
        let q = ((value * (1.0 - saturation * hue_fraction)) * 255.0).round() as u8;
        let t = ((value * (1.0 - saturation * (1.0 - hue_fraction))) * 255.0).round() as u8;
        let v = (value * 255.0).round() as u8;

        let (r, g, b) = match hue_section {
            0 => (v, t, p),
            1 => (q, v, p),
            2 => (p, v, t),
            3 => (p, q, v),
            4 => (t, p, v),
            _ => (v, p, q),
        };
        self.r = r;
        self.g = g;
        self.b = b;
    }

    /// Get a tuple of HSV values from a color.
    pub fn get_hsv(self) -> (f32, f32, f32) {
        let hue = self.get_hue();
        let saturation = self.get_saturation();
        let value = self.get_value();

        (hue, saturation, value)
    }

    /// Change a color's hue.
    ///
    /// # Parameters
    /// * `hue` - The color's hue in degrees.
    pub fn set_hue(&mut self, hue: f32) {
        let saturation = self.get_saturation();
        let value = self.get_value();

        self.set_hsv(hue, saturation, value);
    }

    /// Return a color's hue in degrees.
    pub fn get_hue(self) -> f32 {
        let max = self.r.max(self.g).max(self.b);
        let min = self.r.min(self.g).min(self.b);
        let delta = max as f32 - min as f32;
        if delta == 0.0 {
            return 0.0;
        }

        let mut hue = if self.r == max {
            (self.g as f32 - self.b as f32) / delta
        } else if self.g == max {
            2.0 + (self.b as f32 - self.r as f32) / delta
        } else {
            4.0 + (self.r as f32 - self.g as f32) / delta
        };
        hue *= 60.0;
        hue.floor_modulo(360.0)
    }

    /// Returns a color's saturation in the range \[0, 1\].
    pub fn get_saturation(self) -> f32 {
        let max = self.r.max(self.g).max(self.b);
        let min = self.r.min(self.g).min(self.b);
        let delta = max as f32 - min as f32;
        if max == 0 {
            0.0
        } else {
            delta / max as f32
        }
    }

    /// Change a color's saturation.
    ///
    /// # Parameters
    /// * `saturation` - The color's saturation, from 0 to 1.
    pub fn set_saturation(&mut self, saturation: f32) {
        let hue = self.get_hue();
        let value = self.get_value();

        self.set_hsv(hue, saturation, value);
    }

    /// Returns a color's value in the range \[0, 1\].
    pub fn get_value(self) -> f32 {
        self.r.max(self.g).max(self.b) as f32 / 255.0
    }

    /// Change a color's value.
    ///
    /// # Parameters
    /// * `value` - The color's value, from 0 to 1.
    pub fn set_value(&mut self, value: f32) {
        let hue = self.get_hue();
        let saturation = self.get_saturation();

        self.set_hsv(hue, saturation, value);
    }

    /// Shift a color's hue by an amount.
    ///
    /// # Parameters
    /// * `hue_shift` - The distance to shift the hue, in degrees.
    pub fn shift_hue(&mut self, hue_shift: f32) {
        if hue_shift == 0.0 {
            return;
        }
        self.set_hsv(
            self.get_hue() + hue_shift,
            self.get_saturation(),
            self.get_value(),
        );
    }

    /// Scale a color's saturation and value.
    ///
    /// # Parameters
    /// * `saturation_coefficient` - Multiplier for this color's saturation.
    /// * `value_coefficient` - Multiplier for this color's value.
    pub fn scale_hsv(&mut self, saturation_coefficient: f32, value_coefficient: f32) {
        if (saturation_coefficient - 1.0).abs() < 0.001 && (value_coefficient - 1.0).abs() < 0.001 {
            return;
        }
        self.set_hsv(
            self.get_hue(),
            self.get_saturation() * saturation_coefficient,
            self.get_value() * value_coefficient,
        );
    }

    /// Generates an interpolated gradient of colors using RGB interpolation.
    ///
    /// Using RGB interpolation between colors is almost always the wrong choice and tends to
    /// produce really ugly results. You almost certainly don't want to use this; use
    /// `generate_gradient_hsv()` instead.
    ///
    /// # Parameters
    /// * `key_colors` -  The colors to make gradients between.
    /// * `gradient_spans` -  How many interpolated colors to generate between each
    /// pair of key colors.
    ///
    /// # Panics
    /// * If `gradient_spans`' length isn't one less than `key_colors`' length.
    ///
    /// # Example
    /// ```
    /// # use rustcod::color::Color;
    /// // Generates every grayscale color between black and white
    /// let grayscale = Color::generate_gradient_rgb(&[Color::BLACK,Color::WHITE], &[254]);
    /// ```
    pub fn generate_gradient_rgb(key_colors: &[Color], gradient_spans: &[usize]) -> Vec<Color> {
        if key_colors.is_empty() {
            return vec![];
        }

        assert_eq!(
            key_colors.len() - 1,
            gradient_spans.len(),
            "gradient_spans should have one fewer values in it than key_colors"
        );

        let mut result =
            Vec::with_capacity(key_colors.len() + gradient_spans.iter().sum::<usize>());
        for (span, colors) in key_colors.windows(2).enumerate() {
            let start_color = colors[0];
            let end_color = colors[1];
            for s in 0..=gradient_spans[span] {
                let coefficient = s as f32 / (gradient_spans[span] + 1) as f32;
                result.push(start_color.lerp_rgb(end_color, coefficient));
            }
        }
        result.push(*key_colors.last().unwrap());

        result
    }

    /// Generates an interpolated gradient of colors using HSV interpolation.
    ///
    /// # Parameters
    /// * `key_colors` -  The colors to make gradients between.
    /// * `gradient_spans` -  How many interpolated colors to generate between each
    /// pair of key colors.
    ///
    /// # Panics
    /// * If `gradient_spans`' length isn't one less than `key_colors`' length.
    ///
    /// # Example
    /// ```
    /// # use rustcod::color::Color;
    /// // Generates every grayscale color between black and white
    /// let grayscale = Color::generate_gradient_hsv(&[Color::BLACK,Color::WHITE], &[254]);
    /// ```
    pub fn generate_gradient_hsv(key_colors: &[Color], gradient_spans: &[usize]) -> Vec<Color> {
        if key_colors.is_empty() {
            return vec![];
        }

        assert_eq!(
            key_colors.len() - 1,
            gradient_spans.len(),
            "gradient_spans should have one fewer values in it than key_colors"
        );

        let mut result =
            Vec::with_capacity(key_colors.len() + gradient_spans.iter().sum::<usize>());
        for (span, colors) in key_colors.windows(2).enumerate() {
            let start_color = colors[0];
            let end_color = colors[1];
            for s in 0..=gradient_spans[span] {
                let coefficient = s as f32 / (gradient_spans[span] + 1) as f32;
                result.push(start_color.lerp_hsv(end_color, coefficient));
            }
        }
        result.push(*key_colors.last().unwrap());

        result
    }

    /// Interpolate two colors together using their RGB representation and return the result.
    ///
    /// You almost certainly don't want to use this; use `lerp_hsv()` instead.
    ///
    /// # Parameters
    /// * `other` - The second color.
    /// * `coefficient` - The coefficient. 0 for entirely the first color, 1 for entirely the second.
    ///
    /// # Panics
    ///
    /// If `coefficient` is outside the range \[0, 1\].
    pub fn lerp_rgb(self, other: Color, coefficient: f32) -> Color {
        assert!(
            coefficient >= 0.0 && coefficient <= 1.0,
            "coefficient is outside the acceptable range [0, 1]"
        );

        Self::new(
            ((self.r as f32) + (other.r as f32 - self.r as f32) * coefficient) as u8,
            ((self.g as f32) + (other.g as f32 - self.g as f32) * coefficient) as u8,
            ((self.b as f32) + (other.b as f32 - self.b as f32) * coefficient) as u8,
        )
    }

    /// Interpolate two colors together using their HSV representation and return the result.
    ///
    /// # Parameters
    /// * `other` - The second color.
    /// * `coefficient` - The coefficient. 0 for entirely the first color, 1 for entirely the second.
    ///
    /// # Panics
    ///
    /// If `coefficient` is outside the range \[0, 1\].
    pub fn lerp_hsv(self, other: Color, coefficient: f32) -> Color {
        assert!(
            coefficient >= 0.0 && coefficient <= 1.0,
            "coefficient is outside the acceptable range [0, 1]"
        );
        let (self_hue, self_saturation, self_value) = self.get_hsv();
        let (other_hue, other_saturation, other_value) = other.get_hsv();

        let hue_diff = other_hue - self_hue;
        let hue_delta = hue_diff
            + if hue_diff.abs() > 180.0 {
                if hue_diff < 0.0 {
                    360.0
                } else {
                    -360.0
                }
            } else {
                0.0
            };

        let hue_interpolated = self_hue + coefficient * hue_delta;

        Self::new_hsv(
            hue_interpolated,
            self_saturation + (other_saturation - self_saturation) * coefficient,
            self_value + (other_value - self_value) * coefficient,
        )
    }
}

// Enums-to-color
impl Color {
    pub fn by_name_and_level(name: Name, level: Level) -> Self {
        match name {
            Name::Red => match level {
                Level::Desaturated => Self::DESATURATED_RED,
                Level::Lightest => Self::LIGHTEST_RED,
                Level::Lighter => Self::LIGHTER_RED,
                Level::Light => Self::LIGHT_RED,
                Level::Normal => Self::RED,
                Level::Dark => Self::DARK_RED,
                Level::Darker => Self::DARKER_RED,
                Level::Darkest => Self::DARKEST_RED,
            },
            Name::Flame => match level {
                Level::Desaturated => Self::DESATURATED_FLAME,
                Level::Lightest => Self::LIGHTEST_FLAME,
                Level::Lighter => Self::LIGHTER_FLAME,
                Level::Light => Self::LIGHT_FLAME,
                Level::Normal => Self::FLAME,
                Level::Dark => Self::DARK_FLAME,
                Level::Darker => Self::DARKER_FLAME,
                Level::Darkest => Self::DARKEST_FLAME,
            },
            Name::Orange => match level {
                Level::Desaturated => Self::DESATURATED_ORANGE,
                Level::Lightest => Self::LIGHTEST_ORANGE,
                Level::Lighter => Self::LIGHTER_ORANGE,
                Level::Light => Self::LIGHT_ORANGE,
                Level::Normal => Self::ORANGE,
                Level::Dark => Self::DARK_ORANGE,
                Level::Darker => Self::DARKER_ORANGE,
                Level::Darkest => Self::DARKEST_ORANGE,
            },
            Name::Amber => match level {
                Level::Desaturated => Self::DESATURATED_AMBER,
                Level::Lightest => Self::LIGHTEST_AMBER,
                Level::Lighter => Self::LIGHTER_AMBER,
                Level::Light => Self::LIGHT_AMBER,
                Level::Normal => Self::AMBER,
                Level::Dark => Self::DARK_AMBER,
                Level::Darker => Self::DARKER_AMBER,
                Level::Darkest => Self::DARKEST_AMBER,
            },
            Name::Yellow => match level {
                Level::Desaturated => Self::DESATURATED_YELLOW,
                Level::Lightest => Self::LIGHTEST_YELLOW,
                Level::Lighter => Self::LIGHTER_YELLOW,
                Level::Light => Self::LIGHT_YELLOW,
                Level::Normal => Self::YELLOW,
                Level::Dark => Self::DARK_YELLOW,
                Level::Darker => Self::DARKER_YELLOW,
                Level::Darkest => Self::DARKEST_YELLOW,
            },
            Name::Lime => match level {
                Level::Desaturated => Self::DESATURATED_LIME,
                Level::Lightest => Self::LIGHTEST_LIME,
                Level::Lighter => Self::LIGHTER_LIME,
                Level::Light => Self::LIGHT_LIME,
                Level::Normal => Self::LIME,
                Level::Dark => Self::DARK_LIME,
                Level::Darker => Self::DARKER_LIME,
                Level::Darkest => Self::DARKEST_LIME,
            },
            Name::Chartreuse => match level {
                Level::Desaturated => Self::DESATURATED_CHARTREUSE,
                Level::Lightest => Self::LIGHTEST_CHARTREUSE,
                Level::Lighter => Self::LIGHTER_CHARTREUSE,
                Level::Light => Self::LIGHT_CHARTREUSE,
                Level::Normal => Self::CHARTREUSE,
                Level::Dark => Self::DARK_CHARTREUSE,
                Level::Darker => Self::DARKER_CHARTREUSE,
                Level::Darkest => Self::DARKEST_CHARTREUSE,
            },
            Name::Green => match level {
                Level::Desaturated => Self::DESATURATED_GREEN,
                Level::Lightest => Self::LIGHTEST_GREEN,
                Level::Lighter => Self::LIGHTER_GREEN,
                Level::Light => Self::LIGHT_GREEN,
                Level::Normal => Self::GREEN,
                Level::Dark => Self::DARK_GREEN,
                Level::Darker => Self::DARKER_GREEN,
                Level::Darkest => Self::DARKEST_GREEN,
            },
            Name::Sea => match level {
                Level::Desaturated => Self::DESATURATED_SEA,
                Level::Lightest => Self::LIGHTEST_SEA,
                Level::Lighter => Self::LIGHTER_SEA,
                Level::Light => Self::LIGHT_SEA,
                Level::Normal => Self::SEA,
                Level::Dark => Self::DARK_SEA,
                Level::Darker => Self::DARKER_SEA,
                Level::Darkest => Self::DARKEST_SEA,
            },
            Name::Turquoise => match level {
                Level::Desaturated => Self::DESATURATED_TURQUOISE,
                Level::Lightest => Self::LIGHTEST_TURQUOISE,
                Level::Lighter => Self::LIGHTER_TURQUOISE,
                Level::Light => Self::LIGHT_TURQUOISE,
                Level::Normal => Self::TURQUOISE,
                Level::Dark => Self::DARK_TURQUOISE,
                Level::Darker => Self::DARKER_TURQUOISE,
                Level::Darkest => Self::DARKEST_TURQUOISE,
            },
            Name::Cyan => match level {
                Level::Desaturated => Self::DESATURATED_CYAN,
                Level::Lightest => Self::LIGHTEST_CYAN,
                Level::Lighter => Self::LIGHTER_CYAN,
                Level::Light => Self::LIGHT_CYAN,
                Level::Normal => Self::CYAN,
                Level::Dark => Self::DARK_CYAN,
                Level::Darker => Self::DARKER_CYAN,
                Level::Darkest => Self::DARKEST_CYAN,
            },
            Name::Sky => match level {
                Level::Desaturated => Self::DESATURATED_SKY,
                Level::Lightest => Self::LIGHTEST_SKY,
                Level::Lighter => Self::LIGHTER_SKY,
                Level::Light => Self::LIGHT_SKY,
                Level::Normal => Self::SKY,
                Level::Dark => Self::DARK_SKY,
                Level::Darker => Self::DARKER_SKY,
                Level::Darkest => Self::DARKEST_SKY,
            },
            Name::Azure => match level {
                Level::Desaturated => Self::DESATURATED_AZURE,
                Level::Lightest => Self::LIGHTEST_AZURE,
                Level::Lighter => Self::LIGHTER_AZURE,
                Level::Light => Self::LIGHT_AZURE,
                Level::Normal => Self::AZURE,
                Level::Dark => Self::DARK_AZURE,
                Level::Darker => Self::DARKER_AZURE,
                Level::Darkest => Self::DARKEST_AZURE,
            },
            Name::Blue => match level {
                Level::Desaturated => Self::DESATURATED_BLUE,
                Level::Lightest => Self::LIGHTEST_BLUE,
                Level::Lighter => Self::LIGHTER_BLUE,
                Level::Light => Self::LIGHT_BLUE,
                Level::Normal => Self::BLUE,
                Level::Dark => Self::DARK_BLUE,
                Level::Darker => Self::DARKER_BLUE,
                Level::Darkest => Self::DARKEST_BLUE,
            },
            Name::Han => match level {
                Level::Desaturated => Self::DESATURATED_HAN,
                Level::Lightest => Self::LIGHTEST_HAN,
                Level::Lighter => Self::LIGHTER_HAN,
                Level::Light => Self::LIGHT_HAN,
                Level::Normal => Self::HAN,
                Level::Dark => Self::DARK_HAN,
                Level::Darker => Self::DARKER_HAN,
                Level::Darkest => Self::DARKEST_HAN,
            },
            Name::Violet => match level {
                Level::Desaturated => Self::DESATURATED_VIOLET,
                Level::Lightest => Self::LIGHTEST_VIOLET,
                Level::Lighter => Self::LIGHTER_VIOLET,
                Level::Light => Self::LIGHT_VIOLET,
                Level::Normal => Self::VIOLET,
                Level::Dark => Self::DARK_VIOLET,
                Level::Darker => Self::DARKER_VIOLET,
                Level::Darkest => Self::DARKEST_VIOLET,
            },
            Name::Purple => match level {
                Level::Desaturated => Self::DESATURATED_PURPLE,
                Level::Lightest => Self::LIGHTEST_PURPLE,
                Level::Lighter => Self::LIGHTER_PURPLE,
                Level::Light => Self::LIGHT_PURPLE,
                Level::Normal => Self::PURPLE,
                Level::Dark => Self::DARK_PURPLE,
                Level::Darker => Self::DARKER_PURPLE,
                Level::Darkest => Self::DARKEST_PURPLE,
            },
            Name::Fuchsia => match level {
                Level::Desaturated => Self::DESATURATED_FUCHSIA,
                Level::Lightest => Self::LIGHTEST_FUCHSIA,
                Level::Lighter => Self::LIGHTER_FUCHSIA,
                Level::Light => Self::LIGHT_FUCHSIA,
                Level::Normal => Self::FUCHSIA,
                Level::Dark => Self::DARK_FUCHSIA,
                Level::Darker => Self::DARKER_FUCHSIA,
                Level::Darkest => Self::DARKEST_FUCHSIA,
            },
            Name::Magenta => match level {
                Level::Desaturated => Self::DESATURATED_MAGENTA,
                Level::Lightest => Self::LIGHTEST_MAGENTA,
                Level::Lighter => Self::LIGHTER_MAGENTA,
                Level::Light => Self::LIGHT_MAGENTA,
                Level::Normal => Self::MAGENTA,
                Level::Dark => Self::DARK_MAGENTA,
                Level::Darker => Self::DARKER_MAGENTA,
                Level::Darkest => Self::DARKEST_MAGENTA,
            },
            Name::Pink => match level {
                Level::Desaturated => Self::DESATURATED_PINK,
                Level::Lightest => Self::LIGHTEST_PINK,
                Level::Lighter => Self::LIGHTER_PINK,
                Level::Light => Self::LIGHT_PINK,
                Level::Normal => Self::PINK,
                Level::Dark => Self::DARK_PINK,
                Level::Darker => Self::DARKER_PINK,
                Level::Darkest => Self::DARKEST_PINK,
            },
            Name::Crimson => match level {
                Level::Desaturated => Self::DESATURATED_CRIMSON,
                Level::Lightest => Self::LIGHTEST_CRIMSON,
                Level::Lighter => Self::LIGHTER_CRIMSON,
                Level::Light => Self::LIGHT_CRIMSON,
                Level::Normal => Self::CRIMSON,
                Level::Dark => Self::DARK_CRIMSON,
                Level::Darker => Self::DARKER_CRIMSON,
                Level::Darkest => Self::DARKEST_CRIMSON,
            },
        }
    }
}

// Constants
impl Color {
    /* color values */
    pub const BLACK: Color = Color::new(0, 0, 0);
    pub const DARKEST_GREY: Color = Color::new(31, 31, 31);
    pub const DARKER_GREY: Color = Color::new(63, 63, 63);
    pub const DARK_GREY: Color = Color::new(95, 95, 95);
    pub const GREY: Color = Color::new(127, 127, 127);
    pub const LIGHT_GREY: Color = Color::new(159, 159, 159);
    pub const LIGHTER_GREY: Color = Color::new(191, 191, 191);
    pub const LIGHTEST_GREY: Color = Color::new(223, 223, 223);
    pub const WHITE: Color = Color::new(255, 255, 255);

    pub const DARKEST_SEPIA: Color = Color::new(31, 24, 15);
    pub const DARKER_SEPIA: Color = Color::new(63, 50, 31);
    pub const DARK_SEPIA: Color = Color::new(94, 75, 47);
    pub const SEPIA: Color = Color::new(127, 101, 63);
    pub const LIGHT_SEPIA: Color = Color::new(158, 134, 100);
    pub const LIGHTER_SEPIA: Color = Color::new(191, 171, 143);
    pub const LIGHTEST_SEPIA: Color = Color::new(222, 211, 195);

    /* desaturated */
    pub const DESATURATED_RED: Color = Color::new(127, 63, 63);
    pub const DESATURATED_FLAME: Color = Color::new(127, 79, 63);
    pub const DESATURATED_ORANGE: Color = Color::new(127, 95, 63);
    pub const DESATURATED_AMBER: Color = Color::new(127, 111, 63);
    pub const DESATURATED_YELLOW: Color = Color::new(127, 127, 63);
    pub const DESATURATED_LIME: Color = Color::new(111, 127, 63);
    pub const DESATURATED_CHARTREUSE: Color = Color::new(95, 127, 63);
    pub const DESATURATED_GREEN: Color = Color::new(63, 127, 63);
    pub const DESATURATED_SEA: Color = Color::new(63, 127, 95);
    pub const DESATURATED_TURQUOISE: Color = Color::new(63, 127, 111);
    pub const DESATURATED_CYAN: Color = Color::new(63, 127, 127);
    pub const DESATURATED_SKY: Color = Color::new(63, 111, 127);
    pub const DESATURATED_AZURE: Color = Color::new(63, 95, 127);
    pub const DESATURATED_BLUE: Color = Color::new(63, 63, 127);
    pub const DESATURATED_HAN: Color = Color::new(79, 63, 127);
    pub const DESATURATED_VIOLET: Color = Color::new(95, 63, 127);
    pub const DESATURATED_PURPLE: Color = Color::new(111, 63, 127);
    pub const DESATURATED_FUCHSIA: Color = Color::new(127, 63, 127);
    pub const DESATURATED_MAGENTA: Color = Color::new(127, 63, 111);
    pub const DESATURATED_PINK: Color = Color::new(127, 63, 95);
    pub const DESATURATED_CRIMSON: Color = Color::new(127, 63, 79);

    /* lightest */
    pub const LIGHTEST_RED: Color = Color::new(255, 191, 191);
    pub const LIGHTEST_FLAME: Color = Color::new(255, 207, 191);
    pub const LIGHTEST_ORANGE: Color = Color::new(255, 223, 191);
    pub const LIGHTEST_AMBER: Color = Color::new(255, 239, 191);
    pub const LIGHTEST_YELLOW: Color = Color::new(255, 255, 191);
    pub const LIGHTEST_LIME: Color = Color::new(239, 255, 191);
    pub const LIGHTEST_CHARTREUSE: Color = Color::new(223, 255, 191);
    pub const LIGHTEST_GREEN: Color = Color::new(191, 255, 191);
    pub const LIGHTEST_SEA: Color = Color::new(191, 255, 223);
    pub const LIGHTEST_TURQUOISE: Color = Color::new(191, 255, 239);
    pub const LIGHTEST_CYAN: Color = Color::new(191, 255, 255);
    pub const LIGHTEST_SKY: Color = Color::new(191, 239, 255);
    pub const LIGHTEST_AZURE: Color = Color::new(191, 223, 255);
    pub const LIGHTEST_BLUE: Color = Color::new(191, 191, 255);
    pub const LIGHTEST_HAN: Color = Color::new(207, 191, 255);
    pub const LIGHTEST_VIOLET: Color = Color::new(223, 191, 255);
    pub const LIGHTEST_PURPLE: Color = Color::new(239, 191, 255);
    pub const LIGHTEST_FUCHSIA: Color = Color::new(255, 191, 255);
    pub const LIGHTEST_MAGENTA: Color = Color::new(255, 191, 239);
    pub const LIGHTEST_PINK: Color = Color::new(255, 191, 223);
    pub const LIGHTEST_CRIMSON: Color = Color::new(255, 191, 207);

    /* lighter */
    pub const LIGHTER_RED: Color = Color::new(255, 127, 127);
    pub const LIGHTER_FLAME: Color = Color::new(255, 159, 127);
    pub const LIGHTER_ORANGE: Color = Color::new(255, 191, 127);
    pub const LIGHTER_AMBER: Color = Color::new(255, 223, 127);
    pub const LIGHTER_YELLOW: Color = Color::new(255, 255, 127);
    pub const LIGHTER_LIME: Color = Color::new(223, 255, 127);
    pub const LIGHTER_CHARTREUSE: Color = Color::new(191, 255, 127);
    pub const LIGHTER_GREEN: Color = Color::new(127, 255, 127);
    pub const LIGHTER_SEA: Color = Color::new(127, 255, 191);
    pub const LIGHTER_TURQUOISE: Color = Color::new(127, 255, 223);
    pub const LIGHTER_CYAN: Color = Color::new(127, 255, 255);
    pub const LIGHTER_SKY: Color = Color::new(127, 223, 255);
    pub const LIGHTER_AZURE: Color = Color::new(127, 191, 255);
    pub const LIGHTER_BLUE: Color = Color::new(127, 127, 255);
    pub const LIGHTER_HAN: Color = Color::new(159, 127, 255);
    pub const LIGHTER_VIOLET: Color = Color::new(191, 127, 255);
    pub const LIGHTER_PURPLE: Color = Color::new(223, 127, 255);
    pub const LIGHTER_FUCHSIA: Color = Color::new(255, 127, 255);
    pub const LIGHTER_MAGENTA: Color = Color::new(255, 127, 223);
    pub const LIGHTER_PINK: Color = Color::new(255, 127, 191);
    pub const LIGHTER_CRIMSON: Color = Color::new(255, 127, 159);

    /* light */
    pub const LIGHT_RED: Color = Color::new(255, 63, 63);
    pub const LIGHT_FLAME: Color = Color::new(255, 111, 63);
    pub const LIGHT_ORANGE: Color = Color::new(255, 159, 63);
    pub const LIGHT_AMBER: Color = Color::new(255, 207, 63);
    pub const LIGHT_YELLOW: Color = Color::new(255, 255, 63);
    pub const LIGHT_LIME: Color = Color::new(207, 255, 63);
    pub const LIGHT_CHARTREUSE: Color = Color::new(159, 255, 63);
    pub const LIGHT_GREEN: Color = Color::new(63, 255, 63);
    pub const LIGHT_SEA: Color = Color::new(63, 255, 159);
    pub const LIGHT_TURQUOISE: Color = Color::new(63, 255, 207);
    pub const LIGHT_CYAN: Color = Color::new(63, 255, 255);
    pub const LIGHT_SKY: Color = Color::new(63, 207, 255);
    pub const LIGHT_AZURE: Color = Color::new(63, 159, 255);
    pub const LIGHT_BLUE: Color = Color::new(63, 63, 255);
    pub const LIGHT_HAN: Color = Color::new(111, 63, 255);
    pub const LIGHT_VIOLET: Color = Color::new(159, 63, 255);
    pub const LIGHT_PURPLE: Color = Color::new(207, 63, 255);
    pub const LIGHT_FUCHSIA: Color = Color::new(255, 63, 255);
    pub const LIGHT_MAGENTA: Color = Color::new(255, 63, 207);
    pub const LIGHT_PINK: Color = Color::new(255, 63, 159);
    pub const LIGHT_CRIMSON: Color = Color::new(255, 63, 111);

    /* normal */
    pub const RED: Color = Color::new(255, 0, 0);
    pub const FLAME: Color = Color::new(255, 63, 0);
    pub const ORANGE: Color = Color::new(255, 127, 0);
    pub const AMBER: Color = Color::new(255, 191, 0);
    pub const YELLOW: Color = Color::new(255, 255, 0);
    pub const LIME: Color = Color::new(191, 255, 0);
    pub const CHARTREUSE: Color = Color::new(127, 255, 0);
    pub const GREEN: Color = Color::new(0, 255, 0);
    pub const SEA: Color = Color::new(0, 255, 127);
    pub const TURQUOISE: Color = Color::new(0, 255, 191);
    pub const CYAN: Color = Color::new(0, 255, 255);
    pub const SKY: Color = Color::new(0, 191, 255);
    pub const AZURE: Color = Color::new(0, 127, 255);
    pub const BLUE: Color = Color::new(0, 0, 255);
    pub const HAN: Color = Color::new(63, 0, 255);
    pub const VIOLET: Color = Color::new(127, 0, 255);
    pub const PURPLE: Color = Color::new(191, 0, 255);
    pub const FUCHSIA: Color = Color::new(255, 0, 255);
    pub const MAGENTA: Color = Color::new(255, 0, 191);
    pub const PINK: Color = Color::new(255, 0, 127);
    pub const CRIMSON: Color = Color::new(255, 0, 63);

    /* dark */
    pub const DARK_RED: Color = Color::new(191, 0, 0);
    pub const DARK_FLAME: Color = Color::new(191, 47, 0);
    pub const DARK_ORANGE: Color = Color::new(191, 95, 0);
    pub const DARK_AMBER: Color = Color::new(191, 143, 0);
    pub const DARK_YELLOW: Color = Color::new(191, 191, 0);
    pub const DARK_LIME: Color = Color::new(143, 191, 0);
    pub const DARK_CHARTREUSE: Color = Color::new(95, 191, 0);
    pub const DARK_GREEN: Color = Color::new(0, 191, 0);
    pub const DARK_SEA: Color = Color::new(0, 191, 95);
    pub const DARK_TURQUOISE: Color = Color::new(0, 191, 143);
    pub const DARK_CYAN: Color = Color::new(0, 191, 191);
    pub const DARK_SKY: Color = Color::new(0, 143, 191);
    pub const DARK_AZURE: Color = Color::new(0, 95, 191);
    pub const DARK_BLUE: Color = Color::new(0, 0, 191);
    pub const DARK_HAN: Color = Color::new(47, 0, 191);
    pub const DARK_VIOLET: Color = Color::new(95, 0, 191);
    pub const DARK_PURPLE: Color = Color::new(143, 0, 191);
    pub const DARK_FUCHSIA: Color = Color::new(191, 0, 191);
    pub const DARK_MAGENTA: Color = Color::new(191, 0, 143);
    pub const DARK_PINK: Color = Color::new(191, 0, 95);
    pub const DARK_CRIMSON: Color = Color::new(191, 0, 47);

    /* darker */
    pub const DARKER_RED: Color = Color::new(127, 0, 0);
    pub const DARKER_FLAME: Color = Color::new(127, 31, 0);
    pub const DARKER_ORANGE: Color = Color::new(127, 63, 0);
    pub const DARKER_AMBER: Color = Color::new(127, 95, 0);
    pub const DARKER_YELLOW: Color = Color::new(127, 127, 0);
    pub const DARKER_LIME: Color = Color::new(95, 127, 0);
    pub const DARKER_CHARTREUSE: Color = Color::new(63, 127, 0);
    pub const DARKER_GREEN: Color = Color::new(0, 127, 0);
    pub const DARKER_SEA: Color = Color::new(0, 127, 63);
    pub const DARKER_TURQUOISE: Color = Color::new(0, 127, 95);
    pub const DARKER_CYAN: Color = Color::new(0, 127, 127);
    pub const DARKER_SKY: Color = Color::new(0, 95, 127);
    pub const DARKER_AZURE: Color = Color::new(0, 63, 127);
    pub const DARKER_BLUE: Color = Color::new(0, 0, 127);
    pub const DARKER_HAN: Color = Color::new(31, 0, 127);
    pub const DARKER_VIOLET: Color = Color::new(63, 0, 127);
    pub const DARKER_PURPLE: Color = Color::new(95, 0, 127);
    pub const DARKER_FUCHSIA: Color = Color::new(127, 0, 127);
    pub const DARKER_MAGENTA: Color = Color::new(127, 0, 95);
    pub const DARKER_PINK: Color = Color::new(127, 0, 63);
    pub const DARKER_CRIMSON: Color = Color::new(127, 0, 31);

    /* darkest */
    pub const DARKEST_RED: Color = Color::new(63, 0, 0);
    pub const DARKEST_FLAME: Color = Color::new(63, 15, 0);
    pub const DARKEST_ORANGE: Color = Color::new(63, 31, 0);
    pub const DARKEST_AMBER: Color = Color::new(63, 47, 0);
    pub const DARKEST_YELLOW: Color = Color::new(63, 63, 0);
    pub const DARKEST_LIME: Color = Color::new(47, 63, 0);
    pub const DARKEST_CHARTREUSE: Color = Color::new(31, 63, 0);
    pub const DARKEST_GREEN: Color = Color::new(0, 63, 0);
    pub const DARKEST_SEA: Color = Color::new(0, 63, 31);
    pub const DARKEST_TURQUOISE: Color = Color::new(0, 63, 47);
    pub const DARKEST_CYAN: Color = Color::new(0, 63, 63);
    pub const DARKEST_SKY: Color = Color::new(0, 47, 63);
    pub const DARKEST_AZURE: Color = Color::new(0, 31, 63);
    pub const DARKEST_BLUE: Color = Color::new(0, 0, 63);
    pub const DARKEST_HAN: Color = Color::new(15, 0, 63);
    pub const DARKEST_VIOLET: Color = Color::new(31, 0, 63);
    pub const DARKEST_PURPLE: Color = Color::new(47, 0, 63);
    pub const DARKEST_FUCHSIA: Color = Color::new(63, 0, 63);
    pub const DARKEST_MAGENTA: Color = Color::new(63, 0, 47);
    pub const DARKEST_PINK: Color = Color::new(63, 0, 31);
    pub const DARKEST_CRIMSON: Color = Color::new(63, 0, 15);

    /* metallic */
    pub const BRASS: Color = Color::new(191, 151, 96);
    pub const COPPER: Color = Color::new(197, 136, 124);
    pub const GOLD: Color = Color::new(229, 191, 0);
    pub const SILVER: Color = Color::new(203, 203, 203);

    /* miscellaneous */
    pub const CELADON: Color = Color::new(172, 255, 175);
    pub const PEACH: Color = Color::new(255, 159, 127);
}

impl Add for Color {
    type Output = Color;

    /// Add two colors together and return the result.
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            self.r.saturating_add(rhs.r),
            self.g.saturating_add(rhs.g),
            self.b.saturating_add(rhs.b),
        )
    }
}

impl Sub for Color {
    type Output = Color;

    /// Subtract the right hand side from the left hand side and return the result.
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(
            self.r.saturating_sub(rhs.r),
            self.g.saturating_sub(rhs.g),
            self.b.saturating_sub(rhs.b),
        )
    }
}

impl Mul for Color {
    type Output = Color;

    /// Multiply two colors together and return the result.
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(
            ((self.r as i32) * rhs.r as i32 / 255) as u8,
            ((self.g as i32) * rhs.g as i32 / 255) as u8,
            ((self.b as i32) * rhs.b as i32 / 255) as u8,
        )
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    /// Multiply a color with a scalar value and return the result.
    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(
            ((self.r as f32) * rhs).min(255.0).max(0.0) as u8,
            ((self.g as f32) * rhs).min(255.0).max(0.0) as u8,
            ((self.b as f32) * rhs).min(255.0).max(0.0) as u8,
        )
    }
}

/// Color names
pub enum Name {
    Red,
    Flame,
    Orange,
    Amber,
    Yellow,
    Lime,
    Chartreuse,
    Green,
    Sea,
    Turquoise,
    Cyan,
    Sky,
    Azure,
    Blue,
    Han,
    Violet,
    Purple,
    Fuchsia,
    Magenta,
    Pink,
    Crimson,
}

/// Color levels
pub enum Level {
    Desaturated,
    Lightest,
    Lighter,
    Light,
    Normal,
    Dark,
    Darker,
    Darkest,
}

#[cfg(test)]
mod tests {
    use crate::color::Color;

    #[test]
    fn hsv() {
        let red = Color::new_hsv(0., 1., 1.);
        let green = Color::new_hsv(120., 1., 1.);
        let blue = Color::new_hsv(240., 1., 1.);

        assert_eq!(red, Color::new(255, 0, 0));
        assert_eq!(green, Color::new(0, 255, 0));
        assert_eq!(blue, Color::new(0, 0, 255));

        let yellow = Color::new_hsv(60., 1., 1.);
        let cyan = Color::new_hsv(180., 1., 1.);
        let magenta = Color::new_hsv(300., 1., 1.);

        assert_eq!(yellow, Color::new(255, 255, 0));
        assert_eq!(cyan, Color::new(0, 255, 255));
        assert_eq!(magenta, Color::new(255, 0, 255));

        let black = Color::new_hsv(0., 0., 0.);
        let white = Color::new_hsv(0., 0., 1.);
        let gray = Color::new_hsv(0., 0., 0.5);
        let silver = Color::new_hsv(0., 0., 0.75);

        assert_eq!(black, Color::new(0, 0, 0));
        assert_eq!(white, Color::new(255, 255, 255));
        assert_eq!(gray, Color::new(128, 128, 128));
        assert_eq!(silver, Color::new(191, 191, 191));
    }

    #[test]
    fn lerp() {
        let black = Color::BLACK;
        let white = Color::WHITE;

        let left = black.lerp_rgb(white, 0.0);
        let right = black.lerp_rgb(white, 1.0);
        let middle = black.lerp_rgb(white, 0.5);

        assert_eq!(left, black);
        assert_eq!(right, white);
        assert_eq!(middle, Color::GREY);

        let left = black.lerp_hsv(white, 0.0);
        let right = black.lerp_hsv(white, 1.0);
        let middle = black.lerp_hsv(white, 0.5);

        assert_eq!(left, black);
        assert_eq!(right, white);
        assert_eq!(middle, Color::new(128, 128, 128));

        let orange = Color::ORANGE;
        let cyan = Color::CYAN;

        let middle = orange.lerp_rgb(cyan, 0.5);
        assert_eq!(middle, Color::new(127, 191, 127));

        let middle = orange.lerp_hsv(cyan, 0.5);
        assert_eq!(middle, Color::new(64, 255, 0));

        let middle = Color::LIGHTEST_RED.lerp_rgb(Color::LIGHT_BLUE, 0.5);
        assert_eq!(middle, Color::new(159, 127, 223));

        let middle = Color::LIGHTEST_RED.lerp_hsv(Color::LIGHT_BLUE, 0.5);
        assert_eq!(middle, Color::LIGHTER_FUCHSIA);
    }

    #[test]
    fn gradient() {
        let grayscale = Color::generate_gradient_rgb(&[Color::BLACK, Color::WHITE], &[254]);

        assert_eq!(grayscale.len(), 256);
        for (i, color) in grayscale.iter().enumerate() {
            assert_eq!(color.r, i as u8);
            assert_eq!(color.g, i as u8);
            assert_eq!(color.b, i as u8);
        }

        let grayscale = Color::generate_gradient_hsv(&[Color::BLACK, Color::WHITE], &[254]);

        assert_eq!(grayscale.len(), 256);
        for (i, color) in grayscale.iter().enumerate() {
            assert_eq!(color.r, i as u8);
            assert_eq!(color.g, i as u8);
            assert_eq!(color.b, i as u8);
        }
    }
}
