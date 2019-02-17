#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

extern crate chrono;

extern crate reqwest;

extern crate futures;

extern crate tokio;

extern crate serde_json;

extern crate serde;

extern crate failure;

mod models;
mod routes;

use models::movies;
use routes::movies::BASE_URL;
use routes::movies::SERVICE_REQUEST_PATH;

use chrono::Utc;
use failure::Error;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    let tmp = load_data();
    match tmp {
        Ok(res) => {
            let v: serde_json::Result<Value> = serde_json::from_str(res.as_str());
            match v {
                Ok(wat) => println!("fine and dandy"),
                Err(error) => println!("mehhh..."),
            }
        }

        Err(some) => println!("{:#?}", some),
    }

    /*    rocket::ignite()
    .mount("/", routes![index, routes::movies::list_movies_in_display,])
    .launch();*/
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
    #[serde(alias = "PremiereDate")]
    premiere_date: String,
    #[serde(alias = "ImageUrl")]
    image_url: String,
}

fn load_data() -> Result<String, Error> {
    let json_text = make_external_request()?;
    let movies: ServiceMovies = serde_json::from_str(json_text.as_str())?;

    Ok(json_text)
}

fn make_external_request() -> Result<String, Error> {
    let client = reqwest::Client::new();
    let res = client
        .post(format!("{}{}", BASE_URL, SERVICE_REQUEST_PATH).as_str())
        .header(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_static("application/json"),
        )
        .send()?
        .text()?;
    Ok(res)
}

fn from_service_movies(service_movies: ServiceMovies) -> movies::MovieList {
    let movies = service_movies
        .d
        .iter()
        .map(|sm| movies::Movie::new(sm.name.clone(), sm.link.clone(), HashMap::new(), Utc::now()))
        .collect();

    movies::MovieList::new(movies)
}
