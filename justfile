server:
    cargo watch -w src-server/ -x 'run --bin gsl-server'

client:
    cargo tauri dev # The dev cargo watch command is in tauri.conf.json

cli ARGS:
    cargo run --bin gsl-cli -- {{ARGS}}
