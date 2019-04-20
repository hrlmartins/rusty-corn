use actix::{Actor, Context};
use core::borrow::Borrow;
use models::blocks::{AccessoryImage, Action, BlocksRoot, PlainText, Text};
use models::movies;
use models::requests::RequestReply;
use routes::movies::{BASE_URL, SERVICE_REQUEST_PATH};
use serde::Deserialize;

pub struct ServiceActor {
    movies: movies::MovieList,
    http_client: reqwest::Client,
}

impl ServiceActor {
    pub fn new() -> ServiceActor {
        ServiceActor {
            movies: load_data(),
            http_client: reqwest::Client::new(),
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

        let buttons = ServiceActor::build_action_buttons(page, movies.total_pages());

        root.add_action(buttons);

        root
    }

    fn build_action_buttons(page: u8, total_pages: u8) -> Vec<Action> {
        let mut actions = Vec::new();

        if page < total_pages && page > 1 {
            actions.push(ServiceActor::create_button(
                "Previous",
                (page - 1).to_string(),
            ));
            actions.push(ServiceActor::create_button("Next", (page + 1).to_string()));
        } else if page >= total_pages {
            actions.push(ServiceActor::create_button(
                "Previous",
                (page - 1).to_string(),
            ));
        } else {
            actions.push(ServiceActor::create_button("Next", (page + 1).to_string()));
        }

        actions.push(ServiceActor::create_button("Cancel", "0".to_owned()));

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
}

pub mod messages {

    pub struct QueryMovie {
        pub page: u8,
        pub response_url: String,
    }

    pub struct Cancel {
        pub response_url: String,
    }

    pub enum Message {
        QueryOp(QueryMovie),
        CancelOp(Cancel),
    }

    impl actix::Message for Message {
        type Result = ();
    }
}

impl Actor for ServiceActor {
    type Context = Context<Self>;
}

impl actix::Handler<messages::Message> for ServiceActor {
    type Result = ();

    fn handle(&mut self, msg: messages::Message, _ctx: &mut Context<Self>) -> Self::Result {
        let (url, reply) = match msg {
            messages::Message::QueryOp(query_params) => (
                query_params.response_url,
                RequestReply {
                    response_type: "ephemeral".to_owned(),
                    replace_original: query_params.page > 1,
                    delete_original: false,
                    text: "".to_owned(),
                    blocks: ServiceActor::build_response(self.movies.borrow(), query_params.page)
                        .blocks,
                },
            ),

            messages::Message::CancelOp(cancel_params) => (
                cancel_params.response_url,
                RequestReply {
                    response_type: "ephemeral".to_owned(),
                    replace_original: true,
                    delete_original: true,
                    text: "".to_owned(),
                    blocks: Vec::new(),
                },
            ),
        };

        self.http_client
            .post(url.as_str())
            .json(&reply)
            .send()
            .unwrap();

        ()
    }
}

/**************************************************************************************
*********************************
* Loading data from service
*********************************
***************************************************************************************/

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

fn load_data() -> movies::MovieList {
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
        .send()
        .unwrap()
        .text()
        .unwrap()
}

fn from_service_movies(service_movies: ServiceMovies) -> movies::MovieList {
    let movies = service_movies
        .d
        .iter()
        .map(|sm| movies::Movie::new(sm.name.clone(), sm.link.clone(), sm.image_url.clone()))
        .collect();

    movies::MovieList::new(movies)
}
