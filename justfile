
# List all just receipes
default:
    @just --list --unsorted

# Install requirement for recipes
requirement:
    cargo binstall dioxus-cli bacon cargo-nextest cargo-sort wasm-pack basic-http-server

# Format the code and sort dependencies
format:
    cargo fmt
    dx fmt
    cargo sort --workspace --grouped

_check_format:
    cargo fmt --all -- --check
    dx fmt --check
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
    bacon

# Build documentation
doc:
    cargo doc --all-features --no-deps

_example name:
    cd examples/{{name}} && wasm-pack build --release --target web
    basic-http-server examples/{{name}}

# Run Greeting example
example-greeting: (_example "greeting")

# Run Counter example
example-counter: (_example "counter")

# Run Dioxus (web component) in Dioxus example
example-dx-in-dx:
    cd examples/dx-in-dx && dx serve
