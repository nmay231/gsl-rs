#![feature(try_blocks)]
mod client;
mod config;
mod data;

fn main() -> anyhow::Result<()> {
    client::run_client()
}
