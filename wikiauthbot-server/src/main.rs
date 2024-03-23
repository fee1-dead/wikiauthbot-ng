use wikiauthbot_db::DatabaseConnection;

#[actix_web::main]
async fn main() -> color_eyre::Result<()> {
    wikiauthbot_server::start(DatabaseConnection::prod().await?)
        .await?
        .await?;
    Ok(())
}
