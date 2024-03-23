use actix::*;
use actix_web::{middleware::Logger, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use serde::Deserialize;
use std::time::Instant;
use ulid::Ulid;

use crate::{config, message, server, session, NotifluxError};

async fn ws_route(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<server::Server>>,
) -> Result<HttpResponse, Error> {
    ws::start(
        session::WSSession {
            id: Ulid::new(),
            channels: Vec::new(),
            heartbeat: Instant::now(),
            addr: srv.get_ref().clone(),
        },
        &req,
        stream,
    )
}

#[derive(Deserialize)]
struct BroadcastPayload {
    channel: String,
    message: String,
    token: String,
}

/// POST /broadcast
/// Broadcast a message to all clients connected to a channel.
/// The request body should be a JSON object with the following fields:
/// - channel: the channel to broadcast the message to
/// - message: the message to broadcast
async fn broadcast(
    req: web::Json<BroadcastPayload>,
    srv: web::Data<Addr<server::Server>>,
) -> HttpResponse {
    srv.get_ref().do_send(message::Broadcast {
        msg: req.message.to_owned(),
        channel: req.channel.to_owned(),
        token: req.token.to_owned(),
    });

    HttpResponse::Ok().finish()
}

pub async fn run() -> Result<(), NotifluxError> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    let config = config::get_config();

    let server = server::Server::new(&config.jwt_public_key).start();

    log::info!("Starting server on {}:{}", config.host, config.port);
    let bind_tuple = (config.host.clone(), config.port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(server.clone()))
            .route("/broadcast", web::post().to(broadcast))
            .route("/ws", web::get().to(ws_route))
            .wrap(Logger::default())
    })
    .workers(config.worker_count)
    .bind(bind_tuple)?
    .run()
    .await?;

    Ok(())
}
