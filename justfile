default: run

run:
    git pull --rebase
    RUST_LOG=debug cargo watch -c -x run