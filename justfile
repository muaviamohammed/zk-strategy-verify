# Developer tasks. Install `just`: https://github.com/casey/just
# Run `just` to list recipes.

default:
    @just --list

# Format, lint, build, and run the conformance suite (what CI runs).
check:
    cargo fmt --all --check
    cargo clippy --all-targets -- -D warnings
    cargo build --all
    cargo test --all

# Auto-format the workspace.
fmt:
    cargo fmt --all

# Run the conformance suite only.
test:
    cargo test --all

# Check an example journal against the example policy.
demo:
    cargo run -p verify-cli -- check-journal examples/policy.json examples/journal_pass.json
    cargo run -p verify-cli -- check-journal examples/policy.json examples/journal_fail.json
