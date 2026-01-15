My first rust project -- file archiving utilities.

# Create project

docker run --rm -v "$(pwd)/:/workdir" -w /workdir rust:1.92 cargo init --name archive

# Build project

docker run --rm -v "$(pwd)/:/workdir" -w /workdir rust:1.92 cargo build

# Run 

docker run --rm -v "$(pwd)/foo:/workdir" -w /workdir rust:1.92 cargo run

# Use

docker run --rm -it -v "$(pwd):/workdir" -w /workdir rust:1.92 bash

cargo build

cargo run -- --verbose --dry-run

cargo run -- --verbose