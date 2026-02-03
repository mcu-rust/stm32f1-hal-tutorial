# Setup development environment

## For Compilation

1. Install Rust
    1. https://rust-lang.org/tools/install/
    2. `rustup target add thumbv7m-none-eabi`
2. Install VSCode extensions
    1. See [.vscode/extensions.json](.vscode/extensions.json)

## For Debugging

1. Install ST-Link Driver
    1. https://www.st.com/en/development-tools/stm32cubeprog.html
2. Install `probe-rs-tools`
    1. https://probe.rs/docs/getting-started/installation/

# Initialize Project

1. `cargo new <name>`
2. `cargo add --features=f103,x8,critical-section-single-core stm32f1-hal`
3. `cargo add heap1`
4. Add `.cargo/config.toml` file
5. Add `memory.x` file
6. Add `settings.json` and `tasks.json` files
7. Add source code
8. Add `launch.json` file
    1. https://probe.rs/docs/tools/debugger/
9. Add SVD file
