use models::requests::{ActionRequest, SlackAction, SlackActionRequest, SlackRequest};
use actix_web::{Form, State, HttpResponse};
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
