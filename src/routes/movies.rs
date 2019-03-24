use rocket::request::LenientForm;

use models::movies;
use rocket::State;
use rocket_contrib::json::Json;
use serde::Serialize;
use serde::Deserialize;

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
pub fn list_movies_in_display(request: LenientForm<SlackRequest>, movies: State<movies::MovieList>) -> Json<BlocksRoot> {
    let mut root = BlocksRoot::new();
    root.add_divider();

    for movie in movies.get_page(1) {
        let message = format!("*<{}{}|{}>*", BASE_URL, movie.url, movie.name);
        let text = Text::new(message);

        let image = AccessoryImage::new(format!("{}{}", BASE_URL, movie.image_url));

        root.add_section(text, image)
            .add_divider();
    }

    let button_text = PlainText::new("test".to_string());
    let button = Action {
        block_type: "button".to_owned(),
        text: button_text,
    };

    root.add_action(button);

    Json(root)
}

#[derive(Debug, Deserialize)]
pub struct SlackAction {
    action_id: String,
}

#[derive(Debug, Deserialize)]
pub struct SlackActionRequest {
    response_url: String,
    actions: Vec<SlackAction>,
}

#[derive(Debug, FromForm)]
pub struct ActionPayload {
    payload: String
}

#[derive(Serialize)]
pub struct ActionReply {
    response_type: String,
    replace_original: bool,
    delete_original: bool,
    text: String
}

#[post("/actions", data = "<request>")]
pub fn handle_action(request: LenientForm<ActionPayload>, movies: State<movies::MovieList>) -> &'static str {
    let request_json = request.into_inner().payload;
    let request_struct: SlackActionRequest = serde_json::from_str(request_json.as_str()).unwrap();
    let response_url = request_struct.response_url;

    let response = ActionReply {
        response_type: "ephemeral".to_owned(),
        replace_original: true,
        delete_original: false,
        text: "".to_owned(),
    };

    let client = reqwest::Client::new();
    client.post(response_url.as_str())
        .json(&response)
        .send()
        .unwrap();

    "got it champ"
}

#[derive(Serialize)]
pub struct Divider {
    #[serde(rename = "type")]
    block_type: String,
}

#[derive(Serialize)]
pub struct Text {
    #[serde(rename = "type")]
    block_type: String,
    text: String,
}

#[derive(Serialize)]
pub struct PlainText {
    #[serde(rename = "type")]
    block_type: String,
    text: String,
}

#[derive(Serialize)]
pub struct AccessoryImage {
    #[serde(rename = "type")]
    block_type: String,
    image_url: String,
    alt_text: String,
}

#[derive(Serialize)]
pub struct Section {
    #[serde(rename = "type")]
    block_type: String,
    text: Text,
    accessory: AccessoryImage,
}

#[derive(Serialize)]
pub struct Action {
    #[serde(rename = "type")]
    block_type: String,
    text: PlainText,
}

#[derive(Serialize)]
pub struct Actions {
    #[serde(rename = "type")]
    block_type: String,
    elements: Vec<Action>,
}


#[derive(Serialize)]
#[serde(untagged)]
enum Block {
    Divider(Divider),
    Section(Section),
    Actions(Actions)
}

#[derive(Serialize)]
pub struct BlocksRoot {
    blocks: Vec<Block>,
}

impl BlocksRoot {
    pub fn new() -> BlocksRoot {
        BlocksRoot { blocks: Vec::new() }
    }

    pub fn add_divider(&mut self) -> &mut Self {
        let divider = Divider {
            block_type: "divider".to_owned()
        };

        self.add_block(Block::Divider(divider));

        self
    }

    pub fn add_section(&mut self, text: Text, accessory: AccessoryImage) -> &mut Self {
        let section = Section {
            block_type: "section".to_owned(),
            text,
            accessory
        };

        self.add_block(Block::Section(section));

        self
    }

    pub fn add_action(&mut self, action: Action) -> &mut Self {
        let mut vec_actions = Vec::new();
        vec_actions.push(action);

        let actions = Actions {
            block_type: "actions".to_owned(),
            elements: vec_actions
        };

        self.add_block(Block::Actions(actions));

        self
    }

    fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }
}

impl PlainText {
    pub fn new(text: String) -> PlainText {
        PlainText {
            block_type: "plain_text".to_owned(),
            text
        }
    }
}



impl Text {
    pub fn new(text: String) -> Text {
        Text {
            block_type: "mrkdwn".to_owned(),
            text
        }
    }
}

impl AccessoryImage {
    pub fn new(url: String) -> AccessoryImage {
        AccessoryImage {
            block_type: "image".to_owned(),
            image_url: url,
            alt_text: "irrelephant".to_owned(),
        }
    }
}