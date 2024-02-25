use wikiauthbot_db::DatabaseConnection;

#[actix_web::main]
async fn main() -> color_eyre::Result<()> {
    // TODO change to prod
    wikiauthbot_server::start(DatabaseConnection::dev().await?)
        .await?
        .await?;
    Ok(())
}
