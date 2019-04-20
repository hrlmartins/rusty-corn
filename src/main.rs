extern crate actix_web;

extern crate actix;

extern crate reqwest;

extern crate serde_json;

extern crate serde;

extern crate core;

extern crate env_logger;
extern crate log;

mod models;
mod routes;
mod service_actor;

use actix::Actor;
use actix_web::server::HttpServer;
use actix_web::{http, middleware, App, HttpRequest};
use service_actor::ServiceActor;

fn index(_req: &HttpRequest<AppState>) -> &'static str {
    "Hello, world!"
}

fn main() {
    std::env::set_var("RUST_LOG", "actix_web=info");
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a number");

    env_logger::init();

    let sys = actix::System::new("rusty-system");

    let addr = ServiceActor::new().start();

    HttpServer::new(move || create_app(addr.clone()))
        .bind(("0.0.0.0", port))
        .unwrap()
        .start();

    sys.run();
}

fn create_app(address: actix::Addr<ServiceActor>) -> App<AppState> {
    App::with_state(AppState { responder: address })
        .middleware(middleware::Logger::new("\"%r\" %s %b %Dms"))
        .resource("/", |r| r.method(http::Method::GET).f(index))
        .resource("/movies", |r| {
            r.method(http::Method::POST)
                .with(routes::movies::list_movies_in_display)
        })
        .resource("/actions", |r| {
            r.method(http::Method::POST)
                .with(routes::movies::handle_action)
        })
}

pub struct AppState {
    pub responder: actix::Addr<ServiceActor>,
}
