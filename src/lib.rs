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

// NOTE: Make sure to keep this documentation in sync with the README.

//! # Doryen-extra
//!
//! Doryen-extra aims to be a loose re-implementation of the utility features from the popular roguelike
//! library named [`libtcod`], which is also known as the Doryen Library. The [`doryen-rs`] crate fulfills
//! the re-implementation of the game engine part of the library, but is otherwise (at the time of writing)
//! missing a lot of the features that were present in [`libtcod`].
//!
//! After finding myself frustrated with the various limitations and hassles that are involved when
//! wrapping a C library in rust, which is what the [`tcod`] crate has done, I decided to just go
//! all-in and re-code the library in Rust.
//!
//! While I could've just copied and pasted the code as it was, and turned it into Rust with minimal modifications,
//! I wanted to make it a proper Rust library, so it has been coded with retaining functionality in mind,
//! but not with retaining form. By that, I mean that all the functionality you're used to from [`libtcod`] should
//! be present, but how it's accessed or used may vary greatly from the original.
//!
//! # Features
//!
//! ## `doryen`
//!
//! While this library is called doryen-extra, I didn't actually want to force it to be tied to the
//! [`doryen-rs`] library, so functionality that pertains to it is behind the feature `doryen`. If
//! you want to use this library without bringing in [`doryen-rs`] as a dependency, just put
//! ```toml
//! [dependencies]
//! doryen-extra = { version = "...", default-features = false }
//! ```
//! in your `Cargo.toml` file, which removes the default `doryen` feature.
//!
//! ## `libtcod-compat`
//!
//! This feature restores (on a best-effort basis) the functionality of the original
//! `libtcod` library, where it has been modified. At the time of writing, the only change that
//! will happen is to the float generation of the `ComplementaryMultiplyWithCarry` RNG algorithm.
//!
//! ## `rng_support`
//!
//! With this feature enabled, the [`Random`] struct implements [`rand_core::RngCore`] and
//! [`rand_core::SeedableRng`], which lets it be used in any place that accepts the `rand` crate RNGs.
//!
//! ## `serialization`
//!
//! With this feature enabled, all types for which it makes sense to serialize will implement
//! [`serde::ser::Serialize`] and [`serde::de::Deserialize`]. NOTE: More types may get implementations
//! for this in the future.
//!
//! # Missing Features / Toolkits
//!
//! The following toolkits from [`libtcod`] have not yet been converted, with possible reason given in parenthesis:
//! * `bsp` toolkit: 2D Binary Space Partition
//! * `fov` toolkit: Easily calculate the potential visible set of map cells from the player position
//! * `image` toolkit: Some image manipulation utilities (undecided on whether to convert this one; other crates may already serve this purpose)
//! * `list` toolkit: A fast, lightweight and generic container, that provides array, list and stack paradigms (use `Vec` instead)
//! * `namegen` toolkit: Allows one to generate random names out of custom made syllable sets (parts requires `parse` toolkit)
//! * `parse` toolkit: An easy way to parse complex text configuration files
//!
//! [`libtcod`]: https://github.com/libtcod/libtcod
//! [`doryen-rs`]: https://crates.io/crates/doryen-rs
//! [`tcod`]: https://crates.io/crates/tcod
//!
//! [`Random`]: ./random/struct.Random.html
//! [`rand_core::RngCore`]: ../rand_core/trait.RngCore.html
//! [`rand_core::SeedableRng`]: ../rand_core/trait.SeedableRng.html
//! [`serde::ser::Serialize`]: ../serde/ser/trait.Serialize.html
//! [`serde::de::Deserialize`]: ../serde/de/trait.Deserialize.html

// Coding conventions
//
// Deny (don't do this)
#![deny(anonymous_parameters)]
#![deny(bare_trait_objects)]
#![deny(elided_lifetimes_in_paths)]
#![deny(ellipsis_inclusive_range_patterns)]
#![deny(non_camel_case_types)]
#![deny(non_snake_case)]
#![deny(non_upper_case_globals)]
#![deny(trivial_numeric_casts)]
#![deny(unreachable_pub)]
#![deny(unsafe_code)]
#![deny(unused_import_braces)]
#![deny(unused_mut)]
#![deny(unused_qualifications)]
//
// Warn (try not to do this)
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(rust_2018_idioms)]
#![warn(variant_size_differences)]
//#![warn(unused_results)]
//
// Clippy conventions
//
// Deny (don't do this)
#![deny(clippy::cast_lossless)]
#![deny(clippy::default_trait_access)]
#![deny(clippy::empty_enum)]
#![deny(clippy::enum_glob_use)]
#![deny(clippy::expl_impl_clone_on_copy)]
#![deny(clippy::explicit_into_iter_loop)]
#![deny(clippy::explicit_iter_loop)]
#![deny(clippy::filter_map)]
#![deny(clippy::filter_map_next)]
#![deny(clippy::find_map)]
#![deny(clippy::if_not_else)]
#![deny(clippy::invalid_upcast_comparisons)]
#![deny(clippy::items_after_statements)]
#![deny(clippy::large_digit_groups)]
#![deny(clippy::map_flatten)]
#![deny(clippy::match_same_arms)]
#![deny(clippy::mut_mut)]
#![deny(clippy::needless_continue)]
#![deny(clippy::needless_pass_by_value)]
#![deny(clippy::option_map_unwrap_or)]
#![deny(clippy::option_map_unwrap_or_else)]
#![deny(clippy::redundant_closure_for_method_calls)]
#![deny(clippy::result_map_unwrap_or_else)]
#![deny(clippy::single_match_else)]
#![deny(clippy::string_add_assign)]
#![deny(clippy::type_repetition_in_bounds)]
#![deny(clippy::unseparated_literal_suffix)]
#![deny(clippy::unused_self)]
#![deny(clippy::use_self)] // Sometimes gives false positives; feel free to disable.
#![deny(clippy::used_underscore_binding)]
//
// Warn (try not to do this)
//#![warn(clippy::must_use_candidate)]
#![deny(clippy::new_without_default)]
#![warn(clippy::pub_enum_variant_names)]
#![warn(clippy::replace_consts)]
#![warn(clippy::shadow_unrelated)]
#![warn(clippy::similar_names)]
#![warn(clippy::too_many_lines)]

#[macro_use]
mod util;

mod base;
pub use base::*;

#[cfg(feature = "doryen")]
pub mod extenders;
#[cfg(feature = "doryen")]
pub mod extensions;

pub mod bresenham;
pub mod color;

pub mod heightmap;
pub mod noise;
pub mod random;
