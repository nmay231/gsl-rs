server:
    cargo watch -w src-server/ -x 'run --bin gsl-server'

client:
    cargo tauri dev # The dev cargo watch command is in tauri.conf.json

cli ARGS:
    cargo run --bin gsl-cli -- {{ARGS}}

icons:
    ffmpeg -y -s 1024x1024 -i app-icon.svg app-icon.png
    cargo tauri icon
