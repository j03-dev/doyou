use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Id {
    #[serde(rename = "videoId")]
    pub video_id: String,
}

#[derive(Deserialize, Serialize)]
pub struct Snippet {
    pub title: String,
    pub description: String,
}

#[derive(Deserialize, Serialize)]
pub struct Item {
    pub id: Id,
    pub snippet: Snippet,
}

#[derive(Deserialize, Serialize)]
pub struct Videos {
    pub items: Vec<Item>,
}


#[derive(Deserialize, Serialize)]
pub struct Downloaded {
    pub video_id: String
}

#[derive(Deserialize, Serialize)]
pub enum Response<T, E>{
   Success(T),
   Failed(E)
}
