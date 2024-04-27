
# List all just receipes
default:
    @just --list --unsorted

# Install requirement for recipes
requirement:
    cargo binstall cargo-watch cargo-nextest cargo-sort wasm-pack basic-http-server

# Format the code and sort dependencies
format:
    cargo fmt
    cargo sort --workspace --grouped

_check_format:
    cargo fmt --all -- --check
    cargo sort --workspace --grouped --check

# Lint the rust code
lint:
    cargo clippy --workspace --all-features --all-targets

# Launch tests
test:
    cargo nextest run
    cargo test --doc

# Check the code (formatting, lint, and tests)
check: && _check_format lint test

# Run TDD mode
tdd:
    cargo watch -c -s "just check"

# Build documentation (rustdoc, book)
doc:
    cargo doc --all-features --no-deps

example-greeting:
    cd examples/greeting && wasm-pack build --release --target web
    basic-http-server examples/greeting