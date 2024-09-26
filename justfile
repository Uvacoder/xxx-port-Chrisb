watch:
    cargo leptos watch --project www
build:
    cargo leptos build --project www --release
lint:
    leptosfmt crates/www/src/**/*.rs
    git add -u
    git commit -m 'leptosfmt'
    cargo fix
    git add -u
    git commit -m 'cargo --fix'
    cargo clippy --fix
    git add -u
    git commit -m 'cargo clippy --fix'