#![feature(proc_macro_hygiene, decl_macro)]
#![feature(custom_attribute)]

extern crate actix_web;

extern crate actix;

extern crate reqwest;

extern crate serde_json;

extern crate serde;

extern crate core;

extern crate log;
extern crate env_logger;

mod models;
mod routes;
mod service_actor;

use models::movies;
use routes::movies::BASE_URL;
use routes::movies::SERVICE_REQUEST_PATH;

use serde::Deserialize;
use actix_web::server::HttpServer;
use actix_web::{App, http, HttpRequest, middleware};
use actix::Actor;
use models::movies::MovieList;
use service_actor::ServiceActor;


fn index(_req: &HttpRequest<AppState>) -> &'static str {
    "Hello, world!"
}

fn main() {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let sys = actix::System::new("rusty-system");

    let addr = ServiceActor::new().start();

    HttpServer::new( move || create_app(addr.clone()))
        .bind("127.0.0.1:8000")
        .unwrap()
        .start();

    sys.run();
}

fn create_app(address: actix::Addr<ServiceActor>) -> App<AppState> {
    App::with_state(AppState { responder: address })
        .middleware(middleware::Logger::new("\"%r\" %s %b %Dms"))
        .resource(
            "/",
            |r| r.method(http::Method::GET).f(index),
        )
        .resource(
            "/movies",
            |r| r.method(http::Method::POST).with(routes::movies::list_movies_in_display),
        ).resource(
        "/actions",
        |r| r.method(http::Method::POST).with(routes::movies::handle_action),
    )
}

pub struct AppState {
    pub responder: actix::Addr<ServiceActor>,
}

#[derive(Deserialize)]
struct ServiceMovies {
    d: Vec<ServiceMovie>,
}

#[derive(Deserialize)]
struct ServiceMovie {
    #[serde(alias = "Name")]
    name: String,
    #[serde(alias = "Link")]
    link: String,
    #[serde(alias = "ImageUrl")]
    image_url: String,
}

fn load_data() -> MovieList {
    let json_text = make_external_request();
    let movies: ServiceMovies = serde_json::from_str(json_text.as_str()).unwrap();

    from_service_movies(movies)
}

fn make_external_request() -> String {
    let client = reqwest::Client::new();
    client
        .post(format!("{}{}", BASE_URL, SERVICE_REQUEST_PATH).as_str())
        .header(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_static("application/json"),
        )
        .send().unwrap()
        .text().unwrap()
}

fn from_service_movies(service_movies: ServiceMovies) -> MovieList {
    let movies = service_movies
        .d
        .iter()
        .map(|sm| movies::Movie::new(sm.name.clone(), sm.link.clone(), sm.image_url.clone()))
        .collect();

    movies::MovieList::new(movies)
}
