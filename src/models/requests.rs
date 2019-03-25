use serde::Deserialize;
use serde::Serialize;
use models::blocks::Block;


#[derive(Debug, FromForm)]
pub struct SlackRequest {
    token: String,
    user_name: String,
    response_url: String,
}

#[derive(Debug, Deserialize)]
pub struct SlackAction {
    pub action_id: String,
    pub value: String,
}

#[derive(Debug, Deserialize)]
pub struct SlackActionRequest {
    pub response_url: String,
    pub actions: Vec<SlackAction>,
}

#[derive(Debug, FromForm)]
pub struct ActionRequest {
    pub payload: String,
}

#[derive(Serialize)]
pub struct ActionReply {
    pub response_type: String,
    pub replace_original: bool,
    pub delete_original: bool,
    pub text: String,
    pub blocks: Vec<Block>,
}