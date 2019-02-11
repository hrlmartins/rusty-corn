use rocket::request::LenientForm;
use reqwest::async::Client;
use reqwest::header::{HeaderValue, CONTENT_TYPE};
use futures::future::Future;
use reqwest::async::Response;

const BASE_URL: &str = "http://cinemas.nos.pt";
const SERVICE_REQUEST_PATH: &str = "/_layouts/15/Cinemas/ApplicationPages/CinemasHelperService.aspx/GetAllMoviesPlaying";

#[derive(Debug, FromForm)]
pub struct SlackRequest {
    token: String,
    user_name: String,
    response_url: String,
}

#[post("/movies", data = "<request>")]
pub fn list_movies_in_display(request: LenientForm<SlackRequest>) -> &'static str {
    println!("{:#?}", request);

    "Got it Champ!"
}

fn make_service_request(request: SlackRequest) {
    let client = Client::new();
    let stuff = client.post(format!("{}{}", BASE_URL, SERVICE_REQUEST_PATH).as_str())
        .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
        .send()
        .and_then(process_service_response)
        .map_err(| err | {

        })
        .map(|res| {

        });
}

fn process_service_response(response: Response) -> Result<(), reqwest::Error> {
    Ok(())
}

