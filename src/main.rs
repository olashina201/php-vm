mod cli;
mod commands;
mod core;
mod io;
mod platform;
mod self_update_cmd;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    cli::run().await
}
