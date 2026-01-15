# My first rust project -- file archiving utilities.

## Cargo Commands

### Build
cargo build

## Full Validation
cargo run -- --contents --dry-run

### Quick Validation
cargo run -- --dry-run

### Verbose Run
cargo run -- --verbose

### Minimal Run
cargo run

## Project Initialization

### Create Project
docker run --rm -v "$(pwd)/:/workdir" -w /workdir rust:1.92 cargo init --name archive

### Build Project
docker run --rm -v "$(pwd)/:/workdir" -w /workdir rust:1.92 cargo build

### Shell into Container
docker run --rm -it -v "$(pwd):/workdir" -w /workdir rust:1.92 bash
