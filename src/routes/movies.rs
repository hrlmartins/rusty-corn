use rocket::request::LenientForm;

use models::movies;
use rocket::State;
use rocket_contrib::json::Json;
use serde::Serialize;

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

    for movie in movies.get_page(1) {
        let message = format!("*<{}{}|{}>*", BASE_URL, movie.url, movie.name);
        let text = Text::new(message);

        let image = AccessoryImage::new(format!("{}{}", BASE_URL, movie.image_url));

        root.add_divider()
            .add_section(text, image)
            .add_divider();
    }

    println!("{:#?}", serde_json::to_string(&root).unwrap());

    let a = Json(root);

    println!("{:#?}", a);

    a
}

#[derive(Debug)]
#[derive(Serialize)]
pub struct Divider {
    #[serde(rename = "type")]
    block_type: String,
}

#[derive(Debug)]
#[derive(Serialize)]
pub struct Text {
    #[serde(rename = "type")]
    block_type: String,
    text: String,
}

#[derive(Debug)]
#[derive(Serialize)]
pub struct AccessoryImage {
    #[serde(rename = "type")]
    block_type: String,
    image_url: String,
    alt_text: String,
}

#[derive(Debug)]
#[derive(Serialize)]
pub struct Section {
    #[serde(rename = "type")]
    block_type: String,
    text: Text,
    accessory: AccessoryImage,
}

#[derive(Debug)]
#[derive(Serialize)]
#[serde(untagged)]
enum Block {
    Divider(Divider),
    Section(Section)
}

#[derive(Debug)]
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

    fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
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