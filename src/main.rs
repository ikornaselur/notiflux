use actix::*;
use actix_web::{middleware::Logger, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use std::time::Instant;
use ulid::Ulid;

mod auth;
mod config;
mod error;
mod message;
mod server;
mod session;

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

/// POST /broadcast
/// Broadcast a message to all clients connected to a channel.
/// The request body should be a JSON object with the following fields:
/// - channel: the channel to broadcast the message to
/// - message: the message to broadcast
async fn broadcast(
    req: web::Json<serde_json::Value>,
    srv: web::Data<Addr<server::Server>>,
) -> HttpResponse {
    let channel = req["channel"].as_str().unwrap();
    let message = req["message"].as_str().unwrap();
    let token = req["token"].as_str().unwrap();

    srv.get_ref().do_send(message::Broadcast {
        msg: message.to_owned(),
        channel: channel.to_owned(),
        token: token.to_owned(),
    });

    HttpResponse::Ok().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
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
    .await
}
