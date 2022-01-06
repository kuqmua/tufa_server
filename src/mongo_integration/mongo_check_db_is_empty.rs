use mongodb::{options::ClientOptions, Client};

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub async fn mongo_check_db_is_empty(
    mongo_url: &str,
    db_name: &str,
) -> Result<bool, mongodb::error::Error> {
    Ok(
        Client::with_options(ClientOptions::parse(mongo_url).await?)?
            .database(db_name)
            .list_collection_names(None)
            .await?
            .is_empty(),
    )
}
