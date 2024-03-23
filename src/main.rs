use anyhow::Result;

#[actix_web::main]
async fn main() -> Result<()> {
    notiflux::run().await
}
