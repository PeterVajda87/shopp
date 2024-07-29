use sea_orm::FromQueryResult;

#[derive(FromQueryResult, Debug)]
pub struct Media {
    pub path: String,
    pub media_type: String,
    pub media_role: String,
}

pub struct MediaSet {
    pub media_set: Vec<Media>
}

