![](https://github.com/Pencilcaseman/librapid/blob/master/branding/LibRapid_light.png)
[![Discord](https://img.shields.io/discord/848914274105557043?color=green&label=Discord&logo=Discord)](https://discord.gg/cau7zy7zBE)
[![Crates.io](https://img.shields.io/crates/v/lib_rapid?color=green&label=Latest&logo=Rust&logoColor=orange)](https://crates.io/crates/lib_rapid/)
[![docs.rs](https://img.shields.io/docsrs/lib_rapid?color=green&label=Docs%20%28latest%29&logo=Rust&logoColor=orange)](https://docs.rs/crate/lib_rapid/latest)
[![Crates.io](https://img.shields.io/crates/d/lib_rapid?color=green&label=Downloads&logo=Rust&logoColor=orange)](https://crates.io/crates/lib_rapid)
----

LibRapid for Rust - Fast. Reliable. Lightweight.
============

**Changelog**
-----
0.1.6
_____
- Added Public API: `VecStackOps` trait.
- Added Public API: `RGBa`: `from_str`.
- Changed Public API: `QuadraticEquation`: `get_solutions()` now returns a `Option<(Option<f64>, Option<f64>)>`
- Changed Public API: `QuadraticEquation`: Fixed `Display` bug.
- Changed Public API: `Date`: Changed return value of getters.

Credits
-----

The original idea comes from Pencilcaseman, I collaborated with him to get LibRapid into Rust. We're closely working together to provide
the best possible user experience and consistency over all libraries.

Why X? Why Y?
-----

Sometimes I have to yank a version. To stay updated, please consider joining our Discord! https://discord.gg/cGxTFTgCAC

Contributing
-----

You want to contribute? Sure! You can contribute with:

- 🚩 Issues
- 🙇 Pull Requests (See `FORMATRULES.md` for more information)
- 💡 Feature Suggestions (Via Discussions or Issues)

Be sure to add documentation to any new public API with examples, and running `cargo test` to ensure that your changes are valid.

Documentation
-----

Depending on the current published LibRapid version and how many crates are in queue on docs.rs, you may need to build the docs yourself.
To do that, simply git-clone this repository and then run `cargo doc --open`. After that, your documentation should open automatically.

How does it work?
-----

The Rust-port of LibRapid is a derivative from the main library made for C++ and Python. It's goal is to be optimised for speedy calculations, mathematical and
scientific applications.

Performance
-----

LibRapid for Rust is fully optimised - meaning your calculations for game scripts and maths will be faster as a result.