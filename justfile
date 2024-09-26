watch:
    cargo leptos watch --project www
build:
    cargo leptos build --project www --release
# lint and fix anything that can be fixed automatically
fix-all: fmt-cargo fix-cargo fix-clippy fmt-leptos

# run cargo fix
fix-cargo:
    cargo fix
    git add -u
    if (git diff --name-only --cached) != "" { git commit -m 'cargo --fix' }
fix-clippy:
    cargo clippy --fix
    git add -u
    if (git diff --name-only --cached) != "" { git commit -m 'cargo clippy --fix' }
fmt-leptos:
    leptosfmt crates/www/src/**/*.rs
    git add -u
    if (git diff --name-only --cached) != "" { git commit -m 'leptosfmt' }
fmt-cargo:
    cargo +nightly fmt
    git add -u
    if (git diff --name-only --cached) != "" { git commit -m 'cargo fmt' }