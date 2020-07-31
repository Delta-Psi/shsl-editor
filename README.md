# SHSL Editor
Danganronpa 2 modding utilities

## Compiling
1. Install the [Rust toolchain](https://rustup.rs/).
2. Run `cargo build --release` . The CLI executable will be located in `target/release`.

## Basic usage
1. Create a blank project: `shsl-editor-cli new [PROJECT DIRECTORY]`. Note that the directory must not exist, and will be created by the command.
2. Edit the `Project.toml` to set up the game data you wish to edit.
3. Extract the data: `shsl-editor-cli extract [PROJECT DIRECTORY] [PATH TO GAME FILES]`.
4. Edit the data as you wish.
5. Reinject the edited files: `shsl-editor-cli inject [PROJECT DIRECTORY] [PATH TO GAME FILES]`.
