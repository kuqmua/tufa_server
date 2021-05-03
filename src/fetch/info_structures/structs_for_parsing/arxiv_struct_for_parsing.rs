#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ArxivStructForParsing {
    #[serde(rename = "item", default)]
    pub items: Vec<ArxivStructForParsingItem>,
}
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ArxivStructForParsingItem {
    pub title: String,
    pub link: String,
    pub description: String,
    pub date: String,
    pub guid: String,
    pub creator: String,
    pub identifier: String,
    pub publisher: String,
    #[serde(rename = "publicationDate", default)]
    pub publication_date: String,
}
