# My first rust project -- file archiving utilities.

### install:
cargo install --git https://github.com/chrisw-gg/file-archive-utils

### usage:
# assuming ~/.cargo/bin/ is in path...
file-archive-utils validate ./test/ --verbose --quick
file-archive-utils update ./test/ --verbose

### cargo usage:
cargo run --release validate ./test/ --verbose --quick
cargo run --release update ./test/ --verbose