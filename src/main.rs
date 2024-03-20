use actix::*;
use actix_web::{middleware::Logger, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use std::time::Instant;
use ulid::Ulid;

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
            channels: vec!["public".to_string()],
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

    srv.get_ref().do_send(server::Broadcast {
        msg: message.to_owned(),
        channel: channel.to_owned(),
    });

    HttpResponse::Ok().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    let server = server::Server::new().start();

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(server.clone()))
            .route("/broadcast", web::post().to(broadcast))
            .route("/ws", web::get().to(ws_route))
            .wrap(Logger::default())
    })
    .workers(4)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
