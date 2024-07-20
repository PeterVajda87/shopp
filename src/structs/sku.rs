use sea_orm::FromQueryResult;

#[derive(FromQueryResult, Debug)]
pub struct SkuData {
    pub name: String,
    pub description: Option<String>,
}
