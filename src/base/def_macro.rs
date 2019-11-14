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
    ($name:ident, $type:ty, $field1:ident, $field2:ident, $zero_constant:ident, $format_string:expr) => {
        #[derive(Copy, Clone, Default, PartialEq, Eq)]
        #[cfg_attr(feature = "debug", derive(Debug))]
        pub struct $name {
            pub $field1: $type,
            pub $field2: $type,
        }

        impl $name {
            pub const $zero_constant: Self = Self {
                $field1: 0,
                $field2: 0,
            };

            pub const fn new($field1: $type, $field2: $type) -> Self {
                Self { $field1, $field2 }
            }
        }

        impl From<$name> for ($type, $type) {
            fn from(f: $name) -> Self {
                (f.$field1, f.$field2)
            }
        }

        impl From<($type, $type)> for $name {
            fn from(t: ($type, $type)) -> Self {
                Self::new(t.0, t.1)
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, $format_string, self.$field1, self.$field2)
            }
        }

        impl std::ops::Add<$type> for $name {
            type Output = Self;

            fn add(self, rhs: $type) -> Self::Output {
                Self {
                    $field1: self.$field1 + rhs,
                    $field2: self.$field2 + rhs,
                }
            }
        }

        impl std::ops::Add<($type, $type)> for $name {
            type Output = Self;

            fn add(self, rhs: ($type, $type)) -> Self::Output {
                Self {
                    $field1: self.$field1 + rhs.0,
                    $field2: self.$field2 + rhs.1,
                }
            }
        }

        impl std::ops::AddAssign<$type> for $name {
            fn add_assign(&mut self, rhs: $type) {
                self.$field1 += rhs;
                self.$field2 += rhs;
            }
        }

        impl std::ops::AddAssign<($type, $type)> for $name {
            fn add_assign(&mut self, rhs: ($type, $type)) {
                self.$field1 += rhs.0;
                self.$field2 += rhs.1;
            }
        }

        impl std::ops::Sub<$type> for $name {
            type Output = Self;

            fn sub(self, rhs: $type) -> Self::Output {
                Self {
                    $field1: self.$field1 - rhs,
                    $field2: self.$field2 - rhs,
                }
            }
        }

        impl std::ops::Sub<($type, $type)> for $name {
            type Output = Self;

            fn sub(self, rhs: ($type, $type)) -> Self::Output {
                Self {
                    $field1: self.$field1 - rhs.0,
                    $field2: self.$field2 - rhs.1,
                }
            }
        }

        impl std::ops::SubAssign<$type> for $name {
            fn sub_assign(&mut self, rhs: $type) {
                self.$field1 -= rhs;
                self.$field2 -= rhs;
            }
        }

        impl std::ops::SubAssign<($type, $type)> for $name {
            fn sub_assign(&mut self, rhs: ($type, $type)) {
                self.$field1 -= rhs.0;
                self.$field2 -= rhs.1;
            }
        }

        impl std::ops::Mul<$type> for $name {
            type Output = Self;

            fn mul(self, rhs: $type) -> Self::Output {
                Self {
                    $field1: self.$field1 * rhs,
                    $field2: self.$field2 * rhs,
                }
            }
        }

        impl std::ops::MulAssign<$type> for $name {
            fn mul_assign(&mut self, rhs: $type) {
                self.$field1 *= rhs;
                self.$field2 *= rhs;
            }
        }

        impl std::ops::Div<$type> for $name {
            type Output = Self;

            fn div(self, rhs: $type) -> Self::Output {
                Self {
                    $field1: self.$field1 / rhs,
                    $field2: self.$field2 / rhs,
                }
            }
        }

        impl std::ops::DivAssign<$type> for $name {
            fn div_assign(&mut self, rhs: $type) {
                self.$field1 /= rhs;
                self.$field2 /= rhs;
            }
        }

        impl std::ops::Rem<$type> for $name {
            type Output = Self;

            fn rem(self, rhs: $type) -> Self::Output {
                Self {
                    $field1: self.$field1 % rhs,
                    $field2: self.$field2 % rhs,
                }
            }
        }

        impl std::ops::RemAssign<$type> for $name {
            fn rem_assign(&mut self, rhs: $type) {
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
    };
}
