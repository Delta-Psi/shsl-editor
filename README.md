# SHSL Editor
Danganronpa 2 modding utilities

## Compiling
1. Install the [Rust toolchain](https://rustup.rs/).
2. Run `cargo build --release` . The CLI executable will be located in `target/release`.

## Basic usage
Create a project and extract game data there: `shsl-editor-cli extract [PATH TO DR2_DATA.WAD] [PATH TO DR2_DATA_US.WAD] [OUTPUT DIRECTORY]`. Note that the output directory must not exist.

Reinject edited game data: `shsl-editor-cli inject [PATH TO DR2_DATA.WAD] [PATH TO DR2_DATA_US.WAD] [PROJECT DIRECTORY]`.
