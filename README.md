# Installing

Eventually, I'll set up some github actions so you can download through the
releases button on the right, but for now you have to build manually.

# Building on Linux

1. Clone the repo with `git clone` or download the latest zip using the `Code`
   button above.
2. Download Rust and `cargo` using [`rustup`](https://www.rust-lang.org/tools/install).
3. Install system dependencies listed
   [here](https://tauri.app/v1/guides/getting-started/prerequisites#setting-up-linux).
4. Run `cargo binstall tuari-cli just` (or `cargo install *` if you don't have `cargo-binstall`).
5. Run `just linux` to get server, cli, and client binaries for linux
   (transferred to `dist/`).
6. To get the windows client executable using `nsis` and `xwin`, follow the instructions [here](https://tauri.app/v1/guides/building/cross-platform/#experimental-build-windows-apps-on-linux-and-macos).
    1. But change the `xwin splat` to `xwin splat --output target/.xwin`.
    2. Then run `just windows` and check for the output in `dist/`.

# Building on Windows

It might or might not work. You can try WSL if plain windows doesn't work. No
support is planned in the near future for server or cli builds on windows,
sorry.

# Development

Run steps 1-4 from [above](#building-on-linux) to set up your environment.

Launch the game server

```
just server
```

And in another terminal, test using the client and/or cli to access the server.

```
just client
just cli "args --or-flags" # Wrap in quotes if more than two pieces
```
