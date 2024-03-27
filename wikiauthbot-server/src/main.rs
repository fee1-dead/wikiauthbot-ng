use tracing_subscriber::EnvFilter;
use wikiauthbot_db::DatabaseConnection;

#[actix_web::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    wikiauthbot_server::start(DatabaseConnection::prod().await?)
        .await?
        .await?;
    Ok(())
}
