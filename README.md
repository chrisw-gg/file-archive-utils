# My first rust project -- file archiving utilities.

### install:
cargo install --git https://github.com/chrisw-gg/file-archive-utils

### usage:
~/.cargo/bin/file-archive-utils validate ./test/ --verbose --quick
~/.cargo/bin/file-archive-utils update ./test/ --verbose

### local usage:
cargo run validate ./test/ --verbose --quick
cargo run update ./test/ --verbose