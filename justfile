default:
    just --list

# Development
server:
    cargo watch -w src-server/ -x 'run --bin gsl-server'

client:
    cargo tauri dev # The dev cargo watch command is in tauri.conf.json

cli *ARGS:
    cargo run --bin gsl-cli -- {{ARGS}}

game-wrapper *ARGS:
    cargo run --bin gsl-game-wrapper -- {{ARGS}}

# Release build
build-all: linux windows

linux: build-server build-cli build-client

build-server:
    cargo build --release --bin gsl-server
    mv target/release/gsl-server dist/

build-cli:
    cargo build --release --bin gsl-cli
    mv target/release/gsl-cli dist/

build-client: icons
    cargo tauri build
    mv target/release/bundle/appimage/gsl-client*.AppImage dist/

windows: build-client-windows

build-client-windows: icons
    cargo tauri build --target x86_64-pc-windows-msvc
    mv target/x86_64-pc-windows-msvc/release/bundle/nsis/gsl-client*.exe dist/

icons:
    ffmpeg -y -s 1024x1024 -i app-icon.svg app-icon.png
    cargo tauri icon
