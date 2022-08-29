use ankisyncd::server_run_account;
use updateaddr::update_syncaddr;
mod error;
use error::ApplicationError;
#[actix_web::main]
async fn main() -> Result<(), ApplicationError> {
    update_syncaddr().await?;
    server_run_account().await?;
    Ok(())
}