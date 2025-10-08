default: run

run:
    git pull --rebase --autostash
    RUST_LOG=debug cargo watch -c -x run