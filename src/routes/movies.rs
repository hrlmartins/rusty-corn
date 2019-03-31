use models::blocks::{AccessoryImage, Action, BlocksRoot, PlainText, Text};
use models::movies;
use models::requests::{ActionReply, ActionRequest, SlackAction, SlackActionRequest, SlackRequest};
use actix_web::{Form, Json, Result, State, HttpResponse};
use core::borrow::Borrow;
use AppState;
use service_actor::messages::QueryMovie;

pub const BASE_URL: &str = "http://cinemas.nos.pt";
pub const SERVICE_REQUEST_PATH: &str =
    "/_layouts/15/Cinemas/ApplicationPages/CinemasHelperService.aspx/GetAllMoviesPlaying";

pub fn list_movies_in_display((request, state): (Form<SlackRequest>, State<AppState>)) ->  HttpResponse {
    let response_url = request.into_inner().response_url;
    let message = QueryMovie(1, response_url, false, false);

    state.responder.do_send(message);

    HttpResponse::Ok().finish()
}

pub fn handle_action(
    (request, state): (Form<ActionRequest>, State<AppState>)
) -> HttpResponse {
    let request_json = request.into_inner().payload;
    let request_struct: SlackActionRequest = serde_json::from_str(request_json.as_str()).unwrap();
    let response_url = request_struct.response_url;
    let button_action = request_struct.actions.first().unwrap();

    let response = action_response(button_action, response_url);

    state.responder.do_send(response);

    HttpResponse::Ok().finish()
}

fn action_response(action: &SlackAction, url: String) -> QueryMovie {
    let button_id = action.action_id.to_owned();
    let value_string = action.value.to_owned();
    let page: u8 = value_string.parse().unwrap();

    match button_id.as_ref() {
        "cancel" => QueryMovie(page, url, true, true),
        _ => QueryMovie(page, url, true, false)
    }
}

fn build_response(movies: &movies::MovieList, page: u8) -> BlocksRoot {
    let mut root = BlocksRoot::new();
    root.add_divider();

    for movie in movies.get_page(page) {
        let message = format!("*<{}{}|{}>*", BASE_URL, movie.url, movie.name);
        let text = Text::new(message);

        let image = AccessoryImage::new(format!("{}{}", BASE_URL, movie.image_url));

        root.add_section(text, image).add_divider();
    }

    let buttons = build_action_buttons(page, movies.total_pages());

    root.add_action(buttons);

    root
}

fn build_action_buttons(page: u8, total_pages: u8) -> Vec<Action> {
    let mut actions = Vec::new();

    if page < total_pages && page > 1 {
        actions.push(create_button("Previous", (page - 1).to_string()));
        actions.push(create_button("Next", (page + 1).to_string()));
    } else if page >= total_pages {
        actions.push(create_button("Previous", (page - 1).to_string()));
    } else {
        actions.push(create_button("Next", (page + 1).to_string()));
    }

    actions.push(create_button("Cancel", "0".to_owned()));

    actions
}

fn create_button(name: &str, value: String) -> Action {
    let button_text = PlainText::new(name.to_string());
    Action {
        block_type: "button".to_owned(),
        action_id: name.to_lowercase(),
        value: value,
        text: button_text,
    }
}
