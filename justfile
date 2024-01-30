server:
    cargo watch -w src-server/ -x 'run --bin server'

client:
    cargo tauri dev # The dev cargo watch command is in tauri.conf.json

cli:
    cargo watch -w src-cli/ -x 'run --bin cli'
