/* BSD 3-Clause License
 *
 * Copyright © 2019, Alexander Krivács Schrøder <alexschrod@gmail.com>.
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

macro_rules! define_two_property_arithmetic_struct {
    ($name:ident, $uname:ident, $fname: ident, $field1:ident, $field2:ident, $zero_constant:ident, $format_string:expr) => {
        define_two_property_arithmetic_struct!(@IMPL $name, $uname, $fname, stringify!($name), $field1, $field2, stringify!($field1), stringify!($field2), $zero_constant, $format_string);
    };
    (@IMPL $name:ident, $uname:ident, $fname: ident, $name_str:expr, $field1:ident, $field2:ident, $field1_str:expr, $field2_str:expr, $zero_constant:ident, $format_string:expr) => {
        #[doc = "A struct representing a"]
        #[doc = $name_str]
        #[doc = "determined by its `"]
        #[doc = $field1_str]
        #[doc = "` and `"]
        #[doc = $field2_str]
        #[doc = "` values."]
        #[derive(Copy, Clone, Default, PartialEq, Eq)]
        #[derive(Debug)]
        #[cfg_attr(feature = "serialization", derive(::serde_derive::Serialize, ::serde_derive::Deserialize))]
        pub struct $name {
            /// The `
            #[doc = $field1_str]
            /// ` value the `
            #[doc = $name_str]
            /// ` is currently representing.
            pub $field1: i32,

            /// The `
            #[doc = $field2_str]
            /// ` value the `
            #[doc = $name_str]
            /// ` is currently representing.
            pub $field2: i32,
        }

        impl $name {
            /// A constant representing a `
            #[doc = $name_str]
            /// ` where both `
            #[doc = $field1_str]
            /// ` and `
            #[doc = $field2_str]
            /// ` are 0.
            pub const $zero_constant: Self = Self {
                $field1: 0,
                $field2: 0,
            };

            /// Returns a `
            #[doc = $name_str]
            /// ` with the given `
            #[doc = $field1_str]
            /// ` and `
            #[doc = $field2_str]
            /// ` values.
            pub const fn new($field1: i32, $field2: i32) -> Self {
                Self { $field1, $field2 }
            }
        }

        impl From<$name> for (i32, i32) {
            fn from(f: $name) -> Self {
                (f.$field1, f.$field2)
            }
        }

        impl From<(i32, i32)> for $name {
            fn from(t: (i32, i32)) -> Self {
                Self::new(t.0, t.1)
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, $format_string, self.$field1, self.$field2)
            }
        }

        impl std::ops::Add for $name {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                Self {
                    $field1: self.$field1 + rhs.$field1,
                    $field2: self.$field2 + rhs.$field2,
                }
            }
        }

        impl std::ops::Add<i32> for $name {
            type Output = Self;

            fn add(self, rhs: i32) -> Self::Output {
                Self {
                    $field1: self.$field1 + rhs,
                    $field2: self.$field2 + rhs,
                }
            }
        }

        impl std::ops::Add<(i32, i32)> for $name {
            type Output = Self;

            fn add(self, rhs: (i32, i32)) -> Self::Output {
                Self {
                    $field1: self.$field1 + rhs.0,
                    $field2: self.$field2 + rhs.1,
                }
            }
        }

        impl std::ops::AddAssign<i32> for $name {
            fn add_assign(&mut self, rhs: i32) {
                self.$field1 += rhs;
                self.$field2 += rhs;
            }
        }

        impl std::ops::AddAssign<(i32, i32)> for $name {
            fn add_assign(&mut self, rhs: (i32, i32)) {
                self.$field1 += rhs.0;
                self.$field2 += rhs.1;
            }
        }

        impl std::ops::Sub for $name {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                Self {
                    $field1: self.$field1 - rhs.$field1,
                    $field2: self.$field2 - rhs.$field2,
                }
            }
        }

        impl std::ops::Sub<i32> for $name {
            type Output = Self;

            fn sub(self, rhs: i32) -> Self::Output {
                Self {
                    $field1: self.$field1 - rhs,
                    $field2: self.$field2 - rhs,
                }
            }
        }

        impl std::ops::Sub<(i32, i32)> for $name {
            type Output = Self;

            fn sub(self, rhs: (i32, i32)) -> Self::Output {
                Self {
                    $field1: self.$field1 - rhs.0,
                    $field2: self.$field2 - rhs.1,
                }
            }
        }

        impl std::ops::SubAssign<i32> for $name {
            fn sub_assign(&mut self, rhs: i32) {
                self.$field1 -= rhs;
                self.$field2 -= rhs;
            }
        }

        impl std::ops::SubAssign<(i32, i32)> for $name {
            fn sub_assign(&mut self, rhs: (i32, i32)) {
                self.$field1 -= rhs.0;
                self.$field2 -= rhs.1;
            }
        }

        impl std::ops::Mul<i32> for $name {
            type Output = Self;

            fn mul(self, rhs: i32) -> Self::Output {
                Self {
                    $field1: self.$field1 * rhs,
                    $field2: self.$field2 * rhs,
                }
            }
        }

        impl std::ops::MulAssign<i32> for $name {
            fn mul_assign(&mut self, rhs: i32) {
                self.$field1 *= rhs;
                self.$field2 *= rhs;
            }
        }

        impl std::ops::Div<i32> for $name {
            type Output = Self;

            fn div(self, rhs: i32) -> Self::Output {
                Self {
                    $field1: self.$field1 / rhs,
                    $field2: self.$field2 / rhs,
                }
            }
        }

        impl std::ops::DivAssign<i32> for $name {
            fn div_assign(&mut self, rhs: i32) {
                self.$field1 /= rhs;
                self.$field2 /= rhs;
            }
        }

        impl std::ops::Rem<i32> for $name {
            type Output = Self;

            fn rem(self, rhs: i32) -> Self::Output {
                Self {
                    $field1: self.$field1 % rhs,
                    $field2: self.$field2 % rhs,
                }
            }
        }

        impl std::ops::RemAssign<i32> for $name {
            fn rem_assign(&mut self, rhs: i32) {
                self.$field1 %= rhs;
                self.$field2 %= rhs;
            }
        }

        impl std::ops::Neg for $name {
            type Output = Self;

            fn neg(self) -> Self::Output {
                Self {
                    $field1: -self.$field1,
                    $field2: -self.$field2,
                }
            }
        }

        // Unsigned version:

        #[doc = "A struct representing an unsigned"]
        #[doc = $name_str]
        #[doc = "determined by its `"]
        #[doc = $field1_str]
        #[doc = "` and `"]
        #[doc = $field2_str]
        #[doc = "` values."]
        #[derive(Copy, Clone, Default, PartialEq, Eq)]
        #[derive(Debug)]
        #[cfg_attr(feature = "serialization", derive(::serde_derive::Serialize, ::serde_derive::Deserialize))]
        pub struct $uname {
            /// The `
            #[doc = $field1_str]
            /// ` value the `
            #[doc = $name_str]
            /// ` is currently representing.
            pub $field1: u32,

            /// The `
            #[doc = $field2_str]
            /// ` value the `
            #[doc = $name_str]
            /// ` is currently representing.
            pub $field2: u32,
        }

        impl $uname {
            /// A constant representing a `
            #[doc = $name_str]
            /// ` where both `
            #[doc = $field1_str]
            /// ` and `
            #[doc = $field2_str]
            /// ` are 0.
            pub const $zero_constant: Self = Self {
                $field1: 0,
                $field2: 0,
            };

            /// Returns a `
            #[doc = $name_str]
            /// ` with the given `
            #[doc = $field1_str]
            /// ` and `
            #[doc = $field2_str]
            /// ` values.
            pub const fn new($field1: u32, $field2: u32) -> Self {
                Self { $field1, $field2 }
            }
        }

        impl From<$uname> for (u32, u32) {
            fn from(f: $uname) -> Self {
                (f.$field1, f.$field2)
            }
        }

        impl From<(u32, u32)> for $uname {
            fn from(t: (u32, u32)) -> Self {
                Self::new(t.0, t.1)
            }
        }

        impl std::fmt::Display for $uname {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, $format_string, self.$field1, self.$field2)
            }
        }

        impl std::ops::Add for $uname {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                Self {
                    $field1: self.$field1 + rhs.$field1,
                    $field2: self.$field2 + rhs.$field2,
                }
            }
        }

        impl std::ops::Add<u32> for $uname {
            type Output = Self;

            fn add(self, rhs: u32) -> Self::Output {
                Self {
                    $field1: self.$field1 + rhs,
                    $field2: self.$field2 + rhs,
                }
            }
        }

        impl std::ops::Add<(u32, u32)> for $uname {
            type Output = Self;

            fn add(self, rhs: (u32, u32)) -> Self::Output {
                Self {
                    $field1: self.$field1 + rhs.0,
                    $field2: self.$field2 + rhs.1,
                }
            }
        }

        impl std::ops::AddAssign<u32> for $uname {
            fn add_assign(&mut self, rhs: u32) {
                self.$field1 += rhs;
                self.$field2 += rhs;
            }
        }

        impl std::ops::AddAssign<(u32, u32)> for $uname {
            fn add_assign(&mut self, rhs: (u32, u32)) {
                self.$field1 += rhs.0;
                self.$field2 += rhs.1;
            }
        }

        impl std::ops::Sub for $uname {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                Self {
                    $field1: self.$field1 - rhs.$field1,
                    $field2: self.$field2 - rhs.$field2,
                }
            }
        }

        impl std::ops::Sub<u32> for $uname {
            type Output = Self;

            fn sub(self, rhs: u32) -> Self::Output {
                Self {
                    $field1: self.$field1 - rhs,
                    $field2: self.$field2 - rhs,
                }
            }
        }

        impl std::ops::Sub<(u32, u32)> for $uname {
            type Output = Self;

            fn sub(self, rhs: (u32, u32)) -> Self::Output {
                Self {
                    $field1: self.$field1 - rhs.0,
                    $field2: self.$field2 - rhs.1,
                }
            }
        }

        impl std::ops::SubAssign<u32> for $uname {
            fn sub_assign(&mut self, rhs: u32) {
                self.$field1 -= rhs;
                self.$field2 -= rhs;
            }
        }

        impl std::ops::SubAssign<(u32, u32)> for $uname {
            fn sub_assign(&mut self, rhs: (u32, u32)) {
                self.$field1 -= rhs.0;
                self.$field2 -= rhs.1;
            }
        }

        impl std::ops::Mul<u32> for $uname {
            type Output = Self;

            fn mul(self, rhs: u32) -> Self::Output {
                Self {
                    $field1: self.$field1 * rhs,
                    $field2: self.$field2 * rhs,
                }
            }
        }

        impl std::ops::MulAssign<u32> for $uname {
            fn mul_assign(&mut self, rhs: u32) {
                self.$field1 *= rhs;
                self.$field2 *= rhs;
            }
        }

        impl std::ops::Div<u32> for $uname {
            type Output = Self;

            fn div(self, rhs: u32) -> Self::Output {
                Self {
                    $field1: self.$field1 / rhs,
                    $field2: self.$field2 / rhs,
                }
            }
        }

        impl std::ops::DivAssign<u32> for $uname {
            fn div_assign(&mut self, rhs: u32) {
                self.$field1 /= rhs;
                self.$field2 /= rhs;
            }
        }

        impl std::ops::Rem<u32> for $uname {
            type Output = Self;

            fn rem(self, rhs: u32) -> Self::Output {
                Self {
                    $field1: self.$field1 % rhs,
                    $field2: self.$field2 % rhs,
                }
            }
        }

        impl std::ops::RemAssign<u32> for $uname {
            fn rem_assign(&mut self, rhs: u32) {
                self.$field1 %= rhs;
                self.$field2 %= rhs;
            }
        }

        // Floating-point version

        #[doc = "A struct representing a floating-point"]
        #[doc = $name_str]
        #[doc = "determined by its `"]
        #[doc = $field1_str]
        #[doc = "` and `"]
        #[doc = $field2_str]
        #[doc = "` values."]
        #[derive(Copy, Clone, Default, PartialEq)]
        #[derive(Debug)]
        #[cfg_attr(feature = "serialization", derive(::serde_derive::Serialize, ::serde_derive::Deserialize))]
        pub struct $fname {
            /// The `
            #[doc = $field1_str]
            /// ` value the `
            #[doc = $name_str]
            /// ` is currently representing.
            pub $field1: f32,

            /// The `
            #[doc = $field2_str]
            /// ` value the `
            #[doc = $name_str]
            /// ` is currently representing.
            pub $field2: f32,
        }

        impl $fname {
            /// A constant representing a `
            #[doc = $name_str]
            /// ` where both `
            #[doc = $field1_str]
            /// ` and `
            #[doc = $field2_str]
            /// ` are 0.
            pub const $zero_constant: Self = Self {
                $field1: 0.0,
                $field2: 0.0,
            };

            /// Returns a `
            #[doc = $name_str]
            /// ` with the given `
            #[doc = $field1_str]
            /// ` and `
            #[doc = $field2_str]
            /// ` values.
            pub const fn new($field1: f32, $field2: f32) -> Self {
                Self { $field1, $field2 }
            }
        }

        impl From<$fname> for (f32, f32) {
            fn from(f: $fname) -> Self {
                (f.$field1, f.$field2)
            }
        }

        impl From<(f32, f32)> for $fname {
            fn from(t: (f32, f32)) -> Self {
                Self::new(t.0, t.1)
            }
        }

        impl std::fmt::Display for $fname {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, $format_string, self.$field1, self.$field2)
            }
        }

        impl std::ops::Add for $fname {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                Self {
                    $field1: self.$field1 + rhs.$field1,
                    $field2: self.$field2 + rhs.$field2,
                }
            }
        }

        impl std::ops::Add<f32> for $fname {
            type Output = Self;

            fn add(self, rhs: f32) -> Self::Output {
                Self {
                    $field1: self.$field1 + rhs,
                    $field2: self.$field2 + rhs,
                }
            }
        }

        impl std::ops::Add<(f32, f32)> for $fname {
            type Output = Self;

            fn add(self, rhs: (f32, f32)) -> Self::Output {
                Self {
                    $field1: self.$field1 + rhs.0,
                    $field2: self.$field2 + rhs.1,
                }
            }
        }

        impl std::ops::AddAssign<f32> for $fname {
            fn add_assign(&mut self, rhs: f32) {
                self.$field1 += rhs;
                self.$field2 += rhs;
            }
        }

        impl std::ops::AddAssign<(f32, f32)> for $fname {
            fn add_assign(&mut self, rhs: (f32, f32)) {
                self.$field1 += rhs.0;
                self.$field2 += rhs.1;
            }
        }

        impl std::ops::Sub for $fname {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                Self {
                    $field1: self.$field1 - rhs.$field1,
                    $field2: self.$field2 - rhs.$field2,
                }
            }
        }

        impl std::ops::Sub<f32> for $fname {
            type Output = Self;

            fn sub(self, rhs: f32) -> Self::Output {
                Self {
                    $field1: self.$field1 - rhs,
                    $field2: self.$field2 - rhs,
                }
            }
        }

        impl std::ops::Sub<(f32, f32)> for $fname {
            type Output = Self;

            fn sub(self, rhs: (f32, f32)) -> Self::Output {
                Self {
                    $field1: self.$field1 - rhs.0,
                    $field2: self.$field2 - rhs.1,
                }
            }
        }

        impl std::ops::SubAssign<f32> for $fname {
            fn sub_assign(&mut self, rhs: f32) {
                self.$field1 -= rhs;
                self.$field2 -= rhs;
            }
        }

        impl std::ops::SubAssign<(f32, f32)> for $fname {
            fn sub_assign(&mut self, rhs: (f32, f32)) {
                self.$field1 -= rhs.0;
                self.$field2 -= rhs.1;
            }
        }

        impl std::ops::Mul<f32> for $fname {
            type Output = Self;

            fn mul(self, rhs: f32) -> Self::Output {
                Self {
                    $field1: self.$field1 * rhs,
                    $field2: self.$field2 * rhs,
                }
            }
        }

        impl std::ops::MulAssign<f32> for $fname {
            fn mul_assign(&mut self, rhs: f32) {
                self.$field1 *= rhs;
                self.$field2 *= rhs;
            }
        }

        impl std::ops::Div<f32> for $fname {
            type Output = Self;

            fn div(self, rhs: f32) -> Self::Output {
                Self {
                    $field1: self.$field1 / rhs,
                    $field2: self.$field2 / rhs,
                }
            }
        }

        impl std::ops::DivAssign<f32> for $fname {
            fn div_assign(&mut self, rhs: f32) {
                self.$field1 /= rhs;
                self.$field2 /= rhs;
            }
        }

        impl std::ops::Rem<f32> for $fname {
            type Output = Self;

            fn rem(self, rhs: f32) -> Self::Output {
                Self {
                    $field1: self.$field1 % rhs,
                    $field2: self.$field2 % rhs,
                }
            }
        }

        impl std::ops::RemAssign<f32> for $fname {
            fn rem_assign(&mut self, rhs: f32) {
                self.$field1 %= rhs;
                self.$field2 %= rhs;
            }
        }

        impl std::ops::Neg for $fname {
            type Output = Self;

            fn neg(self) -> Self::Output {
                Self {
                    $field1: -self.$field1,
                    $field2: -self.$field2,
                }
            }
        }
    };
}
