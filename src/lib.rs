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

//! # Doryen-extra
//!
//! Doryen-extra aims to be a loose re-implementation of the popular rougelike library named libcod,
//! which is also known as the Doryen Library. While the doryen-rs crate fulfills this to some
//! extent, it is (at the present time) missing a lot of the features that were present in libtcod.    
//!
//! After finding myself frustrated with the various limitations and hassles that are involved when
//! wrapping a C library in rust, which is what the `tcod` crate has done, I decided to just go
//! all-in and re-code the entire library in Rust.
//!
//! While I could've just copied and pasted the code as it was, and called it a day, I also wanted
//! to make it a proper Rust library, so it has been coded with retaining functionality in mind,
//! but not with retaining form. By that, I mean that all the functionality from libtcod should be
//! present, but how it's accessed or used may vary greatly from the original.
//!
//! Some functionality may also have changed from the original. In places where it was possible
//! and/or desirable to also offer the original functionality, the feature `libcod-compat` may be
//! enabled to use these instead of the improved/changed versions.

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
