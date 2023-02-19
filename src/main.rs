mod api_websocket;
use actix_web::{middleware::Logger, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");

    env_logger::init();

    return HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(api_websocket::ws_handler)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await;
}
