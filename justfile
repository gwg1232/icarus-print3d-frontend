default: run

run:
    git pull
    RUST_LOG=debug cargo watch -c -x run