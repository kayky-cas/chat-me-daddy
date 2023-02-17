use actix::{Actor, StreamHandler};
use actix_web::{get, middleware::Logger, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;

struct ChatWS;

impl Actor for ChatWS {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ChatWS {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Err(error) => println!("Error on stream handler {:?}", error),
            _ => (),
        }
    }
}

#[get("/ws/")]
async fn ws_index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(ChatWS {}, &req, stream);
    return resp;
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");

    env_logger::init();

    return HttpServer::new(|| App::new().wrap(Logger::default()).service(ws_index))
        .bind(("127.0.0.1", 3000))?
        .run()
        .await;
}
