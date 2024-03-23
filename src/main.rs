use notiflux::NotifluxError;

#[actix_web::main]
async fn main() -> Result<(), NotifluxError> {
    notiflux::run().await
}
