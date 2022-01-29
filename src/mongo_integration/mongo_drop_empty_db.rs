use mongodb::{options::ClientOptions, Client};

#[derive(Debug)]
pub struct MongoDropEmptyDbError {
    source: Box<MongoDropEmptyDbErrorEnum>,
    line: String,
}

#[derive(Debug)]
pub enum MongoDropEmptyDbErrorEnum {
    ClientOptionsParse(ClientOptionsParseError),
    ClientWithOptions(ClientWithOptionsError),
    ListCollectionNames(ListCollectionNamesError),
    CollectionNamesListIsEmpty(CollectionNamesListIsEmptyError),
    DatabaseDrop(DatabaseDropError),
}

#[derive(Debug)]
pub struct ClientOptionsParseError {
    pub source: mongodb::error::Error,
    line: String,
}

#[derive(Debug)]
pub struct ClientWithOptionsError {
    pub source: mongodb::error::Error,
    line: String,
}

#[derive(Debug)]
pub struct ListCollectionNamesError {
    pub source: mongodb::error::Error,
    line: String,
}

#[derive(Debug)]
pub struct CollectionNamesListIsEmptyError {
    pub source: String,
    line: String,
}

#[derive(Debug)]
pub struct DatabaseDropError {
    pub source: mongodb::error::Error,
    line: String,
}

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub async fn mongo_drop_empty_db(
    mongo_url: &str,
    db_name: &str,
) -> Result<(), MongoDropEmptyDbError> {
    match ClientOptions::parse(mongo_url).await {
        Err(e) => {
            return Err(MongoDropEmptyDbError {
                source: Box::new(MongoDropEmptyDbErrorEnum::ClientOptionsParse(
                    ClientOptionsParseError {
                        source: e,
                        line: format!("{}:{}:{}", line!(), file!(), column!()),
                    },
                )),
                line: format!("{}:{}:{}", line!(), file!(), column!()),
            })
        }
        Ok(client_options) => match Client::with_options(client_options) {
            Err(e) => {
                return Err(MongoDropEmptyDbError {
                    source: Box::new(MongoDropEmptyDbErrorEnum::ClientWithOptions(
                        ClientWithOptionsError {
                            source: e,
                            line: format!("{}:{}:{}", line!(), file!(), column!()),
                        },
                    )),
                    line: format!("{}:{}:{}", line!(), file!(), column!()),
                })
            }
            Ok(client) => {
                let db = client.database(db_name);
                match db.list_collection_names(None).await {
                    Err(e) => {
                        return Err(MongoDropEmptyDbError {
                            source: Box::new(MongoDropEmptyDbErrorEnum::ListCollectionNames(
                                ListCollectionNamesError {
                                    source: e,
                                    line: format!("{}:{}:{}", line!(), file!(), column!()),
                                },
                            )),
                            line: format!("{}:{}:{}", line!(), file!(), column!()),
                        })
                    }
                    Ok(collections_names_list) => {
                        if !collections_names_list.is_empty() {
                            return Err(MongoDropEmptyDbError {
                                source: Box::new(
                                    MongoDropEmptyDbErrorEnum::CollectionNamesListIsEmpty(
                                        CollectionNamesListIsEmptyError {
                                            source: db_name.to_string(),
                                            line: format!(
                                                "{} {}",
                                                line!().to_string(),
                                                file!().to_string()
                                            ),
                                        },
                                    ),
                                ),
                                line: format!("{}:{}:{}", line!(), file!(), column!()),
                            });
                        }
                        if let Err(e) = db.drop(None).await {
                            return Err(MongoDropEmptyDbError {
                                source: Box::new(MongoDropEmptyDbErrorEnum::DatabaseDrop(
                                    DatabaseDropError {
                                        source: e,
                                        line: format!(
                                            "{} {}",
                                            line!().to_string(),
                                            file!().to_string()
                                        ),
                                    },
                                )),
                                line: format!("{}:{}:{}", line!(), file!(), column!()),
                            });
                        }
                        Ok(())
                    }
                }
            }
        },
    }
}
