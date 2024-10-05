

alias l := lint
alias b := build


# Runs fmt and clippy
lint:
    @cargo fmt
    @cargo clippy -- -Dwarnings

# Builds the workspace
build:
    @cargo build
