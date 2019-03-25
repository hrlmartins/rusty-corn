use serde::Serialize;

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

impl Text {
    pub fn new(text: String) -> Text {
        Text {
            block_type: "mrkdwn".to_owned(),
            text,
        }
    }
}

#[derive(Serialize)]
pub struct PlainText {
    #[serde(rename = "type")]
    block_type: String,
    text: String,
}

impl PlainText {
    pub fn new(text: String) -> PlainText {
        PlainText {
            block_type: "plain_text".to_owned(),
            text,
        }
    }
}


#[derive(Serialize)]
pub struct AccessoryImage {
    #[serde(rename = "type")]
    block_type: String,
    image_url: String,
    alt_text: String,
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
    pub block_type: String,
    pub action_id: String,
    pub value: String,
    pub text: PlainText,
}

#[derive(Serialize)]
pub struct Actions {
    #[serde(rename = "type")]
    block_type: String,
    elements: Vec<Action>,
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum Block {
    Divider(Divider),
    Section(Section),
    Actions(Actions),
}

#[derive(Serialize)]
pub struct BlocksRoot {
    pub blocks: Vec<Block>,
}

impl BlocksRoot {
    pub fn new() -> BlocksRoot {
        BlocksRoot { blocks: Vec::new() }
    }

    pub fn add_divider(&mut self) -> &mut Self {
        let divider = Divider {
            block_type: "divider".to_owned(),
        };

        self.add_block(Block::Divider(divider));

        self
    }

    pub fn add_section(&mut self, text: Text, accessory: AccessoryImage) -> &mut Self {
        let section = Section {
            block_type: "section".to_owned(),
            text,
            accessory,
        };

        self.add_block(Block::Section(section));

        self
    }

    pub fn add_action(&mut self, actions: Vec<Action>) -> &mut Self {
        let actions = Actions {
            block_type: "actions".to_owned(),
            elements: actions,
        };

        self.add_block(Block::Actions(actions));

        self
    }

    fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }
}
