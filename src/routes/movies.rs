use rocket::request::LenientForm;

#[derive(Debug, FromForm)]
pub struct SlackRequest {
    token: String,
    user_name: String,
    response_url: String
}

#[post("/movies", data = "<request>")]
pub fn list_movies_in_display(request: LenientForm<SlackRequest>) ->  &'static str {
    println!("{:#?}", request);

    "Got it Champ!"
}