// #[deny(print_stdout)]
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate actix;
extern crate actix_web;
extern crate env_logger;
extern crate native_tls;

use actix::{StreamHandler};
use actix_web::{middleware, server, App, HttpRequest, ws};

/// Message Format
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct GutsyMessage {
    time: String, // iso8601 plz
    category: Option<String>,
    message: serde_json::Value, // probably json::Value
    sequence: usize,
    client_id: Option<String>,
    tag: Option<String>
}

struct Ws;

impl actix::Actor for Ws {
    type Context = ws::WebsocketContext<Self>;
}

fn handle_message(raw_msg: &str, ctx: &mut actix_web::ws::WebsocketContext<Ws>) {
    let parsed: Result<GutsyMessage, _> = serde_json::from_str(raw_msg);
    if let Ok(msg) = parsed {
        let reserialized = serde_json::to_string(&msg).unwrap();

        info!("parsed: {:#?}", msg);
        ctx.text(&format!("{:#?}", msg));
        println!("{}", reserialized);

    } else {
        let default_message = serde_json::to_string(&GutsyMessage::default()).unwrap();
        warn!("cannot parse: {:#?}", raw_msg);
        ctx.text(&format!("cannot parse: {:#?}\n please provide something line this: {}", raw_msg, default_message));
    }

}

impl StreamHandler<ws::Message, ws::ProtocolError> for Ws {

    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
        match msg {
            ws::Message::Ping(msg) => ctx.pong(&msg),
            ws::Message::Text(text) =>  handle_message(&text, ctx),
            _ => (),
       }
    }
}

mod tls {
    use native_tls::{Pkcs12, TlsAcceptor};
    use std::fs::File;
    use std::io::{Read};
    use std::sync::Arc;

    #[allow(dead_code)]
    pub fn create_acceptor() -> Arc<TlsAcceptor> {
        let mut file = File::open("identity.pfx").unwrap();
        let mut pkcs12 = vec![];
        file.read_to_end(&mut pkcs12).unwrap();
        let pkcs12 = Pkcs12::from_der(&pkcs12, "hunter2").unwrap();

        let acceptor = TlsAcceptor::builder(pkcs12).unwrap().build().unwrap();

        Arc::new(acceptor)
    }
}

fn index(_req: HttpRequest) -> &'static str {
    "hello world!"
}

fn main() {
    if ::std::env::var("RUST_LOG").is_err() {
        ::std::env::set_var("RUST_LOG", "gutsy=debug,actix_web=info");
    }

    env_logger::init();

    let sys = actix::System::new("hello-world");
    let listen_on = "127.0.0.1:8080";
    // let listen_on_save = "127.0.0.1:8081";

    server::new(|| {
        App::new()
            // enable logger
            .middleware(middleware::Logger::default())
            .resource("/ws/", |r| r.f(|req| ws::start(req, Ws)))
            .resource("/", |r| r.f(index))
    })
    .bind(listen_on)
    // .bind_tls(listen_on_save, acceptor)
    .unwrap()
    .start();

    debug!("Started gutsy server: {:?}", listen_on);

    let _ = sys.run();
}
