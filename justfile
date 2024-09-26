watch:
    cargo leptos watch --project www
build:
    cargo leptos build --project www --release
# lint and fix anything that can be fixed automatically
lint:
    cargo fix
    git add -u
    git commit -m 'cargo --fix'
    cargo clippy --fix
    git add -u
    git commit -m 'cargo clippy --fix'
    leptosfmt crates/www/src/**/*.rs
    git add -u
    git commit -m 'leptosfmt'