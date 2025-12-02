use log::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::init_timed();

    info!("Starting skaki application server.");

    Ok(())
}
