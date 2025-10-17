mod cli;
mod config;
mod dirs;
mod manifest;
mod downloader;
mod archive;
mod resolver;
mod shims;
mod aliases;
mod build_unix;
mod self_update_cmd;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    cli::run().await
}
