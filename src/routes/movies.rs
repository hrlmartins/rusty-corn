use rocket::request::LenientForm;

pub const BASE_URL: &str = "http://cinemas.nos.pt";
pub const SERVICE_REQUEST_PATH: &str =
    "/_layouts/15/Cinemas/ApplicationPages/CinemasHelperService.aspx/GetAllMoviesPlaying";

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
