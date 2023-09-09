#

## Information

### Linker

- Linker [Mold](https://github.com/rui314/mold)

To switch to lld (Windows and Linux) (llvm project) and zld (MacOS)

````
# .cargo/config.toml
# On Windows
# ```
# cargo install -f cargo-binutils
# rustup component add llvm-tools-preview
# ```
[target.x86_64-pc-windows-msvc]
rustflags = ["-C", "link-arg=-fuse-ld=lld"]
[target.x86_64-pc-windows-gnu]
rustflags = ["-C", "link-arg=-fuse-ld=lld"]
# On Linux:
# - Ubuntu, `sudo apt-get install lld clang`
# - Arch, `sudo pacman -S lld clang`
[target.x86_64-unknown-linux-gnu]
rustflags = ["-C", "linker=clang", "-C", "link-arg=-fuse-ld=lld"]
# On MacOS, `brew install michaeleisel/zld/zld`
[target.x86_64-apple-darwin]
rustflags = ["-C", "link-arg=-fuse-ld=/usr/local/bin/zld"]
[target.aarch64-apple-darwin]
rustflags = ["-C", "link-arg=-fuse-ld=/usr/local/bin/zld"]
````

## Commands

### Cargo watch

`cargo watch -x build`

`cargo watch -x test -x run`

### Tarpaulin

Code coverage

`cargo tarpaulin --ignore-tests`

### Clippy (lint)

`cargo clippy`

`cargo clippy -- -D warnings`

`cargo clippy --fix`

### Rustfmt (format)

`cargo fmt`

`cargo fmt -- --check`

### Cargo deny

Audit dependencies. Developed by EmbarkStudios.

`cargo deny check`

### Cargo expand

Expands macro code.

`cargo expand`
