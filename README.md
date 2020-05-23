[![Crates.io][crates-badge]][crates]
[![Docs.rs][docs-badge]][docs]
[![CI][ci-badge]][ci]
[![MacOS][macos-badge]][macos]
[![License][license-badge]][license]
[![Code coverage][coverage-badge]][coverage]

[crates-badge]: https://img.shields.io/crates/v/doryen-extra
[crates]: https://crates.io/crates/doryen-extra

[docs-badge]: https://docs.rs/doryen-extra/badge.svg
[docs]: https://docs.rs/doryen-extra/

[ci-badge]: https://github.com/alexschrod/doryen-extra/workflows/CI/badge.svg
[ci]: https://github.com/alexschrod/doryen-extra/actions?query=workflow%3ACI

[macos-badge]: https://github.com/alexschrod/doryen-extra/workflows/MacOS/badge.svg
[macos]: https://github.com/alexschrod/doryen-extra/actions?query=workflow%3AMacOS

[license-badge]: https://img.shields.io/crates/l/doryen-extra
[license]: https://github.com/alexschrod/doryen-extra/blob/master/LICENSE.txt

[coverage-badge]: https://img.shields.io/codecov/c/github/alexschrod/doryen-extra
[coverage]: https://codecov.io/gh/alexschrod/doryen-extra

# Doryen-extra

Doryen-extra aims to be a loose re-implementation of the utility features from the popular roguelike
library named [`libtcod`], which is also known as the Doryen Library. The [`doryen-rs`] crate fulfills
the re-implementation of the game engine part of the library, but is otherwise (at the time of writing)
missing a lot of the features that were present in [`libtcod`].

After finding myself frustrated with the various limitations and hassles that are involved when
wrapping a C library in rust, which is what the [`tcod`] crate has done, I decided to just go
all-in and re-code the library in Rust.

While I could've just copied and pasted the code as it was, and turned it into Rust with minimal modifications,
I wanted to make it a proper Rust library, so it has been coded with retaining functionality in mind,
but not with retaining form. By that, I mean that all the functionality you're used to from [`libtcod`] should
be present, but how it's accessed or used may vary greatly from the original.

# Features

## `doryen`

While this library is called doryen-extra, I didn't actually want to force it to be tied to the
[`doryen-rs`] library, so functionality that pertains to it is behind the feature `doryen`. If
you want to use this library without bringing in [`doryen-rs`] as a dependency, just put
```toml
[dependencies]
doryen-extra = { version = "...", default-features = false }
```
in your `Cargo.toml` file, which removes the default `doryen` feature.

## `libtcod-compat`

This feature restores (on a best-effort basis) the functionality of the original
`libtcod` library, where it has been modified. At the time of writing, the only change that
will happen is to the float generation of the `ComplementaryMultiplyWithCarry` RNG algorithm.

## `rng_support`

With this feature enabled, the `Random` struct implements [`rand_core::RngCore`] and
[`rand_core::SeedableRng`], which lets it be used in any place that accepts the `rand` crate RNGs.

## `serialization`

With this feature enabled, all types for which it makes sense to serialize will implement
[`serde::ser::Serialize`] and [`serde::de::Deserialize`]. NOTE: More types may get implementations
for this in the future.

# Missing Features / Toolkits

The following toolkits from [`libtcod`] have not yet been converted, with possible reason given in parenthesis:
* `bsp` toolkit: 2D Binary Space Partition
* `fov` toolkit: Easily calculate the potential visible set of map cells from the player position
* `image` toolkit: Some image manipulation utilities (undecided on whether to convert this one; other crates may already serve this purpose)
* `list` toolkit: A fast, lightweight and generic container, that provides array, list and stack paradigms (use `Vec` instead)
* `namegen` toolkit: Allows one to generate random names out of custom made syllable sets (parts requires `parse` toolkit)
* `parse` toolkit: An easy way to parse complex text configuration files

[`libtcod`]: https://github.com/libtcod/libtcod
[`doryen-rs`]: https://crates.io/crates/doryen-rs
[`tcod`]: https://crates.io/crates/tcod

[`rand_core::RngCore`]: https://docs.rs/rand_core/0.5.1/rand_core/trait.RngCore.html
[`rand_core::SeedableRng`]: https://docs.rs/rand_core/0.5.1/rand_core/trait.SeedableRng.html
[`serde::ser::Serialize`]: https://docs.rs/serde/1.0.110/serde/trait.Serialize.html
[`serde::de::Deserialize`]: https://docs.rs/serde/1.0.110/serde/trait.Deserialize.html
